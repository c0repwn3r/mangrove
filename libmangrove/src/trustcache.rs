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
use crate::crypt::{PrivateKey, PublicKey};
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

// has_assoc_sk
/// Determine if the trustcache contains the associated secret key for the provided public key.
/// # Errors
/// An error will occur if an invalid key is present in the trustcache.
pub fn has_assoc_sk(trustcache: &Trustcache, key: &PublicKey) -> Result<bool, Box<dyn Error>> {
    // This function is really inefficient, someone please optimize it
    // It was just a quick hack to get mgve trust working
    for sk in &trustcache.keydb.known_privkeys {
        if PrivateKey::from_anonymous(sk)?.derive().to_anonymous() == key.to_anonymous() {
            return Ok(true);
        }
    }
    Ok(false)
}

// assoc_sk_blacklisted
/// Determine if the trustcache blacklists the private key associated with the public key provided.
/// # Errors
/// An error will occur if an invalid key is present in the trustcache.
pub fn assoc_sk_blacklisted(trustcache: &Trustcache, key: &PublicKey) -> Result<bool, Box<dyn Error>> {
    // This function is really inefficient, someone please optimize it
    // It was just a quick hack to get mgve trust working
    for sk in &trustcache.keydb.deny_privkeys {
        if PrivateKey::from_anonymous(sk)?.derive().to_anonymous() == key.to_anonymous() {
            return Ok(true);
        }
    }
    Ok(false)
}

// is_pk_blacklisted
/// Determines if the provided public key is blacklisted.
pub fn is_pk_blacklisted(trustcache: &Trustcache, key: &PublicKey) -> Result<bool, Box<dyn Error>> {
    for pk in &trustcache.keydb.deny_pubkeys {
        if pk == &key.to_anonymous() {
            return Ok(true)
        }
    }
    assoc_sk_blacklisted(trustcache, key)
}

// is_sk_blacklisted
/// Determines if the provided private key is blacklisted.
pub fn is_sk_blacklisted(trustcache: &Trustcache, key: &PrivateKey) -> Result<bool, Box<dyn Error>> {
    for sk in &trustcache.keydb.deny_privkeys {
        if sk == &key.to_anonymous() {
            return Ok(true)
        }
    }
    is_pk_blacklisted(trustcache, &key.derive())
}

// is_pk_trusted
/// Determines if the provided public key is in the trustcache.
/// # Errors
/// An error will occur if an invalid key is present in the trustcache.
pub fn is_pk_trusted(trustcache: &Trustcache, key: &PublicKey) -> Result<bool, Box<dyn Error>> {
    if is_pk_blacklisted(trustcache, key)? { return Ok(false); }
    for pk in &trustcache.keydb.known_pubkeys {
        if pk == &key.to_anonymous() {
            return Ok(true)
        }
    }
    has_assoc_sk(trustcache, key)
}

// is_sk_trusted
/// Determines if the provided private key is in the trustcache.
pub fn is_sk_trusted(trustcache: &Trustcache, key: &PrivateKey) -> Result<bool, Box<dyn Error>> {
    if is_sk_blacklisted(trustcache, key)? { return Ok(false); }
    for sk in &trustcache.keydb.known_privkeys {
        if sk == &key.to_anonymous() {
            return Ok(true)
        }
    }
    is_pk_trusted(trustcache, &key.derive())
}

// allow_sk
/// Add a secret key to the allowlist, removing it from the blacklist if it's blacklisted.
pub fn allow_sk(trustcache: &mut Trustcache, key: &PrivateKey) -> Result<(), Box<dyn Error>> {
    if is_sk_trusted(trustcache, key)? { return Ok(()); } // done! already trusted
    if is_sk_blacklisted(trustcache, key)? {
        // remove from the blacklist
        let index = match trustcache.keydb.deny_privkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.deny_privkeys.remove(index);
    }
    trustcache.keydb.known_privkeys.push(key.to_anonymous());
    Ok(())
}
// deny_sk
/// Add a secret key to the blocklist, reoving it from the allowlist if it's allowlisted.
pub fn deny_sk(trustcache: &mut Trustcache, key: &PrivateKey) -> Result<(), Box<dyn Error>> {
    if is_sk_blacklisted(trustcache, key)? { return Ok(()); } // done! already trusted
    if is_sk_trusted(trustcache, key)? {
        // remove from the allowlist
        let index = match trustcache.keydb.known_privkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.known_privkeys.remove(index);
    }
    trustcache.keydb.deny_privkeys.push(key.to_anonymous());
    Ok(())
}
// clear_sk
/// Remove a secret key from the entirety of the trustcache, removing it from the allowlist and blacklist if either are present.
pub fn clear_sk(trustcache: &mut Trustcache, key: &PrivateKey) -> Result<(), Box<dyn Error>> {
    if is_sk_blacklisted(trustcache, key)? {
        // remove from the blacklist
        let index = match trustcache.keydb.deny_privkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.deny_privkeys.remove(index);
    }
    if is_sk_trusted(trustcache, key)? {
        // remove from the allowlist
        let index = match trustcache.keydb.known_privkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.known_privkeys.remove(index);
    }
    Ok(())
}

// allow_pk
/// Add a public key to the allowlist, removing it from the blacklist if it's blacklisted.
pub fn allow_pk(trustcache: &mut Trustcache, key: &PublicKey) -> Result<(), Box<dyn Error>> {
    if is_pk_trusted(trustcache, key)? { return Ok(()); } // done! already trusted
    if is_pk_blacklisted(trustcache, key)? {
        // remove from the blacklist
        if assoc_sk_blacklisted(trustcache, key)? {
            return Err("Key is blacklisted by association".into())
        }
        let index = match trustcache.keydb.deny_pubkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.deny_pubkeys.remove(index);
    }
    trustcache.keydb.known_pubkeys.push(key.to_anonymous());
    Ok(())
}
// deny_pk
/// Add a public key to the blocklist, reoving it from the allowlist if it's allowlisted.
pub fn deny_pk(trustcache: &mut Trustcache, key: &PublicKey) -> Result<(), Box<dyn Error>> {
    if is_pk_blacklisted(trustcache, key)? { return Ok(()); } // done! already trusted
    if is_pk_trusted(trustcache, key)? {
        // remove from the allowlist
        if has_assoc_sk(trustcache, key)? {
            return Err("Key is trusted by association".into());
        }
        let index = match trustcache.keydb.known_pubkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in allowlist".into())
        };
        trustcache.keydb.known_pubkeys.remove(index);
    }
    trustcache.keydb.deny_pubkeys.push(key.to_anonymous());
    Ok(())
}
// clear_pk
/// Remove a public key from the entirety of the trustcache, removing it from the allowlist and blacklist if either are present.
pub fn clear_pk(trustcache: &mut Trustcache, key: &PublicKey) -> Result<(), Box<dyn Error>> {
    if is_pk_blacklisted(trustcache, key)? {
        // remove from the blacklist
        if assoc_sk_blacklisted(trustcache, key)? {
            return Err("Key is blacklisted by association".into())
        }
        let index = match trustcache.keydb.deny_pubkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.deny_pubkeys.remove(index);
    }
    if is_pk_trusted(trustcache, key)? {
        // remove from the allowlist
        if has_assoc_sk(trustcache, key)? {
            return Err("Key is trusted by association".into());
        }
        let index = match trustcache.keydb.known_pubkeys.iter().position(|r| r == &key.to_anonymous()) {
            Some(i) => i,
            None => return Err("Failed to find index of item in blacklist".into())
        };
        trustcache.keydb.known_pubkeys.remove(index);
    }
    Ok(())
}