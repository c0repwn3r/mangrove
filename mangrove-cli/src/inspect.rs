use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::{ArgAction, Parser};

use libmangrove::crypt::{debug_dump_package, decrypt_package, find_key, is_signed_package, PublicKey};
use libmangrove::pkg::{dump_package, load_package};
use libmangrove::trustcache::{trustcache_load, trustcache_save};

use crate::cli::ExecutableCommand;

#[derive(Parser)]
#[clap(name = "inspect", about = "Get information about a given package, attempting to decrypt it if signed.", version, author)]
pub struct InspectCommand {
    #[clap(name = "file", value_parser)]
    pub file: PathBuf,

    #[clap(name = "pubkey", short = 'k', long = "pubkey", value_parser, help = "Set the publickey to be used for package decryption if a package is encrypted")]
    pub key: Option<String>,

    #[clap(name = "local", short = 'l', long = "local", action = ArgAction::SetTrue, default_value_t = false, help = "Use a local trustcache instead of the default systemwide one")]
    pub local_cache: bool
}

impl ExecutableCommand for InspectCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let args = self;
        let mut data: Vec<u8> = match fs::read(args.file.as_path()) {
            Ok(d) => d,
            Err(e) => {
                println!("Error while opening package: {}", e);
                std::process::exit(1);
            }
        };
        let mut key: Option<PublicKey> = None;
        if args.key.is_some() {
            key = match PublicKey::from_anonymous(&args.key.as_ref().unwrap().to_owned()) {
                Ok(k) => Some(k),
                Err(e) => {
                    println!("warn: failed to load __anonymous__ public key ({}), skipping key checks", e);
                    None
                }
            };
        }
        if is_signed_package(&data) {
            println!("Package Type: Signed");
            println!("Signed Package Format Dump");
            println!("{}", debug_dump_package(&data, key.as_ref()));
            let mut foundkey = key;
            if foundkey.is_none() {
                println!("no key provided, trying trustcache");
                let trustcache = match trustcache_load(args.local_cache) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("failed to load trustcache ({})", e);
                        println!("err: no valid key provided, cannot decrypt package");
                        std::process::exit(1);
                    }
                };
                let realkey = find_key(&data, &trustcache);
                match trustcache_save(trustcache, args.local_cache) {
                    Ok(..) => (),
                    Err(e) => {
                        println!("failed to save trustcache: ({})", e);
                        println!("the lockfile is most likely damaged and the trustcache most likely corrupted");
                        std::process::exit(1);
                    }
                }
                if realkey.is_none() {
                    println!("err: no keys avaliable in trustcache");
                    println!("err: decryption key missing, cannot proceed");
                    std::process::exit(1);
                }
                foundkey = realkey;
            }
            println!("Decrypting package...");
            match decrypt_package(foundkey.as_ref().unwrap(), &data[..]) {
                Ok(d) => {
                    data = d;
                },
                Err(e) => {
                    println!("err: failed to decrypt package ({})", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("Package Type: Unsigned");
        }
        dump_package(&match load_package(&data) {
            Ok(p) => p,
            Err(e) => {
                println!("err: failed to load package: {}", e);
                std::process::exit(1);
            }
        });
        Ok(())
    }
}