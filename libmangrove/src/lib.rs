//! # libmangrove
//!
//! `libmangrove` is a crate containing the original (reference) implementation of the Mangrove packaging system.
//! **It does not implement the CLI, it is merely the library to perform the actual packaging operations.**
//! Looking for the CLI? Check out mangrove-cli.
//!
// Linter configuration
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

// Outright bad practice for a library
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]


// Just shut up
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_lines)]


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
/// Get the cargo package version
pub fn pkgver() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
/// Get the git tag and commit at build time
pub fn gitver() -> String {
    git_version!().to_string()
}
/// Get the short version string
pub fn version() -> String {
    format!("libmangrove {} ({})", pkgver(), gitver())
}
/// Get the detailed version string
pub fn detailed_version() -> String {
    format!("libmangrove {} ({})\nbuilt on {} at {}, commitinfo {}/{} (dirty: {}), rust channel {} rustc {}\nsource timestamp {}",
            pkgver(), gitver(), env!("BUILD_HOSTNAME"), env!("BUILD_TIMESTAMP"), env!("GIT_BRANCH"), env!("GIT_COMMIT_SHORT"), env!("GIT_DIRTY"), env!("RUST_CHANNEL"),
            env!("RUSTC_VERSION"), env!("SOURCE_TIMESTAMP"))
}