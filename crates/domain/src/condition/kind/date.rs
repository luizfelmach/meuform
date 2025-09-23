use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

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

impl Conditionable for DateCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use DateCondition::*;
        use EvaluateAnswerError::*;

        match (self, answer) {
            (Equals(expected), Value(Date(actual))) => Ok(actual == expected),
            (NotEquals(expected), Value(Date(actual))) => Ok(actual != expected),
            (GreaterThan(expected), Value(Date(actual))) => Ok(actual > expected),
            (LessThan(expected), Value(Date(actual))) => Ok(actual < expected),
            (GreaterOrEqual(expected), Value(Date(actual))) => Ok(actual >= expected),
            (LessOrEqual(expected), Value(Date(actual))) => Ok(actual <= expected),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
