use crate::{Answer, AnswerValue, Result, ScreenError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Screen {
    pub title: String,
    pub description: Option<String>,
    pub field: ScreenField,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionItem {
    pub value: String,
    pub label: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum ScreenField {
    Text {
        placeholder: Option<String>,
        min_length: Option<u32>,
        max_length: Option<u32>,
        required: bool,
    },
    TextArea {
        placeholder: Option<String>,
        rows: Option<u32>,
        min_length: Option<u32>,
        max_length: Option<u32>,
        required: bool,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        required: bool,
    },
    Radio {
        options: Vec<OptionItem>,
        required: bool,
    },
    Checkbox {
        options: Vec<OptionItem>,
        min_selections: Option<u32>,
        max_selections: Option<u32>,
        required: bool,
    },
    Date {
        min_date: Option<DateTime<Utc>>,
        max_date: Option<DateTime<Utc>>,
        required: bool,
    },
    Boolean {
        true_label: Option<String>,
        false_label: Option<String>,
        required: bool,
    },
    Info,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum TextCondition {
    Equals(String),
    NotEquals(String),
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum TextAreaCondition {
    Equals(String),
    NotEquals(String),
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum NumberCondition {
    Equals(f64),
    NotEquals(f64),
    GreaterThan(f64),
    LessThan(f64),
    GreaterOrEqual(f64),
    LessOrEqual(f64),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum RadioCondition {
    InText(Vec<String>),
    NotInText(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum CheckboxCondition {
    Equals(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum DateCondition {
    Equals(DateTime<Utc>),
    NotEquals(DateTime<Utc>),
    GreaterThan(DateTime<Utc>),
    LessThan(DateTime<Utc>),
    GreaterOrEqual(DateTime<Utc>),
    LessOrEqual(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "operator", content = "value")]
pub enum BooleanCondition {
    Equals(bool),
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

impl Condition {
    pub fn evaluate(&self, value: &Answer) -> bool {
        use Condition::*;

        match (self.clone(), value.clone()) {
            (Text(cond), Answer::Value(AnswerValue::Text(v))) => match cond {
                TextCondition::Equals(s) => v == s,
                TextCondition::NotEquals(s) => v != s,
                TextCondition::Contains(s) => v.contains(&s),
                TextCondition::NotContains(s) => !v.contains(&s),
                TextCondition::StartsWith(s) => v.starts_with(&s),
                TextCondition::EndsWith(s) => v.ends_with(&s),
            },
            (TextArea(cond), Answer::Value(AnswerValue::Text(v))) => match cond {
                TextAreaCondition::Equals(s) => v == s,
                TextAreaCondition::NotEquals(s) => v != s,
                TextAreaCondition::Contains(s) => v.contains(&s),
                TextAreaCondition::NotContains(s) => !v.contains(&s),
                TextAreaCondition::StartsWith(s) => v.starts_with(&s),
                TextAreaCondition::EndsWith(s) => v.ends_with(&s),
            },
            (Number(cond), Answer::Value(AnswerValue::Number(v))) => match cond {
                NumberCondition::Equals(n) => v == n,
                NumberCondition::NotEquals(n) => v != n,
                NumberCondition::GreaterThan(n) => v > n,
                NumberCondition::LessThan(n) => v < n,
                NumberCondition::GreaterOrEqual(n) => v >= n,
                NumberCondition::LessOrEqual(n) => v <= n,
            },
            (Radio(cond), Answer::Value(AnswerValue::Text(v))) => match cond {
                RadioCondition::InText(options) => options.contains(&v),
                RadioCondition::NotInText(options) => !options.contains(&v),
            },
            (Checkbox(cond), Answer::Value(AnswerValue::TextList(v))) => match cond {
                CheckboxCondition::Equals(c) => v == c,
            },
            (Date(cond), Answer::Value(AnswerValue::Date(v))) => match cond {
                DateCondition::Equals(d) => v == d,
                DateCondition::NotEquals(d) => v != d,
                DateCondition::GreaterThan(d) => v > d,
                DateCondition::LessThan(d) => v < d,
                DateCondition::GreaterOrEqual(d) => v >= d,
                DateCondition::LessOrEqual(d) => v <= d,
            },
            (Boolean(cond), Answer::Value(AnswerValue::Boolean(v))) => match cond {
                BooleanCondition::Equals(b) => v == b,
            },
            _ => false,
        }
    }
}

impl Screen {
    pub fn accepts_answer(&self, answer: &Answer) -> Result<()> {
        match answer {
            Answer::Empty if self.field.required() => Err(ScreenError::RequiredField)?,
            Answer::Empty => Ok(()),
            Answer::Value(value) => self.field.validate(value),
        }
    }

    pub fn accepts_condition(&self, condition: &Condition) -> Result<()> {
        use Condition::*;

        match (&self.field, condition.clone()) {
            (ScreenField::Text { .. }, Text(_)) => Ok(()),
            (ScreenField::TextArea { .. }, TextArea(_)) => Ok(()),
            (ScreenField::Number { .. }, Number(_)) => Ok(()),
            (ScreenField::Radio { .. }, Radio(_)) => Ok(()),
            (ScreenField::Checkbox { .. }, Checkbox(_)) => Ok(()),
            (ScreenField::Date { .. }, Date(_)) => Ok(()),
            (ScreenField::Boolean { .. }, Boolean(_)) => Ok(()),
            (ScreenField::Info, _) => Err(ScreenError::InfoFieldConditionNotAllowed)?,
            _ => Err(ScreenError::ConditionTypeMismatch)?,
        }
    }
}

impl ScreenField {
    pub fn required(&self) -> bool {
        use ScreenField::*;
        match self {
            Text { required, .. }
            | TextArea { required, .. }
            | Number { required, .. }
            | Radio { required, .. }
            | Checkbox { required, .. }
            | Date { required, .. }
            | Boolean { required, .. } => *required,
            Info => false,
        }
    }
}

impl ScreenField {
    pub fn validate(&self, value: &AnswerValue) -> Result<()> {
        match (self, value) {
            (ScreenField::Text { .. }, AnswerValue::Text(v)) => self.validate_text(v),
            (ScreenField::TextArea { .. }, AnswerValue::Text(v)) => self.validate_textarea(v),
            (ScreenField::Number { .. }, AnswerValue::Number(v)) => self.validate_number(*v),
            (ScreenField::Radio { .. }, AnswerValue::Text(v)) => self.validate_radio(v),
            (ScreenField::Checkbox { .. }, AnswerValue::TextList(v)) => self.validate_checkbox(v),
            (ScreenField::Date { .. }, AnswerValue::Date(v)) => self.validate_date(v),
            (ScreenField::Boolean { .. }, AnswerValue::Boolean(v)) => self.validate_boolean(*v),
            _ => Err(ScreenError::TypeMismatch)?,
        }
    }

    fn validate_text(&self, text: &str) -> Result<()> {
        if let ScreenField::Text {
            min_length,
            max_length,
            ..
        } = self
        {
            let len = text.chars().count() as u32;
            if let Some(min) = min_length {
                if len < *min {
                    return Err(ScreenError::TextTooShort {
                        min: *min,
                        got: len,
                    })?;
                }
            }
            if let Some(max) = max_length {
                if len > *max {
                    return Err(ScreenError::TextTooLong {
                        max: *max,
                        got: len,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn validate_textarea(&self, text: &str) -> Result<()> {
        if let ScreenField::TextArea {
            min_length,
            max_length,
            ..
        } = self
        {
            let len = text.chars().count() as u32;
            if let Some(min) = min_length {
                if len < *min {
                    return Err(ScreenError::TextTooShort {
                        min: *min,
                        got: len,
                    })?;
                }
            }
            if let Some(max) = max_length {
                if len > *max {
                    return Err(ScreenError::TextTooLong {
                        max: *max,
                        got: len,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn validate_number(&self, num: f64) -> Result<()> {
        if let ScreenField::Number { min, max, .. } = self {
            if let Some(min) = min {
                if num < *min {
                    return Err(ScreenError::NumberTooSmall {
                        min: *min,
                        got: num,
                    })?;
                }
            }
            if let Some(max) = max {
                if num > *max {
                    return Err(ScreenError::NumberTooLarge {
                        max: *max,
                        got: num,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn validate_radio(&self, selected: &str) -> Result<()> {
        if let ScreenField::Radio { options, .. } = self {
            if !options.iter().any(|o| o.value == *selected) {
                return Err(ScreenError::InvalidOption {
                    value: selected.into(),
                })?;
            }
        }
        Ok(())
    }

    fn validate_checkbox(&self, selected: &Vec<String>) -> Result<()> {
        if let ScreenField::Checkbox {
            options,
            min_selections,
            max_selections,
            ..
        } = self
        {
            for sel in selected {
                if !options.iter().any(|o| o.value == *sel) {
                    return Err(ScreenError::InvalidOption { value: sel.clone() })?;
                }
            }

            let count = selected.len() as u32;

            if let Some(min) = min_selections {
                if count < *min {
                    return Err(ScreenError::TooFewSelections {
                        min: *min,
                        got: count,
                    })?;
                }
            }
            if let Some(max) = max_selections {
                if count > *max {
                    return Err(ScreenError::TooManySelections {
                        max: *max,
                        got: count,
                    })?;
                }
            }

            if selected.len() != HashSet::<_>::from_iter(selected.iter()).len() {
                return Err(ScreenError::DuplicateSelections)?;
            }
        }
        Ok(())
    }

    fn validate_date(&self, date: &DateTime<Utc>) -> Result<()> {
        if let ScreenField::Date {
            min_date, max_date, ..
        } = self
        {
            if let Some(min) = min_date {
                if date < min {
                    return Err(ScreenError::DateTooEarly {
                        min: *min,
                        got: *date,
                    })?;
                }
            }
            if let Some(max) = max_date {
                if date > max {
                    return Err(ScreenError::DateTooLate {
                        max: *max,
                        got: *date,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn validate_boolean(&self, _value: bool) -> Result<()> {
        Ok(())
    }
}
