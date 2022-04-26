//! Lockfiles (e.g. /etc/mangrove/locks/pkgdb.lock)

use lockfile::Lockfile;

use crate::config::ensure_config;

/*
Standard lockfile locations:
Repository operations - /etc/mangrove/locks/repo.lock
Trustcache operations - /etc/mangrove/locks/trustcache.lock
Package operations    - /etc/mangrove/locks/package.lock
*/

pub fn lock_repository() -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }
    let lock = match Lockfile::create("/etc/mangrove/locks/repo.lock") {
        Ok(l) => l,
        Err(_) => return Err("Locked by another process".to_string()),
    };

    Ok(lock)
}
pub fn lock_trustcache() -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }
    let lock = match Lockfile::create("/etc/mangrove/locks/trustcache.lock") {
        Ok(l) => l,
        Err(_) => return Err("Locked by another process".to_string()),
    };

    Ok(lock)
}
pub fn lock_packages() -> Result<Lockfile, String> {
    match ensure_config() {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err)),
    }
    let lock = match Lockfile::create("/etc/mangrove/locks/package.lock") {
        Ok(l) => l,
        Err(_) => return Err("Locked by another process".to_string()),
    };

    Ok(lock)
}
