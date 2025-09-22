use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct RadioScreen {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
    pub required: bool,
}

impl RadioScreen {
    pub fn required(&self) -> bool {
        return self.required;
    }
}

impl Screenable for RadioScreen {
    fn accepts(&self, condition: &Condition) -> Result<()> {
        use Error::*;

        match condition {
            Condition::Radio(_) => Ok(()),
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
