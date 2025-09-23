use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
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

impl NumberScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for NumberScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Number(_) => Ok(()),
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
