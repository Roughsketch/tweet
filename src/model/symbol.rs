use serde_derive::{Deserialize, Serialize};

/// Represents a $cashtag included in a tweet body
#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    /// Offets within the tweet text where the symbol begins and ends
    pub indices: Vec<u32>,
    /// Text of the symbol itself
    pub text: String,
}