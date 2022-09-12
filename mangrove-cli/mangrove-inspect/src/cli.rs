use clap::{arg, Command, crate_authors, crate_description, crate_name, crate_version};
use std::path::PathBuf;

pub fn get_command() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(arg!(<package>).value_parser(clap::value_parser!(PathBuf)))
}
