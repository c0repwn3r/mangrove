//! Package database and trustcache

use serde::{Deserialize, Serialize};

extern crate ed25519_dalek;

use crate::{
    pkg::Package,
    repo::{RepoData, RepoInfo}, crypt::{PublicKey},
};

// Database
/// Represents the package database on disk, contains a list of all installed packages and all configured repositories
//
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub installed_packages: Vec<Package>,
    pub repositories: Vec<DbRepository>,
}

// DbRepository
/// Represents a configured repository, contains a RepoInfo struct, RepoData for the contents of the repo, and the repo's signing key
//
#[derive(Serialize, Deserialize, Debug)]
pub struct DbRepository {
    pub repo_info: RepoInfo,
    pub contents: RepoData,
    pub signing_key: PublicKey,
}

// KeyDb
/// The core of the trustcache, contains a list of trusted public keys and private keys
/// All `PrivateKeys` will also have their associated `PublicKey` inferred
//
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyDb {
    pub known_pubkeys: Vec<String>,
    pub known_privkeys: Vec<String>,

    pub deny_pubkeys: Vec<String>,
    pub deny_privkeys: Vec<String>
}