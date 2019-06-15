use serde_derive::{Deserialize, Serialize};

/// Represents a specific named location corresponding with geo coordinates
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    pub coordinates: Vec<Vec<(f64, f64)>>,
}