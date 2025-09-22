use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RadioScreen {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
    pub required: bool,
}
