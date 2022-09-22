//! # Package Database
//! This module contains the required functions for working with the package database.

use std::error::Error;
use std::fs;
use std::path::Path;

use lockfile::Lockfile;

use crate::config::get_pkgdb_file;
use crate::db::Database;
use crate::lock::lock_packages;

#[allow(dead_code)] // idk why this is warned. this is literally immediately constructed right below here
#[derive(Debug)]
/// Represents a lockfile for the pkgdb and the actual db data
pub struct PackageDb {
    /// The packagedb Lockfile, to ensure a mutex lock on the pkgdb while it is being operated upon
    pub lockfile: Lockfile,
    /// The actual data
    pub db: Database
}

// pkgdb_load
/// Loads the package database from disk and into a `Database`. This also locks the database.
/// # Errors
/// This function can return errors for a number of reasons:
/// - the trustcache is already locked
/// - permission denied while trying to lock the database
/// - permission denied while trying to create the configuration directory
/// - permission denied while creating a default database
/// - failed to serialize the database data
/// - failed to write the database data
/// - failed to read the database data
/// - failed to deserialize the database data
#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)] // used in tests, clippy wtf
pub fn pkgdb_load(local: bool) -> Result<PackageDb, Box<dyn Error>> {
    // attempt to lock the trustcache
    let lockfile = lock_packages(local)?;
    // we have the lock now, load the trustcache
    if !Path::new(&get_pkgdb_file(local)).exists() {
        // need to create the trustcache
        let data = Database {
            installed_packages: vec![],
            repositories: vec![]
        };
        fs::write(get_pkgdb_file(local), rmp_serde::to_vec(&data)?)?;
    }
    let db: Database = rmp_serde::from_slice(&*fs::read(get_pkgdb_file(local))?)?;
    // return a trustcache object
    Ok(
        PackageDb {
            lockfile,
            db
        }
    )
}

// pkgdb_save
/// Save the database from a PackageDatabase object. Requires there to be a lock on the database. Will release that lock.
/// # Errors
/// As with `pkgdb_load`, this function can return errors for any number of reasons:
/// - failed to serialize the database
/// - failed to write the database to disk
/// - failed to release the mutex lock on the database
#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)] // once again, clippy wtf
pub fn pkgdb_save(database: PackageDb, local: bool) -> Result<(), Box<dyn Error>> {
    // save the trustcache
    let str = rmp_serde::to_vec(&database.db)?;
    fs::write(get_pkgdb_file(local), str)?;
    database.lockfile.release()?;
    Ok(())
}