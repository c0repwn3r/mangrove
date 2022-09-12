use crate::cli::get_command;

mod cli;

fn main() {
    let _args = get_command().get_matches();

}
