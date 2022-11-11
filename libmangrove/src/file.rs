//! # Traits, structs, and functions for interfacing with the filesystem

use std::env::{current_dir, set_current_dir};
use std::error::Error;

// FileOps
/// Utility trait to add the to_file and from_file methods to structs
//
#[allow(clippy::module_name_repetitions)]
pub trait FileOps {
    /// Attempt to write this `Self` to the specified file.
    /// # Errors
    /// This function will error in the event that the object could not be written to the specified file
    fn as_file(data: &Self, filename: String) -> Result<(), Box<dyn Error>>;
    /// Attempt to read the specified file, creating an object of type `Self`
    /// # Errors
    /// This function will error in the event that the object could not be read from the specified file
    fn from_file(filename: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

// get_cwd
/// Utility function to get the current working directory
/// # Errors
/// This function will error if there was an error getting the cwd or the string types could not be converted.
pub fn get_cwd() -> Result<String, Box<dyn Error>> {
    let old_dir = match current_dir() {
        Ok(dir) => match dir.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err("Failed to convert string types".into()),
        },
        Err(err) => return Err(format!("Failed to get working directory: {}", err).into()),
    };
    Ok(old_dir)
}

// set_cwd
/// Utility function to set the current working directory
/// # Errors
/// This function will error if the cwd could not be set
pub fn set_cwd(path: &str) -> Result<(), Box<dyn Error>> {
    match set_current_dir(path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("Unable to set directory: {}", err).into()),
    }
}
