mod domain;
mod infra;
mod validation;

pub use domain::*;
pub use infra::*;
pub use validation::*;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Domain(#[from] DomainError),

    #[error("{0}")]
    Submission(#[from] SubmissionError),

    #[error("{0}")]
    Graph(#[from] GraphError),

    #[error("{0}")]
    Screen(#[from] ScreenError),

    #[error("{0}")]
    Infra(#[from] InfraError),
}
