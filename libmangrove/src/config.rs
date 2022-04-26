//! Configuration files

use std::fs::create_dir_all;

// All config locations:
// /etc/mangrove            - root
// /etc/mangrove/locks      - lockfiles
// /etc/mangrove/repos      - repositories
// /etc/mangrove/trust.toml - trust settings
// /etc/mangrove/trust/     - trustcache

pub fn ensure_config() -> Result<(), String> {
    match create_dir_all("/etc/mangrove") {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    match create_dir_all("/etc/mangrove/locks") {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    match create_dir_all("/etc/mangrove/repos") {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    match create_dir_all("/etc/mangrove/trust") {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    }
    Ok(())
}
