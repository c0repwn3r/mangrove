use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::{ArgAction, Parser};
use url::Url;

use libmangrove::crypt::{encrypt_package, is_signed_package, PrivateKey};
use libmangrove::pkg::{get_pkg_filename, load_package, Package};
use libmangrove::platform::Architecture;
use libmangrove::repo::Repository;
use libmangrove::trustcache::{trustcache_load, trustcache_save};

use crate::{err, ExecutableCommand, warn};
use crate::util::info;

#[derive(Parser)]
#[clap(about = "Generate pool files for a package repository")]
pub struct RepogenCommand {
    #[clap(name = "input", value_parser, help = "The folder to get unsigned package files from")]
    input: PathBuf,
    #[clap(name = "output", value_parser, help = "The folder to output the repository data to")]
    output: PathBuf,
    #[clap(name = "baseurl", value_parser, help = "The world-accessible baseurl for this repository")]
    baseurl: Url,
    #[clap(name = "dont_export_index", long = "dont_export_index", value_parser, action = ArgAction::SetTrue, default_value_t = false, help = "Disable exporting a index.json file for the repository browser")]
    disable_export_index: bool,
    #[clap(name = "local", short = 'l', long = "local", value_parser, help = "Use a local trustcache", action = ArgAction::SetTrue, default_value_t = false)]
    local: bool,
    #[clap(name = "key", short = 'k', long = "key", value_parser, help  = "Which private key to use. This may also be a prefix of a key, to use a key from a trustcache. If not provided, will use the first key found in the trustcache.")]
    key: Option<String>
}

impl ExecutableCommand for RepogenCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if !&self.input.is_dir() {
            err("input dir must be a folder".into());
            return Ok(());
        }
        if self.output.is_file() {
            err("output dir must be a folder".into());
            return Ok(());
        }

        // keyfinding logic
        let mut key: Option<PrivateKey> = None;
        info("loading trustcache".into());
        let trustcache = trustcache_load(self.local)?;
        if let Some(ki) = &self.key {
            if let Ok(key_i) = PrivateKey::from_anonymous(ki) {
                if trustcache.keydb.deny_privkeys.contains(ki) {
                    err("this private key has been explicitly blacklisted".into());
                    trustcache_save(trustcache, self.local)?;
                    return Ok(());
                } else {
                    info("loaded private key from cli".into());
                    trustcache_save(trustcache, self.local)?;
                    key = Some(key_i);
                }
            } else {
                if let Some(key_ik) = trustcache.keydb.known_privkeys.iter().find(|k| k.starts_with(ki)) {
                    let key_ikc = key_ik.to_owned();
                    trustcache_save(trustcache, self.local)?;
                    key = Some(PrivateKey::from_anonymous(&key_ikc)?);
                }
            }
        } else {
            if let Some(key_i) = trustcache.keydb.known_privkeys.get(0) {
                let key_ic = key_i.to_owned();
                trustcache_save(trustcache, self.local)?;
                key = Some(PrivateKey::from_anonymous(&key_ic)?);
            } else {
                err("no keys available to sign".into());
            }
        }

        if let Some(kd) = key {
            let files = fs::read_dir(&self.input)?;
            let mut files_to_include: Vec<PathBuf> = vec![];
            info("enumerating repository contents".into());
            for file_r in files {
                let file = file_r?;
                if let Some(ext) = file.path().extension() {
                    if ext != "mgve" { continue; }

                    files_to_include.push(file.path());
                } else {
                    warn(format!("unable to get extension, skipping"));
                    continue;
                }
            }

            // create output directory: make pool
            let pool = self.output.join("pool/");
            create_dir_all(pool.clone())?;

            // process files: sign and write to pool
            info(format!("processing {} packages", files_to_include.len()));

            let mut supported_architectures: Vec<Architecture> = vec![];
            let mut packages: HashMap<Architecture, Vec<Package>> = HashMap::new();

            for f in files_to_include {
                let data = fs::read(f.clone())?;
                if is_signed_package(&data) {
                    warn(format!("skipping already-signed package {}", f.display()));
                    continue;
                }

                let pkg = load_package(&data)?;

                if !supported_architectures.contains(&pkg.arch) { supported_architectures.push((&pkg).arch.clone()) }
                if packages.get(&pkg.arch).is_none() { packages.insert(pkg.arch.clone(), vec![] ); }

                if let Some(pkgarr) = packages.get(&pkg.arch) {
                    let mut pkar = pkgarr.clone();
                    pkar.push(pkg.clone());
                    packages.insert((&pkg.arch).clone(), pkar);
                } else {
                    return Err("md missing vinbinfo".into());
                }

                // sign the package
                info(format!("signing {}", f.display()));

                let enc_data = encrypt_package(&kd, &data)?;
                let outfile = (&pool).clone().join(get_pkg_filename(&pkg));
                fs::write(outfile, enc_data)?;
            }

            let repo = Repository {
                baseurl: (&self.baseurl).clone(),
                signing_key: kd.derive(),
                avaliable_architectures: supported_architectures,
                packages
            };

            info("writing repodata".into());
            fs::write(self.output.join("repodata"), rmp_serde::to_vec(&repo)?)?;

            if self.disable_export_index {
                info("skipping index.json".into());
            } else {
                info("exporting index.json".into());
                fs::write(self.output.join("index.json"), serde_json::to_string(&repo)?)?;
            }
        } else {
            err("no keys avaliable to sign".into());
        }

        Ok(())
    }
}