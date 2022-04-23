use serde::{Deserialize, Serialize};
use version::Version;

use crate::{pkg::Package, platform::Architecture, crypt::PublicKey};

// Repository
// Contains all data the package manager knows about a repository
// !! this is not serializable, see RepoInfo and RepoData for the actual contents of a repo
//
pub struct Repository {
    pub repo_name: String,
    pub repo_base_url: String,
    pub supported_architectures: Vec<Architecture>,
    pub contents: Vec<RepoData>,
    pub signing_key: PublicKey,
}

// RepoInfo
// Contains information on a repository
//
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfo {
    pub repo_name: String,                          // Repository name, required
    pub repo_base_url: String,                      // Base URL of the repository, required
    pub supported_architectures: Vec<Architecture>, // List of required architectures, required
}

// RepoData
// Contains information on the packages contained within a repository
//
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    pub architecture: Architecture,
    pub packages: Vec<RepoPackage>,
}

// RepoPackage
// Contains information on a single package contained within a repository
//
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPackage {
    pub package_data: Package,
    pub avaliable_versions: Vec<Version>,
}
