use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Architecture {
    Amd64,
    Arm64,
    Armv7
}