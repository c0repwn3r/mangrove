use std::error::Error;
use std::fs;
use std::path::Path;
use lockfile::Lockfile;
use crate::config::get_trustcache_file;
use crate::db::KeyDb;
use crate::lock::lock_trustcache;

// Trustcache
/// Provides a mutual lock on the trustcache and also access to the KeyDb.
#[derive(Debug)]
pub struct Trustcache {
    pub lockfile: Lockfile,
    pub keydb: KeyDb
}

// trustcache_load
/// Loads the trustcache from disk and into a KeyDb. This also locks the trustcache.
//
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
            lockfile: lockfile,
            keydb: trustcache
        }
    )
}

// trustcache_save
/// Save the trustcache from a Trustcache object. Requires there to be a lock on the trustcache. Will release that lock.
//
pub fn trustcache_save(trustcache: Trustcache, use_local_trustcache: bool) -> Result<(), Box<dyn Error>> {
    // save the trustcache
    let str = toml::to_string_pretty(&trustcache.keydb)?;
    fs::write(get_trustcache_file(use_local_trustcache), str)?;
    trustcache.lockfile.release()?;
    Ok(())
}