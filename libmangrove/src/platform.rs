use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Architecture {
    Amd64,
    Arm64,
    Armv7,
}

pub fn arch_str(arch: &Architecture) -> String {
    match arch {
        Architecture::Amd64 => "amd64".to_string(),
        Architecture::Arm64 => "arm64".to_string(),
        Architecture::Armv7 => "armv7".to_string(),
    }
}
