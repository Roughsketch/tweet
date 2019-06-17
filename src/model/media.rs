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

impl Media {
    /// Returns the URL associated with this media object
    /// 
    /// This should ideally only return None if Twitter
    /// changes its API in the future.
    pub fn url(&self) -> Option<String> {
        //  If it's a photo, just take the url
        if self.kind == MediaType::Photo {
            return Some(self.media_url_https.clone());
        } 
        
        if let Some(vi) = &self.video_info {
            //  If it's a video, get the max bitrate variant's url
            let max = vi.variants.iter().max_by(|a, b| a.bitrate.cmp(&b.bitrate));

            if let Some(var) = max {
                return Some(var.url.clone());
            }
        }

        return None;
    }
}

/// Stores information about a video media object
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfo {
    /// What aspect ratio the video has
    pub aspect_ratio: (u32, u32),
    /// How long the video lasts in milliseconds
    pub duration_millis: Option<u32>,
    /// A list of quality variants available for this video
    pub variants: Vec<Variant>,
}

/// Holds information about a videos quality and location
#[derive(Debug, Deserialize, Serialize)]
pub struct Variant {
    /// What bitrate the video has. The higher the bitrate,
    /// the higher the quality is.
    pub bitrate: Option<u32>,
    /// What type of content this variant contains
    pub content_type: String,
    /// The location of the video
    pub url: String,
}

/// Holds additional information about a piece of media
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalMediaInfo {
    /// Title of the content
    pub title: Option<String>,
    /// A description of the content
    pub description: Option<String>,
    /// Whether or not this content can be embedded
    pub embeddable: Option<bool>,
    /// Whether or not this content can be monetized
    pub monetizable: bool,
}

/// Stores information about the various sizes available
/// for a piece of media in some standard forms.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sizes {
    /// Thumbnail size information
    pub thumb: Size,
    /// Largest available size information
    pub large: Size,
    /// Medium size information
    pub medium: Size,
    /// Small size information
    pub small: Size,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Size {
    /// Width
    pub w: u32,
    /// Height
    pub h: u32,
    /// The resizing method used to get these values
    pub resize: String,
}