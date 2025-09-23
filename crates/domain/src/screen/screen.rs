use crate::{
    AcceptsConditionResult, Answer, BooleanScreen, CheckAnswerResult, CheckboxScreen, Condition,
    DateScreen, InfoScreen, NumberScreen, RadioScreen, TextAreaScreen, TextScreen,
    ValidateAnswerResult,
};

use serde::{Deserialize, Serialize};

pub trait Screenable {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()>;
    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()>;
    fn check(&self, answer: &Answer) -> CheckAnswerResult<()>;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Screen {
    Text(TextScreen),
    TextArea(TextAreaScreen),
    Number(NumberScreen),
    Radio(RadioScreen),
    Checkbox(CheckboxScreen),
    Date(DateScreen),
    Boolean(BooleanScreen),
    Info(InfoScreen),
}

impl Screenable for Screen {
    fn accepts(&self, condition: &Condition) -> AcceptsConditionResult<()> {
        use Screen::*;

        match self {
            Text(s) => s.accepts(condition),
            TextArea(s) => s.accepts(condition),
            Number(s) => s.accepts(condition),
            Radio(s) => s.accepts(condition),
            Checkbox(s) => s.accepts(condition),
            Date(s) => s.accepts(condition),
            Boolean(s) => s.accepts(condition),
            Info(s) => s.accepts(condition),
        }
    }

    fn validate(&self, answer: &Answer) -> ValidateAnswerResult<()> {
        use Screen::*;

        match self {
            Text(s) => s.validate(answer),
            TextArea(s) => s.validate(answer),
            Number(s) => s.validate(answer),
            Radio(s) => s.validate(answer),
            Checkbox(s) => s.validate(answer),
            Date(s) => s.validate(answer),
            Boolean(s) => s.validate(answer),
            Info(s) => s.validate(answer),
        }
    }

    fn check(&self, answer: &Answer) -> CheckAnswerResult<()> {
        use Screen::*;

        match self {
            Text(s) => s.check(answer),
            TextArea(s) => s.check(answer),
            Number(s) => s.check(answer),
            Radio(s) => s.check(answer),
            Checkbox(s) => s.check(answer),
            Date(s) => s.check(answer),
            Boolean(s) => s.check(answer),
            Info(s) => s.check(answer),
        }
    }
}
