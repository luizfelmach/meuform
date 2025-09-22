use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum RadioCondition {
    InText(Vec<String>),
    NotInText(Vec<String>),
}

impl RadioCondition {
    pub fn evaluate(&self, v: &String) -> bool {
        match self {
            RadioCondition::InText(opts) => opts.contains(&v.to_string()),
            RadioCondition::NotInText(opts) => !opts.contains(&v.to_string()),
        }
    }
}
