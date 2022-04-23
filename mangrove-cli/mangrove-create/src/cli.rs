use clap::{arg, Command};

pub fn get_command() -> Command<'static> {
    Command::new("mangrove-create")
        .version("0.1.0")
        .author("c0repwn3r")
        .about("Create packages for the Mangrove package manager")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new mangrove package in the specified directory.")
                .arg(arg!([directory]).required(true)),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a mangrove package in an existing directory. Defaults to cwd.")
                .arg(arg!([directory]).default_value(".")),
        )
        .subcommand(
            Command::new("build")
                .about("Build a .mgve package. Use --dryrun to run build steps but don't output a package.")
                .arg(arg!(--dryrun))
        )
}
