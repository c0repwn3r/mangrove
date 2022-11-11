//! Package database and trustcache

extern crate ed25519_dalek;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::pkg::Package;
use crate::repo::Repository;

// Database
/// Represents the package database on disk, contains a list of all installed packages and all configured repositories
//
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    /// This is a list of all of the installed packages
    pub installed_packages: Vec<Package>,
    /// This is a list of the configured repositories
    pub repositories: Vec<ConfiguredRepository>,
}

/// Represents a configured repository. Just contains it's base URL and the synced data.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfiguredRepository {
    /// Represents the repository sync baseurl.
    pub baseurl: Url,
    /// Represents the synced repository data.
    pub repodata: Repository
}

// KeyDb
/// The core of the trustcache, contains a list of trusted public keys and private keys
/// All `PrivateKeys` will also have their associated `PublicKey` inferred.
/// The operation for checking a key goes in this order:
/// 1. check if it is in `deny_privkeys` (public or private)
/// -> if so: error
/// 2. check if it is in `deny_pubkeys`
/// -> if so: error
/// 3. check if it is in `known_privkeys` (public or private)
/// -> if so: stop, success
/// 4. check if it is in `known_pubkeys`
/// -> if so: stop, success
/// 5. error, key not known
//
#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct KeyDb {
    /// A list of the known public keys. You do not need to include the public key of a private key in `known_privkeys`, as it will be inferred.
    pub known_pubkeys: Vec<String>,
    /// A list of the known private keys. Each of these will have their public keys inferred, and as such do not need to have their associated public key in `known_pubkeys`
    pub known_privkeys: Vec<String>,

    /// An immediate blacklist for keys. If a key is found in this list, it will be considered unknown and an error will be returned.
    pub deny_pubkeys: Vec<String>,
    /// An immediate blacklist for private keys. Keys in this list will not be used to sign packages, and if the associated public key is found in this list, it will be considered unknown and an error will be returned.
    pub deny_privkeys: Vec<String>
}