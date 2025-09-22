use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum BooleanCondition {
    Equals(bool),
}

impl BooleanCondition {
    pub fn evaluate(&self, v: &bool) -> bool {
        match self {
            BooleanCondition::Equals(b) => v == b,
        }
    }
}
