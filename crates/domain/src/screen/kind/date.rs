use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
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

impl DateScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for DateScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Date(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, _answer: &Answer) -> ValidateAnswerResult<()> {
        Ok(())
    }

    fn check(&self, _answer: &Answer) -> CheckAnswerResult<()> {
        Ok(())
    }
}
