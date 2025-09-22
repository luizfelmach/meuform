use crate::Answer;

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    WrongType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "condition")]
pub enum Condition {
    Text(TextCondition),
    TextArea(TextAreaCondition),
    Number(NumberCondition),
    Radio(RadioCondition),
    Checkbox(CheckboxCondition),
    Date(DateCondition),
    Boolean(BooleanCondition),
}

impl Condition {
    pub fn evaluate(&self, value: &Answer) -> Result<bool> {
        use Answer::Value;
        use Condition::*;

        match (self, value) {
            (Text(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (TextArea(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (Number(cond), Value(AnswerValue::Number(v))) => Ok(cond.evaluate(*v)),
            (Radio(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (Checkbox(cond), Value(AnswerValue::TextList(v))) => Ok(cond.evaluate(v)),
            (Date(cond), Value(AnswerValue::Date(v))) => Ok(cond.evaluate(v)),
            (Boolean(cond), Value(AnswerValue::Boolean(v))) => Ok(cond.evaluate(*v)),
            _ => Err(Error::WrongType),
        }
    }
}

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
    fn evaluate(&self, v: &String) -> bool {
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
    fn evaluate(&self, v: &String) -> bool {
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum NumberCondition {
    Equals(f64),
    NotEquals(f64),
    GreaterThan(f64),
    LessThan(f64),
    GreaterOrEqual(f64),
    LessOrEqual(f64),
}

impl NumberCondition {
    fn evaluate(&self, v: f64) -> bool {
        match self {
            NumberCondition::Equals(n) => v == *n,
            NumberCondition::NotEquals(n) => v != *n,
            NumberCondition::GreaterThan(n) => v > *n,
            NumberCondition::LessThan(n) => v < *n,
            NumberCondition::GreaterOrEqual(n) => v >= *n,
            NumberCondition::LessOrEqual(n) => v <= *n,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum RadioCondition {
    InText(Vec<String>),
    NotInText(Vec<String>),
}

impl RadioCondition {
    fn evaluate(&self, v: &String) -> bool {
        match self {
            RadioCondition::InText(opts) => opts.contains(&v.to_string()),
            RadioCondition::NotInText(opts) => !opts.contains(&v.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum CheckboxCondition {
    Equals(Vec<String>),
}

impl CheckboxCondition {
    fn evaluate(&self, v: &Vec<String>) -> bool {
        match self {
            CheckboxCondition::Equals(c) => v == c,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum DateCondition {
    Equals(DateTime<Utc>),
    NotEquals(DateTime<Utc>),
    GreaterThan(DateTime<Utc>),
    LessThan(DateTime<Utc>),
    GreaterOrEqual(DateTime<Utc>),
    LessOrEqual(DateTime<Utc>),
}

impl DateCondition {
    fn evaluate(&self, v: &DateTime<Utc>) -> bool {
        match self {
            DateCondition::Equals(d) => v == d,
            DateCondition::NotEquals(d) => v != d,
            DateCondition::GreaterThan(d) => v > d,
            DateCondition::LessThan(d) => v < d,
            DateCondition::GreaterOrEqual(d) => v >= d,
            DateCondition::LessOrEqual(d) => v <= d,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum BooleanCondition {
    Equals(bool),
}

impl BooleanCondition {
    fn evaluate(&self, v: bool) -> bool {
        match self {
            BooleanCondition::Equals(b) => v == *b,
        }
    }
}
