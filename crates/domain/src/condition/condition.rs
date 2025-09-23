use crate::{
    Answer, BooleanCondition, CheckboxCondition, DateCondition, EvaluateAnswerResult,
    NumberCondition, RadioCondition, TextAreaCondition, TextCondition,
};

use serde::{Deserialize, Serialize};

pub trait Conditionable {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool>;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "condition")]
pub enum Condition {
    Text(TextCondition),
    TextArea(TextAreaCondition),
    Number(NumberCondition),
    Radio(RadioCondition),
    Checkbox(CheckboxCondition),
    Date(DateCondition),
    Boolean(BooleanCondition),
}

impl Conditionable for Condition {
    fn evaluate(&self, answer: &Answer) -> EvaluateAnswerResult<bool> {
        use Condition::*;

        match self {
            Text(c) => c.evaluate(answer),
            TextArea(c) => c.evaluate(answer),
            Number(c) => c.evaluate(answer),
            Radio(c) => c.evaluate(answer),
            Checkbox(c) => c.evaluate(answer),
            Date(c) => c.evaluate(answer),
            Boolean(c) => c.evaluate(answer),
        }
    }
}
