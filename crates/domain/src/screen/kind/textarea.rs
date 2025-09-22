use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
};

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
    fn accepts(&self, condition: &Condition) -> Result<()> {
        use Error::*;

        match condition {
            Condition::Text(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn check(&self, answer: &Answer) -> Result<()> {
        Ok(())
    }

    fn evaluate(&self, answer: &Answer) -> Result<()> {
        Ok(())
    }
}
