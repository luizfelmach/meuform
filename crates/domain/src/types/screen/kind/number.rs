use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
};

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
    fn accepts(&self, condition: &Condition) -> Result<()> {
        use Error::*;

        match condition {
            Condition::Number(_) => Ok(()),
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
