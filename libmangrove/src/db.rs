use serde::{Deserialize, Serialize};

use crate::{
    pkg::Package,
    repo::{RepoData, RepoInfo},
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
