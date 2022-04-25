//! # Traits, structs, and functions for interfacing with the filesystem

use std::env::{current_dir, set_current_dir};

// FileOps
/// Utility trait to add the to_file and from_file methods to structs
//
pub trait FileOps {
    fn to_file(data: &Self, filename: String) -> Result<(), String>;
    fn from_file(filename: String) -> Result<Self, String>
    where
        Self: Sized;
}

// get_cwd
/// Utility function to get the current working directory
//
pub fn get_cwd() -> Result<String, String> {
    let old_dir = match current_dir() {
        Ok(dir) => match dir.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err("Failed to convert string types".to_string()),
        },
        Err(err) => return Err(format!("Failed to get working directory: {}", err)),
    };
    Ok(old_dir)
}

// set_cwd
/// Utility function to set the current working directory
//
pub fn set_cwd(path: &String) -> Result<(), String> {
    match set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("Unable to set directory: {}", err)),
    }
}
