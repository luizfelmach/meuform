use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum BooleanCondition {
    Equals(bool),
}

impl Conditionable for BooleanCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use BooleanCondition::*;
        use EvaluateAnswerError::*;

        match (self, answer) {
            (Equals(expected), Value(Boolean(actual))) => Ok(actual == expected),
            (_, Answer::Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
