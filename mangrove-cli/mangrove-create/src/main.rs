use std::process::exit;

fn print_help() {
    println!("mangrove-create - Create Mangrove packages");
    println!();
    println!("Usage:");
    println!("  mangrove-create new <name>");
    println!("                  init");
    println!("                  build [--dryrun]");
    println!();
    println!("Options:");
    println!("  new <name> | Create a new mangrove package in the specified directory.");
    println!("  init       | Initialize a mangrove package in the current directory.");
    println!("  build      | Build a .mgve package. Use --dryrun to run build steps but don't output a package.");
    exit(0);
}

fn main() {
    print_help();
}
