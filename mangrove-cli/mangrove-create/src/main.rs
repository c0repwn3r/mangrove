use std::fs::read_to_string;
use cli::get_command;
use crate::build::build;
use crate::mgvetoml::{buildtoml_name, BuildConfig};

mod cli;
mod mgvetoml;
mod build;

fn main() {
    let _args = get_command().get_matches();

    match _args.subcommand() {
        Some(("new", _s_args)) => {
            println!(
                "Creating new package {:?}",
                _s_args.value_of("directory").unwrap()
            );
        }
        Some(("init", _s_args)) => {
            println!("Init {:?}", _s_args.value_of("directory").unwrap());
        }
        Some(("build", _s_args)) => {
            let dryrun: bool = _s_args.occurrences_of("dryrun") > 0;
            if !std::path::Path::new(".mgve.toml").exists() {
                println!("error: .mgve.toml not found");
                std::process::exit(2);
            }
            let buildconfig_str = match read_to_string(".mgve.toml") {
                Ok(s) => s,
                Err(e) => {
                    println!("error while reading build configuration: {}", e);
                    std::process::exit(5);
                }
            };
            let buildconfig: BuildConfig = match toml::from_str(&*buildconfig_str) {
                Ok(c) => c,
                Err(e) => {
                    println!("error while parsing build configuration: {}", e);
                    std::process::exit(22);
                }
            };
            println!("building {}", buildtoml_name(&buildconfig));
            build(buildconfig, dryrun);
        }

        _ => unreachable!("Subcommand not found."),
    }
}
