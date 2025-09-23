use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

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

impl Conditionable for TextCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use EvaluateAnswerError::*;
        use TextCondition::*;

        match (self, answer) {
            (Equals(expected), Value(Text(actual))) => Ok(actual == expected),
            (NotEquals(expected), Value(Text(actual))) => Ok(actual != expected),
            (Contains(substr), Value(Text(actual))) => Ok(actual.contains(substr)),
            (NotContains(substr), Value(Text(actual))) => Ok(!actual.contains(substr)),
            (StartsWith(prefix), Value(Text(actual))) => Ok(actual.starts_with(prefix)),
            (EndsWith(suffix), Value(Text(actual))) => Ok(actual.ends_with(suffix)),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
