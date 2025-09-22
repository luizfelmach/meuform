use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum AnswerValue {
    Text(String),
    TextList(Vec<String>),
    Number(f64),
    Boolean(bool),
    Date(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Answer {
    Value(AnswerValue),
    Empty,
}
