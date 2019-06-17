use serde_derive::{Deserialize, Serialize};

/// Represents a link from a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct Url {
    pub display_url: String,
    pub expanded_url: String,
    pub indices: Vec<u32>,
    pub url: String,
    pub unwound: Option<UnwoundUrl>
}

/// Holds an unshortened link and some metadata about
/// its content.
#[derive(Debug, Deserialize, Serialize)]
pub struct UnwoundUrl {
    /// Long url of a shortened one contained in the tweet
    pub url: String,
    /// HTTP status of the url
    pub status: u32,
    /// Title of the destination
    pub title: String,
    /// Description of the destination
    pub description: String,
}

/// Represents a link from a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyUrl {
    pub display: String,
    pub expanded: String,
    pub url: String,
}