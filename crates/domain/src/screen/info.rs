use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoScreen {
    pub title: String,
    pub description: Option<String>,
}
