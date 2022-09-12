//! # libmangrove
//!
//! `libmangrove` is a crate containing the original (reference) implementation of the Mangrove packaging system.
//! **It does not implement the CLI, it is merely the library to perform the actual packaging operations.**
//! Looking for the CLI? Check out mangrove-cli.
//!

extern crate core;

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
