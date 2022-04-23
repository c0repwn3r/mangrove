use serde::{Deserialize, Serialize};

extern crate ed25519_dalek;

use crate::{
    pkg::Package,
    repo::{RepoData, RepoInfo}, crypt::{PublicKey, PrivateKey},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub installed_packages: Vec<Package>,
    pub repositories: Vec<DbRepository>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbRepository {
    pub repo_info: RepoInfo,
    pub contents: RepoData,
    pub signing_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyDb {
    pub pubkeys: Vec<PublicKey>,
    pub privkeys: Vec<PrivateKey>,
}