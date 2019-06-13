use serde_derive::Deserialize;

use std::str::FromStr;

impl FromStr for Tweet {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// Represents a tweet
#[derive(Debug, Deserialize)]
pub struct Tweet {
    /// When the tweet was posted
    pub created_at: String,
    /// The unique id for the tweet
    pub id: u64,
    /// String version of `id`
    pub id_str: String,
    /// The possibly truncated text of the tweet
    pub text: String,
    /// The client that posted the tweet
    pub source: String,
    /// Whether or not the text field is truncated to 140 characters
    pub truncated: bool,
    /// If this tweet is a reply, this will contain the original tweet id
    pub in_reply_to_status_id: Option<u64>,
    /// Same as `in_reply_to_status_id`, but a String
    pub in_reply_to_status_id_str: Option<String>,
    /// If this tweet is a reply, this will contain the original author id
    pub in_reply_to_user_id: Option<u64>,
    /// Same as `in_reply_to_user_id`, but a String
    pub in_reply_to_user_id_str: Option<String>,
    /// If this tweet is a reply, contains the original user's screen name
    pub in_reply_to_screen_name: Option<String>,
    /// The user who posted this tweet
    pub user: User,
    /// If the tweet was truncated because it was longer than 140 chars, this contains the rest of the text
    pub extended_tweet: Option<ExtendedTweet>,
    /// Represents the geographic location of this tweet as reported by user/client
    pub coordinates: Option<Coordinates>,
    /// The place that this tweet is associated with
    pub place: Option<Place>,
    /// If this tweet is a quote, it contains the of the quoted tweet id
    pub quoted_status_id: Option<u64>,
    /// Same as `quoted_status_id_str`, but a String
    pub quoted_status_id_str: Option<String>,
    /// Whether this is a quoted tweet or not
    pub is_quote_status: bool,
    /// When the tweet is a quote tweet, this contains the quoted tweet
    pub quoted_status: Option<Box<Tweet>>,
    /// When the tweet is a retweet, this contains the retweeted tweet
    pub retweeted_status: Option<Box<Tweet>>,
    /// Approximate count of times tweet was quoted
    pub quote_count: Option<u32>,
    /// How many times this tweet has been replied to
    pub reply_count: u32,
    /// How many times this tweet has been retweeted
    pub retweet_count: u32,
    /// How many times this tweet has been favorited
    pub favorite_count: Option<u32>,
    /// Entities that have been parsed from the tweet
    pub entities: Option<Entity>,
    /// If there are media entities, this field contains them all
    pub extended_entities: Option<ExtendedEntity>,
    /// Whether the authenticated user favorited this tweet
    pub favorited: Option<bool>,
    /// Whether the authenticated user retweeted this tweet
    pub retweeted: bool,
    /// Whether a link in this tweet (including media) is potentially sensitive
    pub possibly_sensitive: Option<bool>,
    /// What filter level is associated with this tweet. Can be none, low, or medium.
    pub filter_level: String,
    /// BCP 47 language identifier corresponding to machine-detected language of tweet
    pub lang: Option<String>,
    //  matching_rules: Vec<Rule>,
    //  current_user_retweet: 
    //  scopes
    /// Indicates whether content was removed via DMCA
    pub withheld_copyright: Option<bool>,
    /// Indicates what countries this tweet is unavailable
    pub withheld_in_countries: Option<Vec<String>>,
    /// Indicates whether content is being withheld because of "status" or "user"
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

    /// Determine whether the tweet is extended or not
    pub fn is_extended(&self) -> bool {
        self.extended_tweet.is_some()
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

    /// Creates a direct URL to the status
    pub fn url(&self) -> String {
        format!("https://twitter.com/{}/status/{}", self.user.screen_name, self.id)
    }

    pub fn full_text(&self) -> String {
        if let Some(ex) = &self.extended_tweet {
            ex.full_text.clone()
        } else if self.truncated {
            //  If truncated, doesn't have extended tweet, and has a retweet
            //  then take the text from the retweet status
            match &self.retweeted_status {
                Some(rt) => rt.text.clone(),
                None => self.text.clone(),
            }
        } else {
            self.text.clone()
        }
    }

    /// Gathers all media urls from the post into a `Vec`.
    /// For videos and gifs this will always have a single
    /// url, but for photos it can be up to 4 max.
    pub fn media_urls(&self) -> Vec<String> {
        use std::collections::HashSet;

        let mut urls = if let Some(rt) = &self.retweeted_status {
            rt.media_urls().iter().cloned().collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        //  Use extended entities to get media
        if let Some(ent) = &self.extended_entities {
            for media in &ent.media {
                //  If it's a photo, just take the url
                if media.kind == "photo" {
                    urls.insert(media.media_url_https.clone());
                } else {
                    //  If it's a video, get the max bitrate variant's url
                    if let Some(vi) = &media.video_info {
                        let max = vi.variants.iter().max_by(|a, b| a.bitrate.cmp(&b.bitrate));

                        if let Some(var) = max {
                            urls.insert(var.url.clone());
                        }
                    }
                }
            }
        }

        urls.iter().cloned().collect::<Vec<_>>()
    }

    /// Gets a list of hashtags associated with the tweet
    pub fn hashtags(&self) -> Vec<String> {
        match &self.entities {
            Some(entities) => {
                entities.hashtags
                        .iter()
                        .map(|ht| ht.text.clone())
                        .collect::<Vec<_>>()
            }
            None => Vec::new(),
        }
    }
}

/// Represents a twitter user
#[derive(Debug, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: u64,
    /// Same as `id`, but a String
    pub id_str: String,
    /// The user's display name
    pub name: String,
    /// The user's handle
    pub screen_name: String,
    /// The user-defined location of the user
    pub location: Option<String>,
    //  derived: Vec<Enrichment>,
    /// User-provided URL associated with their profile
    pub url: Option<String>,
    /// User-provided description of their profile
    pub description: Option<String>,
    /// Whether this user's tweets are protected or not
    pub protected: bool,
    /// Whether this user is verified or not
    pub verified: bool,
    /// How many followers this user has
    pub followers_count: u32,
    /// How many friends this user has
    pub friends_count: u32,
    /// How many public lists this user is a member of
    pub listed_count: u32,
    /// How many favorites this user has
    pub favourites_count: u32,
    /// How many tweets and retweets this user has
    pub statuses_count: u32,
    /// When the account was created
    pub created_at: String,
    /// User's uploaded profile banner
    pub profile_banner_url: Option<String>,
    /// User's uploaded profile image
    pub profile_image_url_https: String,
    /// When true, indicates user has not changed theme or background of their profile
    pub default_profile: bool,
    /// Whether or not the user has a default profile image or not
    pub default_profile_image: bool,
    /// Which countries this user is blocked in
    pub withheld_in_countries: Option<Vec<String>>,
    /// Indicates whether content being withheld is a "user"
    pub withheld_scope: Option<String>,
}

/// Represents a full tweet text and entities when going over 140 characters
#[derive(Debug, Deserialize)]
pub struct ExtendedTweet {
    full_text: String,
    display_text_range: (u32, u32),
    entities: Entity,
}

/// Represents geographic coordinates
#[derive(Debug, Deserialize)]
pub struct Coordinates {
    /// Latitude and Longitude
    pub coordinates: (f64, f64),
    /// Type of data encoded in coordinates
    #[serde(rename = "type")]
    pub kind: String,
}

/// Represents a specific named location corresponding with geo coordinates
#[derive(Debug, Deserialize)]
pub struct Place {
    /// Unique id representing this place
    pub id: String,
    /// URL representing location of additional place metadata
    pub url: String,
    /// Type of location represented by this place
    pub place_type: String,
    /// Short, human-readable place name
    pub name: String,
    /// Full, human-readable place name
    pub full_name: String,
    /// Country code that this place is located in
    pub country_code: String,
    /// Country that this place is located in
    pub country: String,
    /// A bounding box containing the coordinates which enclose this place
    pub bounding_box: BoundingBox,
    //  attributes: No docs?
}

/// Represents a bounding box of geo coordinates
#[derive(Debug, Deserialize)]
pub struct BoundingBox {
    pub coordinates: Vec<Vec<(f64, f64)>>,
}

/// Contains information on various parsed out pieces of tweets
#[derive(Debug, Deserialize)]
pub struct Entity {
    /// Collection of hashtags that were included in this tweet
    pub hashtags: Vec<Hashtag>,
    /// List of links that were included in this tweet
    pub urls: Vec<Url>,
    /// List of users mentioned in this tweet
    pub user_mentions: Vec<UserMention>,
    /// List of symbols in this tweet
    pub symbols: Vec<Symbol>,
    /// List of any media added to this tweet
    pub media: Option<Vec<Media>>,
    /// List of any polls added to this tweet
    pub polls: Option<Vec<Poll>>,
}

/// Represents a hashtag
#[derive(Debug, Deserialize)]
pub struct Hashtag {
    /// The indices of the hastag in the original tweet message
    pub indices: Vec<u32>,
    /// The text of the hashtag
    pub text: String,
}

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

///
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