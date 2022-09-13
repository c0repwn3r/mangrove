//! # Structs and functions for dealing with Packages

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all, remove_dir_all, remove_file, File};
use std::io::{Cursor, Read};
use tar::{Archive, Builder};
use uuid::Uuid;
use zstd::stream::copy_encode;

use crate::{
    crypt::{mcrypt_sha256_verify_file, PrivateKey, encrypt_package},
    file::{get_cwd, set_cwd, FileOps},
    platform::{arch_str, Architecture}
};
use version::{Version, VersionReq};
use zstd::Decoder;
use crate::crypt::mcrypt_sha256_raw;

//
// Package
/// Responsible for representing all data about a package
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
    pub installed_size: usize,           // Installed Size: integer (required)
    pub pkgcontents: PackageContents,    // Package Contents: PackageContents (required)
}

// get_pkg_filename
/// Utility function to get the filename for a Package
//
pub fn get_pkg_filename(package: &Package) -> String {
    // pkgname_1.0.0-alpha.1+3423432_x86_64.mgve
    format!(
        "{}_{}_{}.mgve",
        package.pkgname,
        package.pkgver,
        arch_str(&package.arch)
    )
}

// PkgSpec
/// Represents a package specification (ie `test-package>=1`)
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PkgSpec {
    pub pkgname: String,
    pub version: VersionReq,
}

// PackageContents
/// Represents the contents of a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContents {
    pub folders: Option<Vec<PackageFolder>>,
    pub files: Option<Vec<PackageFile>>,
    pub links: Option<Vec<PackageLink>>,
}

// PackageFolder
/// Represents a folder a package creates, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFolder {
    pub name: String,
    pub mtime: usize,
    pub installpath: String,
    pub meta: FileMetadata,
}

// PackageFile
/// Represents a file inside a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFile {
    pub name: String,
    pub sha256: String,
    pub meta: FileMetadata,
    pub mtime: usize,
    pub installpath: String,
}

// PackageLink
/// Represents a symbolic link that should be created by a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageLink {
    pub file: String,
    pub mtime: usize,
    pub target: String,
}

// FileMetadata
/// Represents metadata on a file or folder
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileMetadata {
    pub owner: usize,
    pub group: usize,
    pub permissions: usize,
}

/// Utility macro for VersionReq { comparators: vec![] } because VersionReq::any() is dumb
#[macro_export]
macro_rules! version_any {
    () => {
        VersionReq {
            comparators: vec![],
        }
    };
}

// save_package_backend
//
fn save_package_backend(package: Package, data_dir: String, signing_key: Option<PrivateKey>) -> Result<String, String> {
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
    let package_contents: &PackageContents = &package.pkgcontents;
    let mut need_directories: bool = true;
    let mut directories: &Vec<PackageFolder> = &vec![];
    let directories_check = &package_contents.folders;
    match directories_check {
        Some(x) => directories = x,
        None => need_directories = false,
    }
    let mut need_files: bool = true;
    let mut files: &Vec<PackageFile> = &vec![];
    let files_check = &package_contents.files;
    match files_check {
        Some(x) => files = x,
        None => need_files = false,
    }

    // Step 3: Create directories
    if need_directories {
        // Directories need to be created
        for dir in directories {
            // Create the directory
            let path = format!("{}{}", &root, dir.name);
            let create_dir_result = create_dir_all(&path);
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
            // Validate sha256 first, reject if invalid
            let e = mcrypt_sha256_verify_file(&orig, &file.sha256);
            match e {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to verify sha256 hash of {}: {}", &orig, e)),
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
    match Package::to_file(&package, format!("{}/pkginfo", &root)) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to save pkginfo: {}", err)),
    }

    // Step 6: Create archive
    let old_dir = match get_cwd() {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    match set_cwd(&root) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    let archive_path_uncompressed = format!("{}/{}.pcm", &data_dir, get_pkg_filename(&package));
    let archive_path = format!("{}/{}", &data_dir, get_pkg_filename(&package));
    let tar_archive_bare = match File::create(&archive_path_uncompressed) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Failed to open file ({}) for writing: {}", &archive_path_uncompressed, err)),
    };
    let mut tar = Builder::new(tar_archive_bare);
    if need_files {
        for file in files {
            match tar.append_path(format!("./{}", file.name)) {
                Ok(_) => (),
                Err(err) => return Err(format!("Failed to write file to archive: {}", err)),
            }
        }
    }
    match tar.append_path("./pkginfo") {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to write file to archive: {}", err)),
    }
    match tar.finish() {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to finalize archive: {}", err)),
    }

    // Step 7: Return archive path
    match set_cwd(&old_dir) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // Step 8: Remove dir
    match remove_dir_all(&root) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to remove tmpdir: {}", err)),
    }

    // Step 9: Compress file
    let uncompressed_istream = match File::open(&archive_path_uncompressed) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Compress: Failed to open file {} for reading: {}", &archive_path_uncompressed, err)),
    };
    let compressed_ostream = match File::create(&archive_path) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Failed to open file for writing: {}", err)),
    };
    match copy_encode(uncompressed_istream, compressed_ostream, 9) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to compress package: {}", err)),
    }
    match remove_file(archive_path_uncompressed) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to remove temporary file: {}", err)),
    }

    match signing_key {
        None => return Ok(archive_path),
        Some(_) => ()
    }

    // Signing is required
    let dat: Vec<u8> = match fs::read(&archive_path) {
        Ok(dat) => dat,
        Err(err) => return Err(format!("Failed to read file: {}", err))
    };
    let enc_res = match encrypt_package(&signing_key.unwrap(), &dat) {
        Ok(vec) => vec,
        Err(err) => return Err(format!("Failed to encrypt package: {}", err))
    };
    match fs::write(&archive_path, enc_res) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to write to file: {}", err))
    }

    Ok(archive_path)
}

//
// PackageType
// Represents the type of package
//
pub enum PackageType {
    UnsignedPackage,
    SignedPackage
}

// save_package
/// Given a Package and a data_dir, use the files contained in the data_dir to build an unsigned .mgve package
//
pub fn save_package(package: Package, data_dir: String) -> Result<String, String> {
    save_package_backend(package, data_dir, None)
}

// save_package_signed
/// Given a Package and a data_dir, use the files contained in the data_dir to build a signed package
//
pub fn save_package_signed(package: Package, data_dir: String, signing_key: PrivateKey) -> Result<String, String> {
    save_package_backend(package, data_dir, Some(signing_key))
}

// load_package
/// Given an arbitrary blob, attempt to load it as an **unencrypted!** Package.
/// WILL NOT DECRYPT! You need to use is_signed_package and decrypt_package first.
//
pub fn load_package(data: &Vec<u8>) -> Result<Package, Box<dyn Error>> {
    let mut archive = Archive::new(Decoder::new(Cursor::new(data))?);
    // Pull out pkginfo
    let entries = archive.entries()?;
    let mut pkginfo: Option<Package> = None;
    let mut hashes: HashMap<String, String> = HashMap::new();
    for raw_entry in entries {
        let mut entry = raw_entry?;
        let fname = match entry.path()?.file_name() {
            Some(p) => match p.to_str() {
                Some(s) => s.to_string(),
                None => Err("Failed to convert string")?
            },
            None => Err("Failed to get entry path".to_string())?
        };
        if fname == "pkginfo" {
            // found pkginfo file!
            let mut pkinfo: Vec<u8> = vec![];
            entry.read_to_end(&mut pkinfo)?;
            pkginfo = Some(rmp_serde::from_slice(&*pkinfo)?);
        } else {
            let mut fdat: Vec<u8> = vec![];
            entry.read_to_end(&mut fdat)?;
            hashes.insert(format!("/{}", match entry.path()?.to_str() {
                Some(f) => f,
                None => {
                    Err("Failed to convert string")?
                }
            }), hex::encode(mcrypt_sha256_raw(&fdat[..])));
        }
    }
    if pkginfo.is_none() {
        Err("Failed to find pkginfo file")?
    }
    // Verify hashes
    if pkginfo.as_ref().unwrap().pkgcontents.files.is_some() {
        for file in pkginfo.as_ref().unwrap().pkgcontents.files.as_ref().unwrap() {
            if hashes.get(&*file.name).unwrap() != &file.sha256 {
                Err(format!("Fatal error: hash verification failed for {} (expected {} got {})", file.name, file.sha256, hashes.get(&*file.name).unwrap()))?
            }
        }
    }
    Ok(pkginfo.unwrap())
}

fn show_opt<T: Debug>(opt: Option<T>) -> String {
    if opt.is_some() {
        return format!("{:?}", opt.unwrap());
    }
    format!("Not provided")
}

// dump_pkg
/// Dumps the provided Package object in a human-readable format
//
pub fn dump_package(pkg: &Package) {
    println!("== Begin Package Dump ==");
    println!("| Package Name: {}", pkg.pkgname);
    println!("| Package Version: {}", pkg.pkgver);
    println!("| Description: {}", pkg.shortdesc);
    println!("| Long Description: {}", show_opt(pkg.longdesc.clone()));
    println!("| Package Architecture: {}", arch_str(&pkg.arch));
    println!("| URL: {}", show_opt(pkg.url.clone()));
    println!("| License: {}", show_opt(pkg.license.clone()));
    println!("| Groups: {}", show_opt(pkg.groups.clone()));
    println!("| Dependencies: {}", show_opt(pkg.depends.as_ref()));
    println!("| Optional Dependenies: {}", show_opt(pkg.optdepends.clone()));
    println!("| Provides: {}", show_opt(pkg.provides.as_ref()));
    println!("| Conflicts: {}", show_opt(pkg.conflicts.as_ref()));
    println!("| Replaces: {}", show_opt(pkg.replaces.as_ref()));
    println!("| Size: {}", pkg.installed_size);
    println!("| Files: {}", show_opt(pkg.pkgcontents.files.as_ref()));
    println!("| Folders: {}", show_opt(pkg.pkgcontents.folders.as_ref()));
    println!("| Links: {}", show_opt(pkg.pkgcontents.links.as_ref()));
    println!("== End Package Dump ==");
}