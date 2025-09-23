use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DateScreen {
    pub title: String,
    pub description: Option<String>,
    pub min_date: Option<DateTime<Utc>>,
    pub max_date: Option<DateTime<Utc>>,
    pub required: bool,
}

impl Screenable for DateScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Date(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use ValidateAnswerError::*;

        match answer {
            Value(Date(_)) => Ok(()),
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

            Value(Date(got)) => {
                if let Some(min) = self.min_date {
                    if got < &min {
                        return Err(DateTooEarly {
                            min,
                            got: got.clone(),
                        });
                    }
                }

                if let Some(max) = self.max_date {
                    if got > &max {
                        return Err(DateTooLate {
                            max,
                            got: got.clone(),
                        });
                    }
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
