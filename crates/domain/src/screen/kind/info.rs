use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoScreen {
    pub title: String,
    pub description: Option<String>,
}

impl InfoScreen {
    pub fn required(&self) -> bool {
        return false;
    }
}

impl Screenable for InfoScreen {
    fn accepts(&self, _condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        Err(InfoScreenConditionNotAllowed)
    }

    fn validate(&self, _answer: &Answer) -> ValidateAnswerResult<()> {
        Ok(())
    }

    fn check(&self, _answer: &Answer) -> CheckAnswerResult<()> {
        Ok(())
    }
}
