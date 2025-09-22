use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
};

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
    fn accepts(&self, condition: &Condition) -> Result<()> {
        use Error::*;

        match condition {
            Condition::Checkbox(_) => Ok(()),
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
