use cli::get_command;

mod buildtoml;
mod cli;

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
            println!("Build {:?}", _s_args.occurrences_of("dryrun"));
        }

        _ => unreachable!("Subcommand not found."),
    }
}
