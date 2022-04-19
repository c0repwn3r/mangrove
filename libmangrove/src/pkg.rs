use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all};
use uuid::Uuid;

use crate::{crypt::mcrypt_sha256_verify_file, platform::Architecture};
use version::{Version, VersionReq};

//
// Package
// Responsible for representing all data about a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)] // Allow serde to do its magic
pub struct Package {
    pub pkgname: String,                 // package name: String (required)
    pub pkgver: Version,                 // package version: semver (required)
    pub shortdesc: String,               // Short description: String (required)
    pub longdesc: Option<String>,        // Long description: String (optional)
    pub arch: Architecture,              // Architecture: Architecture (required)
    pub url: Option<String>,             // URL: String (optional)
    pub license: Option<String>,         // License: String (optional)
    pub groups: Option<Vec<String>>,     // Groups: List of String (optional)
    pub depends: Option<Vec<PkgSpec>>,   // Depends: List of PkgSpec (optional)
    pub optdepends: Option<Vec<String>>, // Optional Depends: List of String (optional)
    pub provides: Option<Vec<PkgSpec>>,  // Provides: List of PkgSpec (optional)
    pub conflicts: Option<Vec<PkgSpec>>, // Conflicts: List of PkgSpec (optional)
    pub replaces: Option<Vec<PkgSpec>>,  // Replaces: List of PkgSpec (optional)
    pub installed_size: u64,             // Installed Size: integer (required)
    pub pkgcontents: PackageContents,    // Package Contents: PackageContents (required)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PkgSpec {
    pub pkgname: String,
    pub version: VersionReq,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContents {
    pub folders: Option<Vec<PackageFolder>>,
    pub files: Option<Vec<PackageFile>>,
    pub links: Option<Vec<PackageLink>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFolder {
    pub name: String,
    pub mtime: i32,
    pub installpath: String,
    pub meta: FileMetadata,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFile {
    pub name: String,
    pub sha256: String,
    pub meta: FileMetadata,
    pub mtime: i32,
    pub installpath: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageLink {
    pub file: String,
    pub mtime: i32,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileMetadata {
    pub owner: i32,
    pub group: i32,
    pub permissions: i32,
}

// Utility macro for VersionReq { comparators: vec![] } because VersionReq::any() is dumb
#[macro_export]
macro_rules! version_any {
    () => {
        VersionReq {
            comparators: vec![],
        }
    };
}

pub fn save_package(package: Package, data_dir: String) -> Result<String, String> {
    // Step 1: Create temporary dir
    let random_identifier: String = Uuid::new_v4().to_string(); // Get a random uuidv4
    let root_prefix: String = "/tmp/mangrove_build_".to_string(); // prefix

    let root: String = format!("{}{}", root_prefix, random_identifier); // generate the root

    let create_root_result = fs::create_dir_all(&root);
    match create_root_result {
        Ok(_) => (),
        Err(error) => {
            return Err(format!(
                "[!] Failed to create fakeroot directory: {}",
                error
            ))
        }
    }

    // Step 2: Check package contents
    let package_contents: PackageContents = package.pkgcontents;
    let mut need_directories: bool = true;
    let mut directories: Vec<PackageFolder> = vec![];
    let directories_check = package_contents.folders;
    match directories_check {
        Some(x) => directories = x,
        None => need_directories = false,
    }
    let mut need_files: bool = true;
    let mut files: Vec<PackageFile> = vec![];
    let files_check = package_contents.files;
    match files_check {
        Some(x) => files = x,
        None => need_files = false,
    }

    // Step 3: Create directories
    if need_directories {
        // Directories need to be created
        for dir in directories {
            let path = format!("{}{}", &root, dir.name);
            let create_dir_result = create_dir_all(path);
            match create_dir_result {
                Ok(_) => (),
                Err(err) => return Err(format!("[!] Unable to create directory: {}", err)),
            }
        }
    }
    // Step 4: Copy files
    if need_files {
        // Files need to be copied
        for file in files {
            let orig = format!("{}{}", data_dir, &file.name);
            println!("{}", orig);
            // Validate sha256 first, reject if invalid
            let e = mcrypt_sha256_verify_file(&orig, &file.sha256);
            match e {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Failed to verify sha256 hash of {}: {}",
                        &file.name, e
                    ))
                }
            }
            let target = format!("{}{}", &root, &file.name);
            let copy_result = fs::copy(&orig, target);
            match copy_result {
                Ok(_) => (),
                Err(err) => return Err(format!("Unable to copy file: {}", err)),
            }
        }
    }
    // Step 5: Write package metadata
    // Step 6: Fakeroot
    // Step 7: Set directory metadata
    // Step 8: Set file metadata
    // Step 9: Set link metadata
    // Step 10: Create archive
    // Step 11: Exit fakeroot
    // Step 12: Return archive path

    Ok("a".to_string())
}
