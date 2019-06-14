use serde_derive::Deserialize;

#[derive(Debug, Eq, PartialEq)]
pub enum MediaType {
    Photo,
    Gif,
    Video,
    Unknown(String),
}

/// Represents media that is associated with the tweet.
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<u32>,
    pub duration_millis: Option<u32>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub bitrate: Option<u32>,
    pub content_type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AdditionalMediaInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub embeddable: Option<bool>,
    pub monetizable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Sizes {
    pub thumb: Size,
    pub large: Size,
    pub medium: Size,
    pub small: Size,
}

#[derive(Debug, Deserialize)]
pub struct Size {
    pub w: u32,
    pub h: u32,
    pub resize: String,
}

impl serde::Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(match *self {
            MediaType::Photo => "photo",
            MediaType::Gif => "gif",
            MediaType::Video => "video",
            MediaType::Unknown(ref other) => other,
        })
    }
}

impl<'de> serde::Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "photo" => MediaType::Photo,
            "gif" => MediaType::Gif,
            "video" => MediaType::Video,
            _ => MediaType::Unknown(s),
        })
    }
}