use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

use serde::{Deserialize, Serialize};

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

impl Conditionable for NumberCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use EvaluateAnswerError::*;
        use NumberCondition::*;

        match (self, answer) {
            (Equals(expected), Value(Number(actual))) => {
                Ok((actual - expected).abs() < f64::EPSILON)
            }
            (NotEquals(expected), Value(Number(actual))) => {
                Ok((actual - expected).abs() >= f64::EPSILON)
            }
            (GreaterThan(expected), Value(Number(actual))) => Ok(actual > expected),
            (LessThan(expected), Value(Number(actual))) => Ok(actual < expected),
            (GreaterOrEqual(expected), Value(Number(actual))) => Ok(actual >= expected),
            (LessOrEqual(expected), Value(Number(actual))) => Ok(actual <= expected),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
