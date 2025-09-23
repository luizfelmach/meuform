use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
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

impl BooleanScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for BooleanScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Boolean(_) => Ok(()),
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
