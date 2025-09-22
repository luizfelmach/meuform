use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionItem {
    pub value: String,
    pub label: Option<String>,
}
