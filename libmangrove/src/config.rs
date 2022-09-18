//! Configuration files

use std::error::Error;
use std::fs::create_dir_all;

// All config locations:
// /etc/mangrove            - root
// /etc/mangrove/locks      - lockfiles
// /etc/mangrove/repos      - repositories
// /etc/mangrove/trust.toml - trust settings

// ensure_config
/// This function is used to create the expected configuration structure at the specified location.
/// If `local` is true, it will be created in "./test/config", otherwise it will be in "/etc/mangrove"
/// # Errors
/// This function will error if there are any errors while creating the directories.
pub fn create_config_structure(local: bool) -> Result<(), Box<dyn Error>> {
    create_dir_all(if local {"./test/config/"} else {"/etc/mangrove"})?;
    create_dir_all(if local {"./test/config/locks/"} else {"/etc/mangrove/locks/"})?;
    create_dir_all(if local {"./test/config/repos/"} else {"/etc/mangrove/repos/"})?;
    Ok(())
}

// get_trustcache_file
/// This function is used to determine what file the trustcache should be stored in, depending if it is `local` or not.
/// If `local` is true, this will return "./trust.toml", otherwise "/etc/mangrove/trust.toml". Subject to change.
pub fn get_trustcache_file(local: bool) -> String {
    if local {
        "./trust.toml".to_string()
    } else {
        "/etc/mangrove/trust.toml".to_string()
    }
}