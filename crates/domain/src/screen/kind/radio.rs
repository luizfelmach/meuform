use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RadioScreen {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
    pub required: bool,
}

impl Screenable for RadioScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Radio(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use ValidateAnswerError::*;

        match answer {
            Value(Text(_)) => Ok(()),
            Empty => Ok(()),
            _ => Err(IncompatibleAnswerType),
        }
    }

    fn check(&self, answer: &Answer) -> CheckAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use CheckAnswerError::*;

        match answer {
            Empty if self.required => Err(Required),

            Value(Text(choice)) => {
                if !self.options.contains(choice) {
                    return Err(InvalidOption(choice.clone()));
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
