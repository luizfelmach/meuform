use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub fn evaluate(&self, v: &DateTime<Utc>) -> bool {
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
