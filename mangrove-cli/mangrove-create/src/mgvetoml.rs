use version::Version;
use serde::{Serialize,Deserialize};
use libmangrove::pkg::PkgSpec;
use libmangrove::platform::{arch_str, Architecture};

#[derive(Serialize, Deserialize, Debug, PartialEq)] // Allow serde to do its magic
pub struct BuildConfig {
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
}

pub fn buildtoml_name(cf: &BuildConfig) -> String {
    return format!(
        "{}_{}_{}.mgve",
        cf.pkgname,
        cf.pkgver,
        arch_str(&cf.arch)
    )
}