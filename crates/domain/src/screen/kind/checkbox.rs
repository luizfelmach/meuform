use crate::{
    AcceptsConditionError, AcceptsConditionResult, Answer, AnswerValue, CheckAnswerError,
    CheckAnswerResult, Condition, Screenable, ValidateAnswerError, ValidateAnswerResult,
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

impl Screenable for CheckboxScreen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use AcceptsConditionError::*;

        match condition {
            Condition::Checkbox(_) => Ok(()),
            _ => Err(ConditionTypeMismatch),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Answer::*;
        use AnswerValue::*;
        use ValidateAnswerError::*;

        match answer {
            Value(TextList(_)) => Ok(()),
            Empty => Ok(()),
            _ => Err(IncompatibleAnswerType),
        }
    }

    fn check(&self, answer: &Answer) -> CheckAnswerResult<()> {
        use Answer::*;
        use CheckAnswerError::*;

        match answer {
            Empty if self.required => Err(Required),

            Value(AnswerValue::TextList(choices)) => {
                for choice in choices {
                    if !self.options.contains(choice) {
                        return Err(InvalidOption(choice.clone()));
                    }
                }

                let got = choices.len() as u32;

                if let Some(min) = self.min_selections {
                    if got < min {
                        return Err(TooFewSelections { min, got });
                    }
                }

                if let Some(max) = self.max_selections {
                    if got > max {
                        return Err(TooManySelections { max, got });
                    }
                }

                let mut sorted = choices.clone();
                sorted.sort();
                sorted.dedup();
                if sorted.len() != choices.len() {
                    return Err(DuplicateSelections);
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}
