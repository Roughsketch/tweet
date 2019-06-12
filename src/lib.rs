use serde_derive::Deserialize;

use std::str::FromStr;

impl FromStr for Tweet {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub id: u64,
    pub id_str: String,
    pub text: String,
    pub source: String,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    pub user: User,
    pub coordinates: Option<Coordinates>,
    pub place: Option<Place>,
    pub quoted_status_id: Option<u64>,
    pub quoted_status_id_str: Option<String>,
    pub is_quote_status: bool,
    pub quoted_status: Option<Box<Tweet>>,
    pub retweeted_status: Option<Box<Tweet>>,
    pub quote_count: Option<u32>,
    pub reply_count: u32,
    pub retweet_count: u32,
    pub favorite_count: Option<u32>,
    pub entities: Option<Entity>,
    pub extended_entities: Option<ExtendedEntity>,
    pub favorited: Option<bool>,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: String,
    pub lang: Option<String>,
    //  matching_rules: Vec<Rule>,
    //  current_user_retweet: 
    //  scopes
    pub withheld_copyright: Option<bool>,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
}

impl Tweet {
    /// Determine whether Twitter thinks this post is sensitive or not.
    /// For tweets that do not have this attribute, the default return
    /// value is false.
    pub fn is_sensitive(&self) -> bool {
        self.possibly_sensitive.unwrap_or(false)
    }

    /// Determine whether this is a retweet or not.
    pub fn is_retweet(&self) -> bool {
        self.retweeted_status.is_some()
    }

    /// Returns true when the tweet has a photo, gif, or video.
    pub fn has_media(&self) -> bool {
        self.extended_entities.is_some()
    }

    /// If it's a retweet, the original tweet id is returned. Otherwise
    /// the current tweet id is returned.
    pub fn base_id(&self) -> u64 {
        if let Some(rt) = &self.retweeted_status {
            return rt.id;
        }

        return self.id;
    }

    /// Gathers all media urls from the post into a `Vec`.
    /// For videos and gifs this will always have a single
    /// url, but for photos it can be up to 4 max.
    pub fn media_urls(&self) -> Vec<String> {
        let mut urls = Vec::with_capacity(4);

        //  Use extended entities to get media
        if let Some(ent) = &self.extended_entities {
            for media in &ent.media {
                //  If it's a photo, just take the url
                if media.kind == "photo" {
                    urls.push(media.media_url_https.clone());
                } else {
                    //  If it's a video, get the max bitrate variant's url
                    if let Some(vi) = &media.video_info {
                        let max = vi.variants.iter().max_by(|a, b| a.bitrate.cmp(&b.bitrate));

                        if let Some(var) = max {
                            urls.push(var.url.clone());
                        }
                    }
                }
            }
        }

        urls
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub location: Option<String>,
    //  derived: Vec<Enrichment>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: u32,
    pub friends_count: u32,
    pub listed_count: u32,
    pub favourites_count: u32,
    pub statuses_count: u32,
    pub created_at: String,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    pub id: String,
    pub url: String,
    pub place_type: String,
    pub name: String,
    pub full_name: String,
    pub country_code: String,
    pub country: String,
    pub bounding_box: BoundingBox,
    //  attributes: No docs?
}

#[derive(Debug, Deserialize)]
pub struct BoundingBox {
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub hashtags: Vec<Hashtag>,
    pub urls: Vec<Url>,
    pub user_mentions: Vec<UserMention>,
    pub symbols: Vec<Symbol>,
    pub media: Option<Vec<Media>>,
    pub polls: Option<Vec<Poll>>,
}

#[derive(Debug, Deserialize)]
pub struct Hashtag {
    pub indices: Vec<u32>,
    pub text: String,
}

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
    #[serde(rename="type")]
    pub kind: String,
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

#[derive(Debug, Deserialize)]
pub struct UserMention {
    pub id: Option<u64>,
    pub id_str: Option<String>,
    pub indices: Vec<u32>,
    pub name: Option<String>,
    pub screen_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Symbol {
    pub indices: Vec<u32>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    pub options: Vec<PollOption>,
    pub end_datetime: String,
    pub duration_minutes: String,
}

#[derive(Debug, Deserialize)]
pub struct PollOption {
    pub position: u32,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedEntity {
    pub media: Vec<Media>,
}