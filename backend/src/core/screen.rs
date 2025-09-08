use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Screen {
    pub base: ScreenBase,
    pub kind: ScreenKind,
}

#[derive(Clone)]
pub struct ScreenBase {
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Clone)]
pub enum ScreenKind {
    Text(TextConfig),
    TextArea(TextAreaConfig),
    Number(NumberConfig),
    Radio(RadioConfig),
    Checkbox(CheckboxConfig),
    Date(DateConfig),
    Boolean(BooleanConfig),
}

#[derive(Clone)]
pub struct TextConfig {
    pub placeholder: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
}

#[derive(Clone)]
pub struct TextAreaConfig {
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
}

#[derive(Clone)]
pub struct NumberConfig {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone)]
pub struct RadioConfig {
    pub options: Vec<SelectOption>,
}

#[derive(Clone)]
pub struct CheckboxConfig {
    pub options: Vec<SelectOption>,
    pub min_selections: Option<u32>,
    pub max_selections: Option<u32>,
}

#[derive(Clone)]
pub struct DateConfig {
    pub min_date: Option<DateTime<Utc>>,
    pub max_date: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct BooleanConfig {
    pub true_label: Option<String>,
    pub false_label: Option<String>,
}
