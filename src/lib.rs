use serde_derive::Deserialize;

use std::str::FromStr;

impl FromStr for Tweet {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
struct Tweet {
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u64>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    coordinates: Option<Coordinates>,
    place: Option<Place>,
    quoted_status_id: Option<u64>,
    quoted_status_id_str: Option<String>,
    is_quote_status: bool,
    quoted_status: Box<Tweet>,
    retweeted_status: Box<Tweet>,
    quote_count: Option<u32>,
    reply_count: u32,
    retweet_count: u32,
    favorite_count: Option<u32>,
    entities: Option<Entity>,
    extended_entities: Option<ExtendedEntity>,
    favorited: Option<bool>,
    retweeted: bool,
    possibly_sensitive: Option<bool>,
    filter_level: String,
    lang: Option<String>,
    //  matching_rules: Vec<Rule>,
    //  current_user_retweet: 
    //  scopes
    withheld_copyright: Option<bool>,
    withheld_in_countries: Option<Vec<String>>,
    withheld_scope: Option<String>,
}

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    id_str: String,
    name: String,
    screen_name: String,
    location: Option<String>,
    //  derived: Vec<Enrichment>,
    url: Option<String>,
    description: Option<String>,
    protected: bool,
    verified: bool,
    followers_count: u32,
    friends_count: u32,
    listed_count: u32,
    favourites_count: u32,
    statuses_count: u32,
    created_at: String,
    profile_banner_url: String,
    profile_banner_url_https: String,
    default_profile: bool,
    default_profile_image: bool,
    withheld_in_countries: Vec<String>,
    withheld_scope: String,
}

#[derive(Debug, Deserialize)]
struct Coordinates {
    coordinates: Vec<f64>,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct Place {
    id: String,
    url: String,
    place_type: String,
    name: String,
    full_name: String,
    country_code: String,
    country: String,
    bounding_box: BoundingBox,
    //  attributes: No docs?
}

#[derive(Debug, Deserialize)]
struct BoundingBox {
    coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Debug, Deserialize)]
struct Entity {
    hashtags: Vec<Hashtag>,
    urls: Vec<Url>,
    user_mentions: Vec<UserMention>,
    symbols: Vec<Symbol>,
    media: Option<Vec<Media>>,
    polls: Option<Vec<Poll>>,
}

#[derive(Debug, Deserialize)]
struct Hashtag {
    indices: Vec<u32>,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Media {
    display_url: String,
    expanded_url: String,
    id: u64,
    id_str: String,
    indices: Vec<u32>,
    media_url: String,
    media_url_https: String,
    sizes: Sizes,
    source_status_id: Option<u64>,
    source_status_id_str: Option<String>,
    #[serde(rename="type")]
    kind: String,
    url: String,
    video_info: Option<VideoInfo>,
    additional_media_info: Option<AdditionalMediaInfo>,
}

#[derive(Debug, Deserialize)]
struct VideoInfo {
    aspect_ratio: Vec<u32>,
    duration_millis: u32,
    variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
struct Variant {
    bitrate: Option<u32>,
    content_type: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct AdditionalMediaInfo {
    title: String,
    description: String,
    embeddable: bool,
    monetizable: bool,
}

#[derive(Debug, Deserialize)]
struct Sizes {
    thumb: Size,
    large: Size,
    medium: Size,
    small: Size,
}

#[derive(Debug, Deserialize)]
struct Size {
    w: u32,
    h: u32,
    resize: String,
}

#[derive(Debug, Deserialize)]
struct Url {
    display_url: String,
    expanded_url: String,
    indices: Vec<u32>,
    url: String,
    unwound: Option<UnwoundUrl>
}

#[derive(Debug, Deserialize)]
struct UnwoundUrl {
    url: String,
    status: u32,
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UserMention {
    id: u64,
    id_str: String,
    indices: Vec<u32>,
    name: String,
    screen_name: String,
}

#[derive(Debug, Deserialize)]
struct Symbol {
    indices: Vec<u32>,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Poll {
    options: Vec<PollOption>,
    end_datetime: String,
    duration_minutes: String,
}

#[derive(Debug, Deserialize)]
struct PollOption {
    position: u32,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ExtendedEntity {
    media: Vec<Media>,
}