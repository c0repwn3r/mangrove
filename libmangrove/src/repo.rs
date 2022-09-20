//! # Structs and functions for dealing with Repositories

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{crypt::PublicKey, pkg::Package, platform::Architecture};

/// Represents a fully configured repository. This is the data contained in /repodata, and is what is synced by the package manager.
#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    /// The base sync url for this repository
    pub baseurl: Url,
    /// The signing key for this repository
    pub signing_key: PublicKey,

    /// The list of avaliable architectures
    pub avaliable_architectures: Vec<Architecture>,
    /// The list of avaliable packages
    pub packages: HashMap<Architecture, Vec<Package>>
}

// get_repoinfo_url
/// Gets the RepoInfo `Url` for a given baseurl.
//
pub fn get_repoinfo_url(baseurl: Url) -> Result<Url, url::ParseError> {
    baseurl.join("repoinfo")
}