//! # Platform-specific code

use serde::{Deserialize, Serialize};

// Architecture
/// Represents a system architecture
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[allow(missing_docs)] // this is self explanatory
pub enum Architecture {
    #[allow(non_camel_case_types)]
    amd64,
    #[allow(non_camel_case_types)]
    arm64,
    #[allow(non_camel_case_types)]
    armv7,
}

// arch_str
/// Converts and Architecture to its String variant
//
pub fn arch_str(arch: &Architecture) -> String {
    match arch {
        Architecture::amd64 => "amd64".to_string(),
        Architecture::arm64 => "arm64".to_string(),
        Architecture::armv7 => "armv7".to_string(),
    }
}
