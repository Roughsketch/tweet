use serde_derive::{Deserialize, Serialize};

/// Represents a hashtag
#[derive(Debug, Deserialize, Serialize)]
pub struct Hashtag {
    /// The indices of the hastag in the original tweet message
    pub indices: Vec<u32>,
    /// The text of the hashtag
    pub text: String,
}