use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

mod model;
mod util;
pub use crate::model::*;

impl FromStr for TwitterResponse {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TwitterResponse {
    Tweet(Tweet),
    Limit(Limit),
}