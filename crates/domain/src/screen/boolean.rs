use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BooleanScreen {
    pub title: String,
    pub description: Option<String>,
    pub true_label: Option<String>,
    pub false_label: Option<String>,
    pub required: bool,
}
