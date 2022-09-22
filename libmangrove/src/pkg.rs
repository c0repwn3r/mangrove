//! # Structs and functions for dealing with Packages

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::{self, create_dir_all, File, remove_dir_all, remove_file};
use std::io::{Cursor, Read};
use std::os::unix::fs::symlink;

use log::debug;
use serde::{Deserialize, Serialize};
use tar::{Archive, Builder};
use uuid::Uuid;
use version::{Version, VersionReq};
use zstd::Decoder;
use zstd::stream::copy_encode;

use crate::{
    crypt::{encrypt_package, mcrypt_sha256_verify_file, PrivateKey},
    file::{FileOps, get_cwd, set_cwd},
    platform::{arch_str, Architecture}
};
use crate::crypt::mcrypt_sha256_raw;
use crate::pkgdb::PackageDb;

//
// Package
/// Responsible for representing all data about a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)] // Allow serde to do its magic
pub struct Package {
    /// The name of the package
    pub pkgname: String,                 // package name: String (required)
    /// The `Version` of the package
    pub pkgver: Version,                 // package version: semver (required)
    /// The short description of the package
    pub shortdesc: String,               // Short description: String (required)
    /// The optional long description of the package
    pub longdesc: Option<String>,        // Long description: String (optional)
    /// The architecture of the package
    pub arch: Architecture,              // Architecture: Architecture (required)
    /// An optional url for the package
    pub url: Option<String>,             // URL: String (optional)
    /// The optional license of the package
    pub license: Option<String>,         // License: String (optional)
    /// The groups this package is a part of, if any
    pub groups: Option<Vec<String>>,     // Groups: List of String (optional)
    /// The packages this package depends on, if any
    pub depends: Option<Vec<PkgSpec>>,   // Depends: List of PkgSpec (optional)
    /// The package this package optionally depends on, if any
    pub optdepends: Option<Vec<String>>, // Optional Depends: List of String (optional)
    /// The packages this package provides, if any
    pub provides: Option<Vec<PkgSpec>>,  // Provides: List of PkgSpec (optional)
    /// The packages this package conflicts with, if any
    pub conflicts: Option<Vec<PkgSpec>>, // Conflicts: List of PkgSpec (optional)
    /// The packages this package replaces, if any
    pub replaces: Option<Vec<PkgSpec>>,  // Replaces: List of PkgSpec (optional)
    /// The total installed size of this package
    pub installed_size: usize,           // Installed Size: integer (required)
    /// The contents of this package
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
#[allow(clippy::module_name_repetitions)]
pub struct PkgSpec {
    /// The name of the package
    pub pkgname: String,
    /// The version requirements for this package
    pub version: VersionReq,
}

// PackageContents
/// Represents the contents of a package
/// The extraction order should be folders, then files, then links.
/// This is because:
/// - it cannot be guaranteed that the `folders` for all of the `files` will exist if the `folders` are extracted after the `files`
/// - it cannot be guaranteed that the `files` for all of the `links` will exist if the `files` are extracted after the `links`
/// Written differently: folders < files < links
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContents {
    /// The folders present inside this package, if any
    pub folders: Option<Vec<PackageFolder>>,
    /// The files present inside this package, if any
    pub files: Option<Vec<PackageFile>>,
    /// The symbolic links present inside this package, if any
    pub links: Option<Vec<PackageLink>>,
}

// PackageFolder
/// Represents a folder a package creates, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFolder {
    /// The name of the folder inside the package file. Due to a design limitation, this must be the same as `installpath`
    pub name: String,
    /// The last modified time of the folder
    pub mtime: usize,
    /// The installation path for the folder. Due to a design limitation, this must be the same as `name`
    pub installpath: String,
    /// The Unix metadata for this file.
    pub meta: FileMetadata,
}

// PackageFile
/// Represents a file inside a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFile {
    /// The name of the file inside the package file. Due to a design limitation, this must be the same as `installpath`
    pub name: String,
    /// The sha256 hash of the file's contents.
    pub sha256: String,
    /// The Unix metadata for this file.
    pub meta: FileMetadata,
    /// The modification time for this file.
    pub mtime: usize,
    /// The location this file should be extraced to upon installation. Due to a design limitation, this must be the same as `name`
    pub installpath: String,
}

// PackageLink
/// Represents a symbolic link that should be created by a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageLink {
    /// The source file for the symbolic link
    pub file: String,
    /// The modification time of the **symbolic link itself**, not it's target
    pub mtime: usize,
    /// The target file for the symbolic link
    pub target: String,
}

// FileMetadata
/// Represents metadata on a file or folder
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileMetadata {
    /// The file owner's user ID.
    pub owner: usize,
    /// The file owner's group ID.
    pub group: usize,
    /// The file owner's permission tuple.
    pub permissions: usize,
}

/// Utility macro for `VersionReq { comparators: vec![] }` because `VersionReq::any()` is dumb
#[macro_export]
macro_rules! version_any {
    () => {
        VersionReq {
            comparators: vec![],
        }
    };
}

// save_package_raw
/// This function is the raw API for saving a package. You most likely want to use `save_package` and `save_package_signed` instead.
/// # Errors
/// This function, and it's wrapper functions `save_package` and `save_package_signed` will return errors if:
/// - there was an issue creating the fakeroot directory
/// - there was an error creating a directory inside the fakeroot
/// - a file failed sha256 validation
/// - a file could not be copied into the temporary directory
/// - the package metadata could not be saved
/// - there were errors during CWD switches
/// - there were issues creating the archive
/// - there were issues copying files into the archive
/// - the archive could not be saved
/// - the temporary directory could not be removed
/// - the archive could not be opened for compression
/// - the archive could not be read for compression
/// - the archive could not be compressed
/// - the compressed archive could not be written
/// - the temporary archive could not be removed
/// - the file could not be read for encryption
/// - the file could not be encrypted
/// - the encrypted file could not be written
pub fn save_package_raw(package: &Package, data_dir: String, signing_key: Option<PrivateKey>) -> Result<String, Box<dyn Error>> {
    // Step 1: Create temporary dir
    let random_identifier: String = Uuid::new_v4().to_string(); // Get a random uuidv4
    let root_prefix: String = "/tmp/mangrove_build_".to_string(); // prefix

    let root: String = format!("{}{}", root_prefix, random_identifier); // generate the root

    let create_root_result = create_dir_all(&root);
    match create_root_result {
        Ok(_) => (),
        Err(error) => {
            return Err(format!(
                "Failed to create fakeroot directory: {}",
                error
            ).into())
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
                Err(err) => return Err(format!("Unable to create directory: {}", err).into()),
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
                Err(e) => return Err(format!("Failed to verify sha256 hash of {}: {}", &orig, e).into()),
            }
            let target = format!("{}{}", &root, &file.name);
            let copy_result = fs::copy(&orig, target);
            match copy_result {
                Ok(_) => (),
                Err(err) => return Err(format!("Unable to copy file: {}", err).into()),
            }
        }
    }
    // Step 5: Write package metadata
    match Package::as_file(&package, format!("{}/pkginfo", &root)) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to save pkginfo: {}", err).into()),
    }

    // Step 6: Create archive
    let old_dir = match get_cwd() {
        Ok(f) => f,
        Err(err) => return Err(err)
    };
    match set_cwd(&root) {
        Ok(_) => (),
        Err(err) => return Err(err.into()),
    }
    let archive_path_uncompressed = format!("{}/{}.pcm", &data_dir, get_pkg_filename(&package));
    let archive_path = format!("{}/{}", &data_dir, get_pkg_filename(&package));
    let tar_archive_bare = match File::create(&archive_path_uncompressed) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Failed to open file ({}) for writing: {}", &archive_path_uncompressed, err).into()),
    };
    let mut tar = Builder::new(tar_archive_bare);
    if need_files {
        for file in files {
            match tar.append_path(format!("./{}", file.name)) {
                Ok(_) => (),
                Err(err) => return Err(format!("Failed to write file to archive: {}", err).into()),
            }
        }
    }
    match tar.append_path("./pkginfo") {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to write file to archive: {}", err).into()),
    }
    match tar.finish() {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to finalize archive: {}", err).into()),
    }

    // Step 7: Return archive path
    match set_cwd(&old_dir) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // Step 8: Remove dir
    match remove_dir_all(&root) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to remove tmpdir: {}", err).into()),
    }

    // Step 9: Compress file
    let uncompressed_istream = match File::open(&archive_path_uncompressed) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Compress: Failed to open file {} for reading: {}", &archive_path_uncompressed, err).into()),
    };
    let compressed_ostream = match File::create(&archive_path) {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Failed to open file for writing: {}", err).into()),
    };
    match copy_encode(uncompressed_istream, compressed_ostream, 9) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to compress package: {}", err).into()),
    }
    match remove_file(archive_path_uncompressed) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to remove temporary file: {}", err).into()),
    }

    match signing_key {
        None => return Ok(archive_path),
        Some(_) => ()
    }

    // Signing is required
    let dat: Vec<u8> = match fs::read(&archive_path) {
        Ok(dat) => dat,
        Err(err) => return Err(format!("Failed to read file: {}", err).into())
    };
    let sk = match &signing_key {
        Some(k) => k,
        None => return Err("The private key could not be extracted".into())
    };
    let enc_res = match encrypt_package(sk, &dat) {
        Ok(vec) => vec,
        Err(err) => return Err(format!("Failed to encrypt package: {}", err).into())
    };
    match fs::write(&archive_path, enc_res) {
        Ok(_) => (),
        Err(err) => return Err(format!("Failed to write to file: {}", err).into())
    }

    Ok(archive_path)
}

//
// PackageType
/// Represents the type of a package
pub enum PackageType {
    /// Represents an unsigned package
    UnsignedPackage,
    /// Represents a signed package
    SignedPackage
}

// save_package
/// Given a Package and a data_dir, use the files contained in the data_dir to build an unsigned .mgve package
/// # Errors
/// This function is a wrapper for `save_package_backend`, and returns the exact same errors as it.
pub fn save_package(package: &Package, data_dir: String) -> Result<String, Box<dyn Error>> {
    save_package_raw(package, data_dir, None)
}

// save_package_signed
/// Given a Package and a data_dir, use the files contained in the data_dir to build a signed package
/// # Errors
/// This function is a wrapper for `save_package_backend`, and returns the exact same errors as it.
pub fn save_package_signed(package: &Package, data_dir: String, signing_key: PrivateKey) -> Result<String, Box<dyn Error>> {
    save_package_raw(package, data_dir, Some(signing_key))
}

// load_package
/// Given an arbitrary blob, attempt to load it as an **unencrypted!** Package.
/// WILL NOT DECRYPT! You need to use is_signed_package and decrypt_package first.
/// # Errors
/// Literally too many things to list.
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
                None => return Err("Failed to convert string".into())
            },
            None => return Err("Failed to get entry path".into())
        };
        if fname == "pkginfo" {
            /* START BUGGY-LINT-SECTION */
            // Clippy might prompt you to move this code. Don't, as it causes a use-after-free bug in the non-pkginfo case.
            // The lint is buggy and I cannot figure out how to turn it off.
            let mut pkinfo: Vec<u8> = vec![];
            entry.read_to_end(&mut pkinfo)?;
            pkginfo = Some(rmp_serde::from_slice(&*pkinfo)?);
            /* END BUGGY-LINT-SECTION */
        } else {
            let mut fdat: Vec<u8> = vec![];
            entry.read_to_end(&mut fdat)?;
            hashes.insert(format!("/{}", match entry.path()?.to_str() {
                Some(f) => f,
                None => {
                    return Err("Failed to convert string".into())
                }
            }), hex::encode(mcrypt_sha256_raw(&fdat[..])));
        }
    }
    if pkginfo.is_none() {
        return Err("Failed to find pkginfo file".into())
    }
    // Verify hashes
    let pkg = match pkginfo {
        Some(p) => p,
        None => return Err("Pkginfo data missing after check".into())
    };
    if pkg.pkgcontents.files.is_some() {
        let files = match &pkg.pkgcontents.files {
            Some(f) => f,
            None => return Err("Package files field missing".into())
        };
        for file in files {
            if !hashes.contains_key(&*file.name) {
                return Err(format!("Hash for {} is missing", file.name).into())
            }
            let hash = match hashes.get(&*file.name) {
                Some(h) => h,
                None => return Err(format!("Hash for {} is missing", file.name).into())
            };
            if hash != &file.sha256 {
                return Err(format!("Fatal error: hash verification failed for {} (expected {} got {})", file.name, file.sha256, hash).into())
            }
        }
    }
    Ok(pkg)
}

fn show_opt<T: Debug>(opt: Option<T>) -> String {
    opt.map_or_else(|| "Not provided".to_string(), |val| format!("{:?}", val))
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

// extract_pkg_to
/// Extract a &Package to the given target directory, performing validation as it goes.
/// # Errors
/// Once again, due to the amount of filesystem operations there are too many things to list here.
pub fn extract_pkg_to(package: &Vec<u8>, target: String) -> Result<(), Box<dyn Error>> {
    debug!("extract package atl to {}", target);
    let pkginfo = load_package(package)?;
    debug!("pkginfo load success");
    // package is valid, open the archive
    let mut archive = Archive::new(Decoder::new(Cursor::new(package))?);
    debug!("archive load success");
    if let Some(folders) = pkginfo.pkgcontents.folders {
        for folder in folders {
            debug!("creating directory {}", format!("{}{}", target, folder.installpath));
            create_dir_all(format!("{}{}", target, folder.installpath))?;
        }
    }
    if let Some(files) = pkginfo.pkgcontents.files {

        for file_raw in archive.entries()? {
            let mut file = file_raw?;
            for f_to_extract in &files {
                if f_to_extract.installpath == match file.path()?.to_str() {
                    Some(f) => f,
                    None => return Err("Failed to convert string types".into())
                }.to_string() {
                    let mut data: Vec<u8> = vec![];
                    file.read_to_end(&mut data)?;
                    fs::write(format!("{}{}", target, &f_to_extract.installpath), data)?;
                }
            }
        }
    }
    if let Some(links) = pkginfo.pkgcontents.links {
        for link in links {
            symlink(format!("{}{}", target, link.file), format!("{}{}", target, link.target))?;
        }
    }
    Ok(())
}


// install_pkg_to
/// Install a package to the target directory. Performs package validation, dependency checking, and conflict checking.
/// # Errors
/// Once again, due to the amount of filesystem operations there are too many things to list here.
pub fn install_pkg_to(package: &Vec<u8>, target: String, db: &mut PackageDb) -> Result<(), Box<dyn Error>> {
    let pkginfo = load_package(package)?;

    for pkg in &db.db.installed_packages {
        // Conflict checking: another package lists this one as a conflict
        if let Some(conflicts) = &pkg.conflicts {
            let conflicting = conflicts.iter().find(|x| x.pkgname == pkginfo.pkgname && x.version.matches(&pkginfo.pkgver));
            if let Some(conflict) = conflicting {
                return Err(format!("This package conflicts with {}, remove it first", conflict.pkgname).into());
            }
        }
        // Conflict checking: this package lists another one as a conflict
        if let Some(conflicts) = &pkginfo.conflicts {
            let conflicting = conflicts.iter().find(|x| x.pkgname == pkginfo.pkgname && x.version.matches(&pkginfo.pkgver));
            if let Some(conflict) = conflicting {
                return Err(format!("This package conflicts with {}, remove it first", conflict.pkgname).into());
            }
        }
    }
    // No conflicts
    // Dependency checking
    if let Some(dependencies) = &pkginfo.depends {
        for dependency in dependencies {
            if !&db.db.installed_packages.iter().any(|x| x.pkgname == dependency.pkgname && dependency.version.matches(&x.pkgver)) {
                return Err(format!("Required dependency {} not installed", dependency.pkgname).into());
            }
        }
    }
    // Good to go!
    // Extract package files
    extract_pkg_to(package, target)?;
    // Add to package database
    db.db.installed_packages.push(pkginfo);
    // All done!
    Ok(())
}
