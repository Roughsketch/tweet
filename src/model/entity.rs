use serde_derive::{Deserialize, Serialize};

use crate::model::hashtag::Hashtag;
use crate::model::media::Media;
use crate::model::poll::Poll;
use crate::model::symbol::Symbol;
use crate::model::url::Url;
use crate::model::user_mention::UserMention;

/// Contains information on various parsed out pieces of tweets
#[derive(Debug, Deserialize, Serialize)]
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

/// When a tweet has more than one image or has a video or gif,
/// the full media information is sent through an extended entity.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedEntity {
    /// The media inside the tweet
    pub media: Vec<Media>,
}