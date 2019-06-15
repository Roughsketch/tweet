use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::util::datetime::{datefmt_de, datefmt_ser};

/// Represents a twitter user
#[derive(Debug, Deserialize, Serialize)]
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
    #[serde(deserialize_with="datefmt_de", serialize_with="datefmt_ser")]
    pub created_at: DateTime<Utc>,
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