use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum TextAreaCondition {
    Equals(String),
    NotEquals(String),
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
}

impl TextAreaCondition {
    pub fn evaluate(&self, v: &String) -> bool {
        match self {
            TextAreaCondition::Equals(s) => v == s,
            TextAreaCondition::NotEquals(s) => v != s,
            TextAreaCondition::Contains(s) => v.contains(s),
            TextAreaCondition::NotContains(s) => !v.contains(s),
            TextAreaCondition::StartsWith(s) => v.starts_with(s),
            TextAreaCondition::EndsWith(s) => v.ends_with(s),
        }
    }
}
