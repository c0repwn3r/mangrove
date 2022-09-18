//! # Provides implementation of FileOps for Package

use std::error::Error;
use std::fs;

use crate::{file::FileOps, pkg::Package};

impl FileOps for Package {
    /// Takes a Package and saves it to the specified file
    fn as_file(data: &Package, filename: String) -> Result<(), Box<dyn Error>> {
        // step 1: serialize myself
        let pkginfo_serialization_result = rmp_serde::to_vec(data);
        let pkginfo_serialized = match pkginfo_serialization_result {
            Ok(serialized) => serialized,
            Err(error) => {
                return Err(format!(
                    "Failed to serialize package data. It may have been corrupted: {:?}",
                    error
                ).into())
            }
        };
        // step 2: write to file
        let write_result = fs::write(filename, pkginfo_serialized);
        match write_result {
            Ok(_) => Ok(()),
            Err(error) => return Err(format!("Failed to write package data. The file may be inaccessible or you might not have the proper permissions: {:?}", error).into()),
        }
    }

    /// Creates a Package from the data contained in a file
    fn from_file(filename: String) -> Result<Package, Box<dyn Error>> {
        // step 1: load the file
        let read_result = fs::read(filename);
        let pkginfo_serialized: Vec<u8> = match read_result {
            Ok(file) => file,
            Err(error) => {
                return Err(format!(
                    "Failed to read pkginfo file. It may be missing or corrupted: {:?}",
                    error
                ).into())
            }
        };
        // step 2: deserialize
        let package_result = rmp_serde::from_slice(&pkginfo_serialized[..]);
        let package: Package = match package_result {
            Ok(package) => package,
            Err(error) => {
                return Err(format!(
                    "Failed to deserialize pkginfo file. It may be missing or corrupted: {:?}",
                    error
                ).into())
            }
        };
        // step 3: return
        Ok(package)
    }
}
