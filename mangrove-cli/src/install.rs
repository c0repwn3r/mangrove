use std::error::Error;
use std::fmt::format;
use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgAction;

use libmangrove::pkg::install_pkg_to;
use libmangrove::pkgdb::{pkgdb_load, pkgdb_save};

use crate::{err, ExecutableCommand};
use crate::util::info;

#[derive(Parser)]
#[clap(name = "install", about = "Install Mangrove package files", version, author)]
pub struct InstallCommand {
    #[clap(name = "repo", short = 'R', long = "--from-repo", help = "Install a package by name from your synced repositories")]
    pub repo_package: Option<String>,

    #[clap(name = "file", short = 'F', long = "--from-file", help = "Install a package from file - no dependency resolution")]
    pub file_package: Option<PathBuf>,

    #[clap(name = "target", short = 'T', long = "--target", help = "Installation target rootfs. Defaults to /, mostly for testing", default_value_t = String::from("/"))]
    pub target: String,

    #[clap(name = "local", short = 'l', long = "--local", help = "Use a local database file", action = ArgAction::SetTrue, default_value_t = false)]
    pub local: bool
}

impl ExecutableCommand for InstallCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if self.repo_package.is_none() && self.file_package.is_none() {
            err("a package is required".into());
            Ok(())
        }
        if self.repo_package.is_some() {
            err("repo_package installation is currently unfinished".into());
            Ok(())
        }
        if let Some(f_path_r) = &self.file_package {
            if let Some(f_path) = f_path_r.to_str() {
                let path = f_path.to_string();
                if !Path::new(&path).exists() || !Path::new(&path).is_file() {
                    err(format!("{} does not exist or is not a file", file));
                    Ok(())
                }
                let data = fs::read(path)?;
                info("locking database".into());
                let mut db = pkgdb_load(self.local)?;
                install_pkg_to(&data, self.target.clone(), &mut db)?;
                pkgdb_save(db, self.local)?
            } else {
                err("failed to convert path to the right string type".into())
            }
        }

        Ok(())
    }
}