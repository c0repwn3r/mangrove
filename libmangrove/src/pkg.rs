use serde::{Deserialize, Serialize};

use crate::platform::Architecture;
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
