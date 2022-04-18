use crate::{file::FileOps, pkg::Package};
use std::fs;

impl FileOps for Package {
    fn to_file(data: Package, filename: String) {
        // step 1: serialize myself
        let pkginfo_serialization_result = rmp_serde::to_vec(&data);
        let pkginfo_serialized = match pkginfo_serialization_result {
            Ok(serialized) => serialized,
            Err(error) => panic!(
                "[!] Failed to serialize package data. It may have been corrupted: {:?}",
                error
            ),
        };
        // step 2: write to file
        let write_result = fs::write(filename, pkginfo_serialized);
        match write_result {
            Ok(_) => (),
            Err(error) => panic!("[!] Failed to write package data. The file may be inaccessible or you might not have the proper permissions: {:?}", error),
        }
    }

    fn from_file(filename: String) -> Package {
        // step 1: load the file
        let read_result = fs::read(filename);
        let pkginfo_serialized: Vec<u8> = match read_result {
            Ok(file) => file,
            Err(error) => panic!(
                "[!] Failed to read pkginfo file. It may be missing or corrupted: {:?}",
                error
            ),
        };
        // step 2: deserialize
        let package_result = rmp_serde::from_slice(&pkginfo_serialized[..]);
        let package: Package = match package_result {
            Ok(package) => package,
            Err(error) => panic!(
                "[!] Failed to deserialize pkginfo file. It may be missing or corrupted: {:?}",
                error
            ),
        };
        // step 3: return
        package
    }
}
