use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerError, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoScreen {
    pub title: String,
    pub description: Option<String>,
}

impl Screenable for InfoScreen {
    fn accepts(&self, _condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        Err(InfoScreenConditionNotAllowed)
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use ValidateAnswerError::*;

        match answer {
            Empty => Ok(()),
            _ => Err(IncompatibleAnswerType),
        }
    }

    fn check(&self, _answer: &Answer) -> CheckAnswerResult<()> {
        Ok(())
    }
}
