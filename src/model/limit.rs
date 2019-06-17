use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

impl FromStr for Limit {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// When Twitter has more messages to send than
/// what rate limiting would allow, you will
/// receive a limit payload. This documents how
/// many messages have been left undelivered.
#[derive(Debug, Deserialize, Serialize)]
pub struct Limit {
    /// Contains information about the limit
    limit: LimitFields,
}

/// Holds information on a Limit payload
#[derive(Debug, Deserialize, Serialize)]
pub struct LimitFields {
    /// Total count of all undelivered tweets since
    /// the connection was established.
    track: u32,
    /// Timestamp of the payload
    timestamp_ms: String,
}