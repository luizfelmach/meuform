use crate::answer::{Answer, AnswerValue};
use crate::condition::{BooleanCondition, CheckboxCondition, TextAreaCondition, TextCondition};
use crate::condition::{DateCondition, NumberCondition, RadioCondition};
use crate::condition::{Error, Result};
use serde::{Deserialize, Serialize};

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

impl Condition {
    pub fn evaluate(&self, value: &Answer) -> Result<bool> {
        use Answer::*;
        use Condition::*;
        use Error::*;

        match (self, value) {
            (Text(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (TextArea(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (Number(cond), Value(AnswerValue::Number(v))) => Ok(cond.evaluate(v)),
            (Radio(cond), Value(AnswerValue::Text(v))) => Ok(cond.evaluate(v)),
            (Checkbox(cond), Value(AnswerValue::TextList(v))) => Ok(cond.evaluate(v)),
            (Date(cond), Value(AnswerValue::Date(v))) => Ok(cond.evaluate(v)),
            (Boolean(cond), Value(AnswerValue::Boolean(v))) => Ok(cond.evaluate(v)),
            (_, Empty) => Err(EmptyAnswer),
            _ => Err(AnswerTypeMismatch),
        }
    }
}
