use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    pub indices: Vec<u32>,
    pub text: String,
}