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

impl From<AnswerValue> for Answer {
    fn from(value: AnswerValue) -> Self {
        Answer::Value(value)
    }
}

impl From<String> for AnswerValue {
    fn from(s: String) -> Self {
        AnswerValue::Text(s)
    }
}

impl From<Vec<String>> for AnswerValue {
    fn from(list: Vec<String>) -> Self {
        AnswerValue::TextList(list)
    }
}

impl From<f64> for AnswerValue {
    fn from(n: f64) -> Self {
        AnswerValue::Number(n)
    }
}

impl From<bool> for AnswerValue {
    fn from(b: bool) -> Self {
        AnswerValue::Boolean(b)
    }
}

impl From<DateTime<Utc>> for AnswerValue {
    fn from(d: DateTime<Utc>) -> Self {
        AnswerValue::Date(d)
    }
}

impl From<&str> for AnswerValue {
    fn from(s: &str) -> Self {
        AnswerValue::Text(s.to_string())
    }
}

impl From<i32> for AnswerValue {
    fn from(n: i32) -> Self {
        AnswerValue::Number(n as f64)
    }
}

impl From<i64> for AnswerValue {
    fn from(n: i64) -> Self {
        AnswerValue::Number(n as f64)
    }
}

impl From<u32> for AnswerValue {
    fn from(n: u32) -> Self {
        AnswerValue::Number(n as f64)
    }
}

impl From<f32> for AnswerValue {
    fn from(n: f32) -> Self {
        AnswerValue::Number(n as f64)
    }
}
