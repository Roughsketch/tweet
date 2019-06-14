use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserMention {
    pub id: Option<u64>,
    pub id_str: Option<String>,
    pub indices: Vec<u32>,
    pub name: Option<String>,
    pub screen_name: String,
}