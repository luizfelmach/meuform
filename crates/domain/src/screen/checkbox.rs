use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CheckboxScreen {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
    pub min_selections: Option<u32>,
    pub max_selections: Option<u32>,
    pub required: bool,
}
