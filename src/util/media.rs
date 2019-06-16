use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::MediaType;

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            MediaType::Photo => "photo",
            MediaType::Gif => "gif",
            MediaType::Video => "video",
            MediaType::Unknown(ref other) => other,
        })
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
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