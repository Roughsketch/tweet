use serde_derive::{Deserialize, Serialize};

/// Represents a @mention of a user
#[derive(Debug, Deserialize, Serialize)]
pub struct UserMention {
    /// Id of the user being mentioned
    pub id: Option<u64>,
    /// Id of the user being mentioned, but a string
    pub id_str: Option<String>,
    /// Indices in the tweet where this mention is located
    pub indices: Vec<u32>,
    /// Name of the user being mentioned
    pub name: Option<String>,
    /// Screen name of the user being mentioned
    pub screen_name: String,
}