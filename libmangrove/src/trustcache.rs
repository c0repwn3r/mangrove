//! # Trustcache
//! The trustcache is one of the most important, and simplest, components of Mangrove.
//! It is a toml file, usually located at /etc/mangrove/trust.toml, which is responsible for recording
//! known public and private keys, and is used to prevent the need for specifying keys for every action.
//! This module contains the core structures and functions for operating with the trustcache.

use std::error::Error;
use std::fs;
use std::path::Path;

use lockfile::Lockfile;

use crate::config::get_trustcache_file;
use crate::db::KeyDb;
use crate::lock::lock_trustcache;

// Trustcache
/// Provides a mutual lock on the trustcache and also access to the `KeyDb`.
#[derive(Debug)]
pub struct Trustcache {
    /// The trustcache Lockfile, to ensure a mutex lock on the trustcache while it is being operated upon
    pub lockfile: Lockfile,
    /// The actual trustcache data
    pub keydb: KeyDb
}

// trustcache_load
/// Loads the trustcache from disk and into a `KeyDb`. This also locks the trustcache.
/// # Errors
/// This function can return errors for a number of reasons:
/// - the trustcache is already locked
/// - permission denied while trying to lock the trustcache
/// - permission denied while trying to create the configuration directory
/// - permission denied while creating a default trustcache
/// - failed to serialize the trustcache data
/// - failed to write the trustcache data
/// - failed to read the trustcache data
/// - failed to deserialize the trustcache data
#[allow(clippy::module_name_repetitions)]
pub fn trustcache_load(use_local_trustcache: bool) -> Result<Trustcache, Box<dyn Error>> {
    // attempt to lock the trustcache
    let lockfile = lock_trustcache(use_local_trustcache)?;
    // we have the lock now, load the trustcache
    if !Path::new(&get_trustcache_file(use_local_trustcache)).exists() {
        // need to create the trustcache
        let data = KeyDb {
            known_pubkeys: vec![],
            known_privkeys: vec![],
            deny_pubkeys: vec![],
            deny_privkeys: vec![]
        };
        fs::write(get_trustcache_file(use_local_trustcache), toml::to_vec(&data)?)?;
    }
    let trustcache: KeyDb = toml::from_slice(&*fs::read(get_trustcache_file(use_local_trustcache))?)?;
    // return a trustcache object
    Ok(
        Trustcache {
            lockfile,
            keydb: trustcache
        }
    )
}

// trustcache_save
/// Save the trustcache from a Trustcache object. Requires there to be a lock on the trustcache. Will release that lock.
/// # Errors
/// As with `trustcache_load`, this function can return errors for any number of reasons:
/// - failed to serialize the trustcache
/// - failed to write the trustcache to disk
/// - failed to release the mutex lock on the trustcache
#[allow(clippy::module_name_repetitions)]
pub fn trustcache_save(trustcache: Trustcache, use_local_trustcache: bool) -> Result<(), Box<dyn Error>> {
    // save the trustcache
    let str = toml::to_string_pretty(&trustcache.keydb)?;
    fs::write(get_trustcache_file(use_local_trustcache), str)?;
    trustcache.lockfile.release()?;
    Ok(())
}