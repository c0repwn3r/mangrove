//! # libmangrove
//!
//! `libmangrove` is a crate containing the original (reference) implementation of the Mangrove packaging system.
//! **It does not implement the CLI, it is merely the library to perform the actual packaging operations.**
//! Looking for the CLI? Check out mangrove-cli.
//!
#[deny(missing_docs)]

extern crate core;

use git_version::git_version;

pub mod crypt; // Various cryptographic helper functions to remove repetitive code
pub mod db; // Package database
pub mod file; // Traits, structs, and functions for interfacing with the filesystem
pub mod pkg; // Structs and functions for dealing with Packages
pub mod pkginfo; // Provides implementation of FileOps
pub mod platform; // Platform-specific code
pub mod repo; // Structs and functions for dealing with Repositories
pub mod stropt; // String operations
pub mod test; // Testing
#[macro_use]
pub mod aes; // AES helper functions
pub mod config; // Configuration
pub mod lock; // Lockfiles
pub mod trustcache; // Trustcache management

// Version stuff //
pub fn pkgver() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
pub fn gitver() -> String {
    git_version!().to_string()
}
pub fn version() -> String {
    format!("libmangrove {} ({})", pkgver(), gitver())
}

pub fn detailed_version() -> String {
    format!("libmangrove {} ({})\nbuilt on {} at {}, commitinfo {}/{} (dirty: {}), rust channel {} rustc {}\nsource timestamp {}",
            pkgver(), gitver(), env!("BUILD_HOSTNAME"), env!("BUILD_TIMESTAMP"), env!("GIT_BRANCH"), env!("GIT_COMMIT_SHORT"), env!("GIT_DIRTY"), env!("RUST_CHANNEL"),
            env!("RUSTC_VERSION"), env!("SOURCE_TIMESTAMP"))
}