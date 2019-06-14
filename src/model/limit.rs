use serde_derive::Deserialize;
use std::str::FromStr;

impl FromStr for Limit {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct Limit {
    limit: LimitFields,
}

#[derive(Debug, Deserialize)]
pub struct LimitFields {
    track: u32,
    timestamp_ms: String,
}