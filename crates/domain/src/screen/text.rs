use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextScreen {
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
}
