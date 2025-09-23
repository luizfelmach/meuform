use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CheckboxScreen {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
    pub min_selections: Option<u32>,
    pub max_selections: Option<u32>,
    pub required: bool,
}

impl CheckboxScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for CheckboxScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Checkbox(_) => Ok(()),
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
