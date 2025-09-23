use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, CheckAnswerResult, Condition,
    Screenable, ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TextAreaScreen {
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub required: bool,
}

impl TextAreaScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for TextAreaScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Text(_) => Ok(()),
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
