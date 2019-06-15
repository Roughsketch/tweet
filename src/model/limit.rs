use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

impl FromStr for Limit {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Limit {
    limit: LimitFields,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LimitFields {
    track: u32,
    timestamp_ms: String,
}