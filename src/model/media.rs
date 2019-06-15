use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub enum MediaType {
    Photo,
    Gif,
    Video,
    Unknown(String),
}

/// Represents media that is associated with the tweet.
#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub display_url: String,
    pub expanded_url: String,
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u32>,
    pub media_url: String,
    pub media_url_https: String,
    pub sizes: Sizes,
    pub source_status_id: Option<u64>,
    pub source_status_id_str: Option<String>,
    /// The type of media
    #[serde(rename="type")]
    pub kind: MediaType,
    pub url: String,
    pub video_info: Option<VideoInfo>,
    pub additional_media_info: Option<AdditionalMediaInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<u32>,
    pub duration_millis: Option<u32>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Variant {
    pub bitrate: Option<u32>,
    pub content_type: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalMediaInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub embeddable: Option<bool>,
    pub monetizable: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sizes {
    pub thumb: Size,
    pub large: Size,
    pub medium: Size,
    pub small: Size,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Size {
    pub w: u32,
    pub h: u32,
    pub resize: String,
}