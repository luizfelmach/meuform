use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BooleanScreen {
    pub title: String,
    pub description: Option<String>,
    pub true_label: Option<String>,
    pub false_label: Option<String>,
    pub required: bool,
}

impl Screenable for BooleanScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Boolean(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use ValidateAnswerError::*;

        match answer {
            Value(Boolean(_)) => Ok(()),
            Empty => Ok(()),
            _ => Err(IncompatibleAnswerType),
        }
    }

    fn check(&self, answer: &Answer) -> CheckAnswerResult<()> {
        use Answer::*;
        use CheckAnswerError::*;

        match answer {
            Empty if self.required => Err(Required),
            _ => Ok(()),
        }
    }
}
