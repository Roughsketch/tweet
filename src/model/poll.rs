use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::util::datetime::{datefmt_de, datefmt_ser};

/// Represents a twitter poll
#[derive(Debug, Deserialize, Serialize)]
pub struct Poll {
    /// All the options for the poll
    pub options: Vec<PollOption>,
    /// When the poll ends
    #[serde(deserialize_with="datefmt_de", serialize_with="datefmt_ser")]
    pub end_datetime: DateTime<Utc>,
    /// How long the poll has been running
    pub duration_minutes: String,
}

/// Represents an option in a poll
#[derive(Debug, Deserialize, Serialize)]
pub struct PollOption {
    /// What position in the poll this option has
    pub position: u32,
    /// Option text
    pub text: String,
}