use std::fs;
use std::path::PathBuf;
use libmangrove::crypt::{debug_dump_package, decrypt_package, is_signed_package, PublicKey};
use libmangrove::pkg::load_package;
use crate::cli::get_command;

mod cli;

fn main() {
    let _args = get_command().get_matches();

    let path: &PathBuf = _args.get_one("package").unwrap();
    let data: Vec<u8> = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error while opening package: {}", e);
            std::process::exit(1);
        }
    };
    let mut key: Option<PublicKey> = None;
    if _args.get_one::<String>("pubkey").is_some() {
        key = match PublicKey::from_anonymous(_args.get_one::<String>("pubkey").unwrap().to_owned()) {
            Ok(k) => Some(k),
            Err(e) => {
                println!("warn: failed to load __anonymous__ public key ({}), skipping key checks", e);
                None
            }
        };
    }
    let mut package_data = data.clone();
    if is_signed_package(data.clone()) {
        println!("Package Type: Signed");
        println!("Signed Package Format Dump");
        println!("{}", debug_dump_package(data.clone(), key.as_ref()));
        if key.is_none() {
            // TODO: trustcache checking
            println!("err: no valid key provided, cannot decrypt package");
            std::process::exit(1);
        }
        println!("Decrypting package...");
        match decrypt_package(key.as_ref().unwrap(), data) {
            Ok(d) => {
                package_data = d;
            },
            Err(e) => {
                println!("err: failed to decrypt package ({})", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("Package Type: Unsigned");
    }
    println!("{:?}", load_package(&package_data));
}
