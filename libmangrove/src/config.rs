//! Configuration files

use std::fs::create_dir_all;

// All config locations:
// /etc/mangrove            - root
// /etc/mangrove/locks      - lockfiles
// /etc/mangrove/repos      - repositories
// /etc/mangrove/trust.toml - trust settings

pub fn ensure_config(local: bool) -> Result<(), String> {
    match create_dir_all(if local {"./test/config/"} else {"/etc/mangrove"}) {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    match create_dir_all(if local {"./test/config/locks/"} else {"/etc/mangrove/locks/"}) {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    match create_dir_all(if local {"./test/config/repos/"} else {"/etc/mangrove/repos/"}) {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    Ok(())
}

pub fn get_trustcache_file(local: bool) -> String {
    if local {
        "./trust.toml".to_string()
    } else {
        "/etc/mangrove/trust.toml".to_string()
    }
}