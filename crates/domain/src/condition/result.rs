pub type EvaluateAnswerResult<T> = std::result::Result<T, EvaluateAnswerError>;

pub enum EvaluateAnswerError {
    AnswerTypeMismatch,
    EmptyAnswer,
}
