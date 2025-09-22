use crate::answer::Answer;
use crate::condition::Condition;
use crate::screen::{BooleanScreen, CheckboxScreen, DateScreen, InfoScreen, Result};
use crate::screen::{NumberScreen, RadioScreen, TextAreaScreen, TextScreen};
use serde::{Deserialize, Serialize};

pub trait Screenable {
    fn accepts(&self, condition: &Condition) -> Result<()>;
    fn check(&self, answer: &Answer) -> Result<()>;
    fn evaluate(&self, answer: &Answer) -> Result<()>;
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
    fn accepts(&self, condition: &Condition) -> Result<()> {
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

    fn check(&self, answer: &Answer) -> Result<()> {
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

    fn evaluate(&self, answer: &Answer) -> Result<()> {
        use Screen::*;

        match self {
            Text(s) => s.evaluate(answer),
            TextArea(s) => s.evaluate(answer),
            Number(s) => s.evaluate(answer),
            Radio(s) => s.evaluate(answer),
            Checkbox(s) => s.evaluate(answer),
            Date(s) => s.evaluate(answer),
            Boolean(s) => s.evaluate(answer),
            Info(s) => s.evaluate(answer),
        }
    }
}
