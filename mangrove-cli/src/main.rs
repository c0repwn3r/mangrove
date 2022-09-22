// Linter configuration
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

// Outright bad practice
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

// Just shut up
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_lines)]

use std::error::Error;

use clap::{AppSettings, ArgAction, Parser, Subcommand};

use libmangrove::{detailed_version, version};

use crate::cli::ExecutableCommand;
use crate::create::CreateCommand;
use crate::inspect::InspectCommand;
use crate::install::InstallCommand;
use crate::trust::TrustCommand;
use crate::util::err;

mod inspect;
mod cli;
mod create;
mod util;
mod mgvetoml;
mod trust;
mod install;

#[derive(Parser)]
#[clap(name = "mgve", about = "Mangrove CLI interface", version, author)]
#[clap(propagate_version = true)]
#[clap(subcommand_required = false)]
#[clap(arg_required_else_help = true)]
#[clap(global_setting(AppSettings::NoAutoVersion))]
pub struct MangroveCLI {
    #[clap(short = 'v', long = "version", action = ArgAction::SetTrue, default_value_t = false, help = "Show the Mangrove CLI and libmangrove versions.")]
    show_version: bool,

    #[clap(short = 'V', long = "detailed_version", action = ArgAction::SetTrue, default_value_t = false, help = "Show detailed information about the version of libmangrove this binary is linked against")]
    show_lmg_version: bool,

    #[clap(subcommand)]
    command: Option<MangroveCLIOptions>
}

#[derive(Subcommand)]
pub enum MangroveCLIOptions {
    #[clap(name = "inspect")]
    Inspect(InspectCommand),
    #[clap(name = "create")]
    Create(CreateCommand),
    #[clap(name = "trust")]
    Trust(TrustCommand),
    #[clap(name = "install")]
    Install(InstallCommand)
}

impl ExecutableCommand for MangroveCLI {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        if self.show_version {
            println!("mangrove-cli {}, {}", env!("CARGO_PKG_VERSION"), version());
            return Ok(());
        }
        if self.show_lmg_version {
            println!("mangrove-cli {}\n{}", env!("CARGO_PKG_VERSION"), detailed_version());
            return Ok(())
        }
        if self.command.is_none() {
            err(format!("a subcommand is required"));
            return Ok(());
        }
        match &self.command.as_ref().unwrap() {
            MangroveCLIOptions::Inspect(inspect) => inspect.execute()?,
            MangroveCLIOptions::Create(create) => create.execute()?,
            MangroveCLIOptions::Trust(trust) => trust.execute()?,
            MangroveCLIOptions::Install(install) => install.execute()?
        };
        Ok(())
    }
}

fn main() {
    let args: MangroveCLI = MangroveCLI::parse();
    match args.execute() {
        Ok(_) => (),
        Err(e) => {
            err(format!("error while executing subcommand: {}", e))
        }
    }
}
