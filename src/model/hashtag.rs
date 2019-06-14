use serde_derive::Deserialize;

/// Represents a hashtag
#[derive(Debug, Deserialize)]
pub struct Hashtag {
    /// The indices of the hastag in the original tweet message
    pub indices: Vec<u32>,
    /// The text of the hashtag
    pub text: String,
}