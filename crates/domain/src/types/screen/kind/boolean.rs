use crate::{
    answer::Answer,
    condition::Condition,
    screen::{Error, Result, Screenable},
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
    fn accepts(&self, condition: &Condition) -> Result<()> {
        use Error::*;

        match condition {
            Condition::Boolean(_) => Ok(()),
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
