//! # Structs and functions for dealing with Repositories

use serde::{Deserialize, Serialize};
use version::Version;

use crate::{crypt::PublicKey, pkg::Package, platform::Architecture};

// Repository
/// Contains all data the package manager knows about a repository
//
pub struct Repository {
    /// The name of the repository
    pub repo_name: String,
    /// The base HTTP URL of the repository
    pub repo_base_url: String,
    /// The repository's supported architectures
    pub supported_architectures: Vec<Architecture>,
    /// The contents of the repository
    pub contents: Vec<RepoData>,
    /// The signing key for the repository
    pub signing_key: PublicKey,
}

// RepoInfo
/// Contains information on a repository
//
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfo {
    /// The name of the repository
    pub repo_name: String,                          // Repository name, required
    /// The base HTTP URL of the repository
    pub repo_base_url: String,                      // Base URL of the repository, required
    /// The repository's supported architectures
    pub supported_architectures: Vec<Architecture>, // List of required architectures, required
}

// RepoData
/// Contains information on the packages contained within a repository
//
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    /// Contains the architecture for this RepoData blob
    pub architecture: Architecture,
    /// The package headers for this RepoData blob
    pub packages: Vec<RepoPackage>,
}

// RepoPackage
/// Contains information on a single package contained within a repository
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RepoPackage {
    /// The current package header
    pub package_data: Package,
    /// The available versions
    pub available_versions: Vec<Version>,
}
