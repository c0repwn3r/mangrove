use std::error::Error;
use std::fs;
use std::path::PathBuf;

use colored::Colorize;

use libmangrove::crypt::{encrypt_package, PrivateKey};

// info, warn, err

pub fn info(text: String) {
    println!("{} {}", "info:".bold(), text.bold());
}

pub fn warn(text: String) {
    println!("{} {}", "warn:".bold().yellow(), text.bold());
}

pub fn err(text: String) {
    println!("{} {}", "err:".bold().red(), text.bold());
}

pub fn sign_pkg(file: &PathBuf, out: &PathBuf, key: &PrivateKey) -> Result<(), Box<dyn Error>> {
    let data = fs::read(file)?;
    let out_data = encrypt_package(key, &data)?;

    fs::write(out, out_data)?;

    Ok(())
}