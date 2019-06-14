use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Symbol {
    pub indices: Vec<u32>,
    pub text: String,
}