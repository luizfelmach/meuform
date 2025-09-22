use crate::screen::{BooleanScreen, CheckboxScreen, DateScreen, InfoScreen};
use crate::screen::{NumberScreen, RadioScreen, TextAreaScreen, TextScreen};
use serde::{Deserialize, Serialize};

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
