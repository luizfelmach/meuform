use crate::NodeId;

pub type SubmissionResult<T> = Result<T, SubmissionError>;

pub enum SubmissionError {
    AlreadyCompleted,
    NoPreviousNode,
    SameNodeNavigation(NodeId),
}
