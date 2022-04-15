use serde::{Serialize, Deserialize};

// VersionRange
// Represents a range of possible versions
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VersionRange {
    pub any: bool,
    pub minver: Option<Version>,
    pub maxver: Option<Version>
}

// Version
// Represents a single version
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version {
    pub major: u32,                 // x.0.0-prerelease+build
    pub minor: u32,                 // 0.x.0-prerelease+build
    pub patch: u32,                 // 0.0.x-prerelease+build
    pub prerelease: Option<String>, // 0.0.0-x+build
    pub build: Option<String>       // 0.0.0-prerelease+x
}