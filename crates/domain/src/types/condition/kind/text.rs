use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum TextCondition {
    Equals(String),
    NotEquals(String),
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
}

impl TextCondition {
    pub fn evaluate(&self, v: &String) -> bool {
        match self {
            TextCondition::Equals(s) => v == s,
            TextCondition::NotEquals(s) => v != s,
            TextCondition::Contains(s) => v.contains(s),
            TextCondition::NotContains(s) => !v.contains(s),
            TextCondition::StartsWith(s) => v.starts_with(s),
            TextCondition::EndsWith(s) => v.ends_with(s),
        }
    }
}
