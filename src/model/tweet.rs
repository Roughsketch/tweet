use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

use crate::model::coordinates::Coordinates;
use crate::model::entity::{Entity, ExtendedEntity};
use crate::model::media::MediaType;
use crate::model::place::Place;
use crate::model::url::LegacyUrl;
use crate::model::user::User;
use crate::util::datetime::{datefmt_de, datefmt_ser};

impl FromStr for Tweet {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// Represents a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    /// When the tweet was posted
    #[serde(deserialize_with="datefmt_de", serialize_with="datefmt_ser")]
    pub created_at: DateTime<Utc>,
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

    #[deprecated(since="0.2.0", note="Deprecated in the Twitter API, but kept here for completion.")]
    pub contributors: Option<String>,
    #[deprecated(since="0.2.0", note="Deprecated in the Twitter API, but kept here for completion.")]
    pub display_text_range: Option<(u32, u32)>,
    #[deprecated(since="0.2.0", note="Deprecated in the Twitter API, but kept here for completion. Use coordinates instead.")]
    pub geo: Option<Coordinates>,
    #[deprecated(since="0.2.0", note="Deprecated in the Twitter API, but kept here for completion.")]
    pub quoted_status_permalink: Option<LegacyUrl>,
    #[deprecated(since="0.2.0", note="Deprecated in the Twitter API, but kept here for completion. Used created_at instead.")]
    pub timestamp_ms: Option<String>,
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

    /// When a tweet is extended it can go over the 140 character limit.
    /// In such cases, the tweet text field is truncated as noted by the
    /// truncated flag. This method will check the extended tweet data
    /// and return either it, the full retweet text if it was truncated
    /// or the original text field depending on what's appropriate.
    pub fn full_text(&self) -> String {
        if let Some(ex) = &self.extended_tweet {
            //  If base tweet is extended, get that text
            ex.full_text.clone()
        } else if let Some(rt) = &self.retweeted_status {
            //  If this is a retweet, get its full text
            rt.full_text().clone()
        } else {
            //  Otherwise return base text
            self.text.clone()
        }
    }

    /// Gathers all media urls from the post into a `Vec`.
    /// For videos and gifs this will always have a single
    /// url, but for photos it can be up to 4 max.
    pub fn media_urls(&self) -> Vec<String> {
        use std::collections::HashSet;

        //  Sometimes the original tweet media and current tweet
        //  media are different. This combines them all into a
        //  set so that we don't miss anything.
        let mut urls = if let Some(rt) = &self.retweeted_status {
            rt.media_urls().iter().cloned().collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        //  Use extended entities to get media
        if let Some(ent) = &self.extended_entities {
            for media in &ent.media {
                //  If it's a photo, just take the url
                if media.kind == MediaType::Photo {
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

/// Represents a full tweet text and entities when going over 140 characters
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedTweet {
    full_text: String,
    display_text_range: (u32, u32),
    entities: Entity,
}