use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum CheckboxCondition {
    Equals(Vec<String>),
}

impl CheckboxCondition {
    pub fn evaluate(&self, v: &Vec<String>) -> bool {
        match self {
            CheckboxCondition::Equals(c) => v == c,
        }
    }
}
