use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NumberScreen {
    pub title: String,
    pub description: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub required: bool,
}

impl Screenable for NumberScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Number(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use ValidateAnswerError::*;

        match answer {
            Value(Number(_)) => Ok(()),
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

            Value(Number(got)) => {
                if let Some(min) = self.min {
                    if *got < min {
                        return Err(NumberTooSmall { min, got: *got });
                    }
                }

                if let Some(max) = self.max {
                    if *got > max {
                        return Err(NumberTooLarge { max, got: *got });
                    }
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
