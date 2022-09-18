use std::env::set_current_dir;
use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;
use version::Version;

use libmangrove::crypt::mcrypt_sha256_file;
use libmangrove::pkg::{FileMetadata, Package, PackageContents, PackageFile, PackageFolder, PackageLink, save_package};
use libmangrove::platform::Architecture;

use crate::{err, ExecutableCommand};
use crate::mgvetoml::BuildConfig;
use crate::util::{info, warn};

#[derive(Parser)]
#[clap(name = "create", about = "Create and build Mangrove packages using buildtoml files", version, author)]
pub struct CreateCommand {
    #[clap(subcommand)]
    command: CreateCommandOptions
}

impl ExecutableCommand for CreateCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            CreateCommandOptions::Build(build) => build.execute()?,
            CreateCommandOptions::New(new) => new.execute()?
        };
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum CreateCommandOptions {
    #[clap(name = "build")]
    Build(CreateBuildCommand),
    #[clap(name = "new")]
    New(CreateNewCommand)
}

#[derive(Parser)]
pub struct CreateNewCommand {
    #[clap(name = "name", help = "The name of the new package to create. If not provided, will initialize the current directory instead.", default_value_t = String::from("."), value_parser)]
    pub new_name: String,

    #[clap(short = 'f', long = "force", help = "Force creation of a package if it will overwrite existing files.", action = ArgAction::SetTrue, default_value_t = false)]
    pub force: bool,
}
impl ExecutableCommand for CreateNewCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let pkgname;
        if self.new_name == "." {
            // init current dir
            pkgname = match match std::env::current_dir()?.file_name() {
                Some(s) => s,
                None => Err("Failed to get current working directory")?
            }.to_str() {
                Some(s) => s,
                None => Err("Failed to convert OsStr")?
            }.to_string();
            if Path::new(".mgve.toml").exists() {
                if self.force {
                    warn(format!(".mgve.toml already exists but -f provided, continuing anyway"));
                } else {
                    err(format!("cannot create package {} because .mgve.toml already exists", pkgname));
                    err(format!("use -f to force creation anyway"));
                    Err("Build information file already exists")?
                }
            }
        } else {
            // get new dir
            let _cwd = std::env::current_dir()?;
            let cwd = _cwd.as_path();
            let new_dir = cwd.join(Path::new(&self.new_name));
            let new_dir_str = match (&new_dir).to_str() {
                Some(s) => s,
                None => {
                    Err("Failed to convert string types")?
                }
            };
            if new_dir.exists() {
                if self.force {
                    warn(format!("directory {} already exists but -f provided, continuing anyway", new_dir_str));
                } else {
                    err(format!("cannot create directory {} as it already exists (perhaps you meant to initialize it?)", new_dir_str));
                    err(format!("use -f to force creation anyway"));
                    Err("Provided directory already exists")?
                }
            }
            create_dir_all(&new_dir)?;
            set_current_dir(new_dir)?;
            pkgname = (&self.new_name).to_owned();
        }
        info(format!("creating new package {}", pkgname.blue()));
        let buildconfig = BuildConfig {
            pkgname,
            pkgver: Version::new(0, 1, 0),
            shortdesc: "".to_string(),
            longdesc: None,
            arch: Architecture::amd64,
            url: None,
            license: None,
            groups: None,
            depends: None,
            optdepends: None,
            provides: None,
            conflicts: None,
            replaces: None
        };
        info(format!("writing .mgve.toml"));
        fs::write(".mgve.toml", toml::to_vec(&buildconfig)?)?;
        Ok(())
    }
}

#[derive(Parser)]
pub struct CreateBuildCommand {}
impl ExecutableCommand for CreateBuildCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if !Path::new(".mgve.toml").exists() {
            err(format!(".mgve.toml does not exist"));
            Err("Build information file missing")?
        }
        info(format!("loading build configuration"));
        let buildconfig_data = fs::read(".mgve.toml")?;
        let config: BuildConfig = toml::from_slice(&buildconfig_data[..])?;
        info(format!("building package {}", config.pkgname.blue()));

        let contents = match get_pkgcontents(Path::new("."), true, Path::new(".")) {
            Ok(c) => c,
            Err(e) => {
                println!("failed to enumerate package contents: {}", e);
                std::process::exit(22);
            }
        };

        let pkgcontents = contents.pkgcontents;
        let size = contents.size;

        let pkg: Package = Package {
            pkgname: config.pkgname,
            pkgver: config.pkgver,
            shortdesc: config.shortdesc,
            longdesc: config.longdesc,
            arch: config.arch,
            url: config.url,
            license: config.license,
            groups: config.groups,
            depends: config.depends,
            optdepends: config.optdepends,
            provides: config.provides,
            conflicts: config.conflicts,
            replaces: config.replaces,
            installed_size: size,
            pkgcontents
        };

        let data_dir = match fs::canonicalize(PathBuf::from("."))?.to_str() {
            Some(s) => s,
            None => {
                Err("Failed to convert string types (for final save)")?
            }
        }.to_string();

        save_package(&pkg, data_dir.clone())?;

        info(format!("wrote out package to {}", data_dir));
        Ok(())
    }
}

pub struct BuiltPackageContents {
    pub pkgcontents: PackageContents,
    pub size: usize
}

pub fn get_pkgcontents(dir: &Path, cut: bool, root: &Path) -> Result<BuiltPackageContents, Box<dyn Error>> {
    let mut result: BuiltPackageContents = BuiltPackageContents {
        pkgcontents: PackageContents {
            folders: Some(vec![]),
            files: Some(vec![]),
            links: Some(vec![])
        },
        size: 0
    };
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            let pathstr = match path.to_str() {
                Some(s) => s,
                None => {
                    Err("Failed to convert string types")?
                }
            };
            
            let rootstr = match root.to_str() {
                Some(s) => s,
                None => {
                    Err("Failed to convert string types (for root)")?
                }
            };
            
            let dirstr = match dir.to_str() {
                Some(s) => s,
                None => {
                    Err("Failed to convert string types (for dir)")?
                }
            };

            let mut pkgpath = pathstr;
            if pkgpath.starts_with(rootstr) {
                pkgpath = &pkgpath[rootstr.len()..];
            }
            if cut && pkgpath.starts_with(dirstr) {
                pkgpath = &pkgpath[dirstr.len()..];
            }

            if path.is_dir() {
                result.pkgcontents.folders.as_mut().unwrap().push(PackageFolder {
                    name: pkgpath.clone().to_owned(),
                    mtime: path.metadata()?.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize,
                    installpath: pkgpath.to_owned(),
                    meta: FileMetadata {
                        owner: path.metadata()?.uid() as usize,
                        group: path.metadata()?.gid() as usize,
                        permissions: path.metadata()?.permissions().mode() as usize
                    }
                });
                let res = get_pkgcontents(&path, false, root).expect("recursive call failed");
                result.size += res.size;
                if res.pkgcontents.links.is_some() {
                    result.pkgcontents.links.as_mut().unwrap().append(&mut res.pkgcontents.links.unwrap());
                }
                if res.pkgcontents.files.is_some() {
                    result.pkgcontents.files.as_mut().unwrap().append(&mut res.pkgcontents.files.unwrap());
                }
                if res.pkgcontents.folders.is_some() {
                    result.pkgcontents.folders.as_mut().unwrap().append(&mut res.pkgcontents.folders.unwrap());
                }
            } else if path.is_file() {
                if pathstr.contains(".mgve.toml") {
                    continue;
                }
                if pathstr.ends_with(".mgve") {
                    continue;
                }
                result.pkgcontents.files.as_mut().unwrap().push(PackageFile {
                    name: pkgpath.clone().to_owned(),
                    sha256: mcrypt_sha256_file(&pathstr.to_string())?,
                    meta: FileMetadata {
                        owner: path.metadata()?.uid() as usize,
                        group: path.metadata()?.gid() as usize,
                        permissions: path.metadata()?.permissions().mode() as usize
                    },
                    mtime: path.metadata()?.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize,
                    installpath: pkgpath.to_string()
                });
                result.size += path.metadata()?.size() as usize;
            } else if path.is_symlink() {
                let mut pkgpath_target = match path.read_link()?.to_str() {
                    Some(s) => s,
                    None => {
                        Err("Failed to convert string types (for link)")?
                    }
                }.to_string();
                if pkgpath_target.starts_with(rootstr) {
                    pkgpath_target = pkgpath_target[rootstr.len()..].parse()?;
                }
                if cut && pkgpath_target.starts_with(dirstr) {
                    pkgpath_target = pkgpath_target[dirstr.len()..].parse()?;
                }
                result.pkgcontents.links.as_mut().unwrap().push(PackageLink {
                    file: pkgpath.to_string(),
                    mtime: path.symlink_metadata()?.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as usize,
                    target: pkgpath_target
                });
            }
        }
    }
    Ok(result)
}