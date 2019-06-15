use serde_derive::{Deserialize, Serialize};

/// Represents geographic coordinates
#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    /// Latitude and Longitude
    pub coordinates: (f64, f64),
    /// Type of data encoded in coordinates
    #[serde(rename = "type")]
    pub kind: String,
}