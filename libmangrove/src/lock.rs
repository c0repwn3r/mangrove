//! Lockfiles (e.g. /etc/mangrove/locks/pkgdb.lock)

use lockfile::Lockfile;

use crate::config::ensure_config;

/*
Standard lockfile locations:
Repository operations - /etc/mangrove/locks/repo.lock
Trustcache operations - /etc/mangrove/locks/trustcache.lock
Package operations    - /etc/mangrove/locks/package.lock
*/

pub fn lock_repository(use_local_lockfile: bool) -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }

    let path = if use_local_lockfile {"repo.lock"} else {"/etc/mangrove/locks/repo.lock"};

    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e)),
    };

    Ok(lock)
}
pub fn lock_trustcache(use_local_lockfile: bool) -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }

    let path = if use_local_lockfile {"trustcache.lock"} else {"/etc/mangrove/locks/trustcache.lock"};

    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e)),
    };

    Ok(lock)
}
pub fn lock_packages(use_local_lockfile: bool) -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }

    let path = if use_local_lockfile {"package.lock"} else {"/etc/mangrove/locks/package.lock"};


    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e)),
    };

    Ok(lock)
}
