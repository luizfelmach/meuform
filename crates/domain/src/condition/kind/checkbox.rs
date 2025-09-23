use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum CheckboxCondition {
    Equals(Vec<String>),
}

impl Conditionable for CheckboxCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use CheckboxCondition::*;
        use EvaluateAnswerError::*;

        match (self, answer) {
            (Equals(expected), Value(TextList(actual))) => Ok(actual == expected),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
