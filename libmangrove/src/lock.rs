//! Lockfiles (e.g. /etc/mangrove/locks/pkgdb.lock)

use std::error::Error;

use lockfile::Lockfile;

use crate::config::create_config_structure;

/*
Standard lockfile locations:
Repository operations - /etc/mangrove/locks/repo.lock
Trustcache operations - /etc/mangrove/locks/trustcache.lock
Package operations    - /etc/mangrove/locks/package.lock
*/

/// Attempt to get a lock on the repository datastructures
/// # Errors
/// This function will error if:
/// - the locks directory could not be created
/// - the lock could not be created
#[allow(clippy::module_name_repetitions)]
pub fn lock_repository(use_local_lockfile: bool) -> Result<Lockfile, Box<dyn Error>> {
    match create_config_structure(use_local_lockfile) {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err).into()),
    }

    let path = if use_local_lockfile {"repo.lock"} else {"/etc/mangrove/locks/repo.lock"};

    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e).into()),
    };

    Ok(lock)
}
/// Attempt to get a lock on the trustcache
/// # Errors
/// This function will error if:
/// - the locks directory could not be created
/// - the lock could not be created
#[allow(clippy::module_name_repetitions)]
pub fn lock_trustcache(use_local_lockfile: bool) -> Result<Lockfile, Box<dyn Error>> {
    match create_config_structure(use_local_lockfile) {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err).into()),
    }

    let path = if use_local_lockfile {"trustcache.lock"} else {"/etc/mangrove/locks/trustcache.lock"};

    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e).into()),
    };

    Ok(lock)
}
/// Attempt to get a lock on the packages datastructures
/// # Errors
/// This function will error if:
/// - the locks directory could not be created
/// - the lock could not be created
#[allow(clippy::module_name_repetitions)]
pub fn lock_packages(use_local_lockfile: bool) -> Result<Lockfile, Box<dyn Error>> {
    match create_config_structure(use_local_lockfile) {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to ensure config dir: {}", err).into()),
    }

    let path = if use_local_lockfile {"package.lock"} else {"/etc/mangrove/locks/package.lock"};


    let lock = match Lockfile::create(path) {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to lock: {:?}", e).into()),
    };

    Ok(lock)
}
