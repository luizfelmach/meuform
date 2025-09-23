use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextScreen {
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
}

impl TextScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for TextScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Text(_) => Ok(()),
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

            Value(Text(s)) => {
                let got = s.chars().count() as u32;

                if let Some(min) = self.min_length {
                    if got < min {
                        return Err(TextTooShort { min, got });
                    }
                }

                if let Some(max) = self.max_length {
                    if got > max {
                        return Err(TextTooLong { max, got });
                    }
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
