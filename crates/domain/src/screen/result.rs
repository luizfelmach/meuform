use chrono::{DateTime, Utc};

pub type AcceptsConditionResult<T> = std::result::Result<T, AcceptsConditionError>;
pub type ValidateAnswerResult<T> = std::result::Result<T, ValidateAnswerError>;
pub type CheckAnswerResult<T> = std::result::Result<T, CheckAnswerError>;

pub enum AcceptsConditionError {
    ConditionTypeMismatch,
    InfoScreenConditionNotAllowed,
}

pub enum ValidateAnswerError {
    IncompatibleAnswerType,
}

pub enum CheckAnswerError {
    Required,

    TextTooShort {
        min: u32,
        got: u32,
    },

    TextTooLong {
        max: u32,
        got: u32,
    },

    NumberTooSmall {
        min: f64,
        got: f64,
    },

    NumberTooLarge {
        max: f64,
        got: f64,
    },

    InvalidOption(String),

    TooFewSelections {
        min: u32,
        got: u32,
    },

    TooManySelections {
        max: u32,
        got: u32,
    },

    DuplicateSelections,

    DateTooEarly {
        min: DateTime<Utc>,
        got: DateTime<Utc>,
    },

    DateTooLate {
        max: DateTime<Utc>,
        got: DateTime<Utc>,
    },
}
