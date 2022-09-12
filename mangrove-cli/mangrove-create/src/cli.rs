use clap::{arg, Command, crate_authors, crate_description, crate_name, crate_version};

pub fn get_command() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
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
