use serde_derive::Deserialize;

/// Represents a twitter poll
#[derive(Debug, Deserialize)]
pub struct Poll {
    /// All the options for the poll
    pub options: Vec<PollOption>,
    /// When the poll ends
    pub end_datetime: String,
    /// How long the poll has been running
    pub duration_minutes: String,
}

/// Represents an option in a poll
#[derive(Debug, Deserialize)]
pub struct PollOption {
    /// What position in the poll this option has
    pub position: u32,
    /// Option text
    pub text: String,
}