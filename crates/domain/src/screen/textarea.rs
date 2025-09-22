use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextAreaScreen {
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
}
