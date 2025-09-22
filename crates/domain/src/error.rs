pub type DomainResult<T> = std::result::Result<T, DomainError>;

pub enum DomainError {
    Validation(Vec<ValidationError>),
}

pub enum ValidationError {
    RequiredField(String),
    TooShortField { got: i64, min: i64 },
    TooLongField { got: i64, max: i64 },
    OutOfRangeField { min: i64, max: i64, got: i64 },
    InvalidFormatField(FormatKind),
}

pub enum FormatKind {
    Email,
}
