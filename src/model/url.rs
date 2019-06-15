use serde_derive::Deserialize;

/// Represents a link from a tweet
#[derive(Debug, Deserialize)]
pub struct Url {
    pub display_url: String,
    pub expanded_url: String,
    pub indices: Vec<u32>,
    pub url: String,
    pub unwound: Option<UnwoundUrl>
}

#[derive(Debug, Deserialize)]
pub struct UnwoundUrl {
    pub url: String,
    pub status: u32,
    pub title: String,
    pub description: String,
}

/// Represents a link from a tweet
#[derive(Debug, Deserialize)]
pub struct LegacyUrl {
    pub display: String,
    pub expanded: String,
    pub url: String,
}