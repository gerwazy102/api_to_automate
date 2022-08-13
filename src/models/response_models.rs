use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IDResponse {
    pub id: String,
}

impl IDResponse {
    pub fn from_str(id: &str) -> IDResponse {
        IDResponse {
            id: String::from_str(id).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub info: String,
}

impl InfoResponse {
    pub fn from_str(info: &str) -> InfoResponse {
        InfoResponse {
            info: String::from_str(info).unwrap(),
        }
    }
}
