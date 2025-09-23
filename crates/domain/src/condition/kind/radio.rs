use crate::{Answer, AnswerValue, Conditionable, EvaluateAnswerError, EvaluateAnswerResult};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum RadioCondition {
    InText(Vec<String>),
    NotInText(Vec<String>),
}

impl Conditionable for RadioCondition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Answer::*;
        use AnswerValue::*;
        use EvaluateAnswerError::*;
        use RadioCondition::*;

        match (self, answer) {
            (InText(options), Value(Text(actual))) => Ok(options.contains(actual)),
            (NotInText(options), Value(Text(actual))) => Ok(!options.contains(actual)),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
