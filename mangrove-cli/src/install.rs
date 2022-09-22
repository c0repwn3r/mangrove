use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{Read, stdin, stdout, Write};
use std::path::{Path};
use clap::{Parser, ArgAction};
use human_bytes::human_bytes;
use libmangrove::crypt::{decrypt_package, find_key, is_signed_package};
use libmangrove::pkg::{get_pkg_filename, install_pkg_to, load_package, Package};
use libmangrove::pkgdb::{pkgdb_load, pkgdb_save};
use libmangrove::trustcache::{trustcache_load, trustcache_save};
use crate::{err, ExecutableCommand};
use crate::util::{info, warn};

#[derive(Parser)]
#[clap(name = "install", about = "Install Mangrove package files", version, author)]
pub struct InstallCommand {
    #[clap(name = "package", help = "Specify a repository package or file to install")]
    pub packages: Vec<String>,

    #[clap(name = "target", short = 'T', long = "--target", help = "Installation target rootfs. Defaults to /, mostly for testing", default_value_t = String::from("/"))]
    pub target: String,

    #[clap(name = "local", short = 'l', long = "--local", help = "Use a local database file", action = ArgAction::SetTrue, default_value_t = false)]
    pub local: bool
}

impl ExecutableCommand for InstallCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if self.packages.len() == 0 {
            err("no targets specified".into());
            return Ok(());
        }

        info("loading packages...".into());

        let mut files_to_install: Vec<String> = vec![];

        info("resolving packages...".into());
        for package in &self.packages {
            let file = Path::new(package);
            if file.exists() && file.is_file() {
                files_to_install.push(package.clone());
            } else {
                warn(format!("installing from repositories is currently not implemented, skipping {}", package));
                continue;
            }
        }

        info("loading packages...".into());

        let mut packages_to_install: HashMap<String, Package> = HashMap::new();

        let mut needs_trustcache = false;
        let mut packages_need_decryption: Vec<String> = vec![];

        for file in files_to_install {
            let data = match fs::read(file.clone()) {
                Ok(d) => d,
                Err(e) => {
                    warn(format!("an error occured reading {} ({}), it will be skipped", file, e).into());
                    continue;
                }
            };
            if is_signed_package(data.clone()) {
                needs_trustcache = true;
                packages_need_decryption.push(file);
            } else {
                let pkg = match load_package(&data) {
                    Ok(p) => p,
                    Err(e) => {
                        err(format!("error loading package: {}, it will be skipped", e).into());
                        continue;
                    }
                };
                packages_to_install.insert(file, pkg);
            }
        }

        if needs_trustcache {
            let trustcache = trustcache_load(self.local)?;
            info("decrypting packages".into());
            for file in packages_need_decryption {
                let data = match fs::read(file.clone()) {
                    Ok(d) => d,
                    Err(e) => {
                        warn(format!("an error occured decrypting {} ({}), it will be skipped", file, e).into());
                        continue;
                    }
                };
                if !is_signed_package(data.clone()) { continue; }

                let key = find_key(&data[..], &trustcache);

                if let Some(dec_k) = key {
                    let data_dec = match decrypt_package(&dec_k, &data[..]) {
                        Ok(d) => d,
                        Err(e) => {
                            err(format!("failed to decrypt {} ({}), it will be skipped", &file, e).into());
                            continue;
                        }
                    };
                    let pkg = match load_package(&data_dec) {
                        Ok(d) => d,
                        Err(e) => {
                            err(format!("failed to load decrypted {} ({}), it will be skipped", &file, e).into());
                            continue;
                        }
                    };
                    match fs::write(format!("DECRYPTED_TMP_PACKAGE_MM_pkg{}.mgve", get_pkg_filename(&pkg)), data_dec) {
                        Ok(_) => (),
                        Err(e) => {
                            err(format!("failed to write to temporary file {} ({}), {} will be skipped", format!("DECRYPTED_TMP_PACKAGE_MM_pkg{}.mgve", get_pkg_filename(&pkg)), e, &file).into());
                            continue;
                        }
                    };
                    packages_to_install.insert(format!("DECRYPTED_TMP_PACKAGE_MM_pkg{}.mgve", get_pkg_filename(&pkg)), pkg);
                } else {
                    warn(format!("no key avaliable to decrypt {}, it will be skipped", &file).into());
                }
            }
            trustcache_save(trustcache, self.local)?;
        }

        println!("Caluclating conflicts...");
        let pkgdb = pkgdb_load(self.local)?;

        for (_, pkginfo) in &packages_to_install {
            let conflicting = pkgdb.db.installed_packages.iter().find(|pkg| {
                if let Some(conflicts) = &pkginfo.conflicts {
                    if conflicts.iter().any(|conflict| conflict.pkgname == pkg.pkgname && conflict.version.matches(&pkg.pkgver)) {
                        return true;
                    }
                }
                if let Some(conflicts) = &pkg.conflicts {
                    return conflicts.iter().find(|conflict| conflict.pkgname == pkginfo.pkgname && conflict.version.matches(&pkginfo.pkgver)).is_some()
                }
                false
            });
            if let Some(conflict) = conflicting {
                err(format!("{} conflicts with {}, please resolve conflicts first", pkginfo.pkgname, conflict.pkgname));
                return Ok(())
            }
        }

        println!("Resolving dependencies..");
        // TODO: real dependency resolution here, add missing packages
        for (_, pkginfo) in &packages_to_install {
            if let Some(dependencies) = &pkginfo.depends {
                for dependency in dependencies {
                    if !&pkgdb.db.installed_packages.iter().any(|x| x.pkgname == dependency.pkgname && dependency.version.matches(&x.pkgver)) {
                        err(format!("{} has required dependency {} that is not installed", pkginfo.pkgname, dependency.pkgname).into());
                        return Ok(())
                    }
                }
            }
            if pkgdb.db.installed_packages.iter().any(|x| x.pkgname == pkginfo.pkgname && pkginfo.pkgver >= x.pkgver) {
                warn(format!("{} is up to date - reinstalling", pkginfo.pkgname));
            }
        }
        println!();

        pkgdb_save(pkgdb, self.local)?;

        println!("To install:");
        println!("Number\tName\tVersion\tSize");
        let mut total_size = 0;
        let mut i = 1;
        for (_, package) in &packages_to_install {
            println!("{}\t{}\t{}\t{}", i, package.pkgname, package.pkgver, human_bytes(package.installed_size as f64));
            total_size += package.installed_size;
            i+=1;
        }
        println!("Total installed size: {}\n", human_bytes(total_size as f64));

        print!("Continue with installation: [Y/n] ");
        let _=stdout().flush();

        let mut c: [u8; 1] = [0];
        stdin().read_exact(&mut c)?;
        let c = c[0] as char;
        if c == 'n' || c == 'N' {
            println!("Aborted by user");
            return Ok(());
        }
        println!("Installing packages...");

        let mut pkgdb = pkgdb_load(self.local)?;
        for (file, pkg) in packages_to_install {
            let data = match fs::read(file.clone()) {
                Ok(d) => d,
                Err(e) => {
                    err(format!("failed to read package: {}", e));
                    pkgdb_save(pkgdb, self.local)?;
                    return Ok(())
                }
            };
            println!("Installing {}-{}...", pkg.pkgname, pkg.pkgver);
            match install_pkg_to(&data, (&self.target).clone(), &mut pkgdb) {
                Ok(_) => (),
                Err(e) => {
                    err(format!("error installing package: {}", e));
                    pkgdb_save(pkgdb, self.local)?;
                    return Ok(())
                }
            }
            if file.starts_with("DECRYPTED_TMP_PACKAGE_MM_pkg") {
                println!("Deleting temporary file {}...", file);
                match fs::remove_file(file) {
                    Ok(_) => (),
                    Err(e) => {
                        err(format!("error removing temporary dir: {}", e));
                        pkgdb_save(pkgdb, self.local)?;
                        return Ok(())
                    }
                }
            }
        }
        pkgdb_save(pkgdb, self.local)?;

        Ok(())
    }
}