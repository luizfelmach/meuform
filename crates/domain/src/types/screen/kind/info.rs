use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
};

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
    fn accepts(&self, _condition: &Condition) -> Result<()> {
        use Error::*;

        Err(InfoScreenConditionNotAllowed)
    }

    fn check(&self, answer: &Answer) -> Result<()> {
        Ok(())
    }

    fn evaluate(&self, answer: &Answer) -> Result<()> {
        Ok(())
    }
}
