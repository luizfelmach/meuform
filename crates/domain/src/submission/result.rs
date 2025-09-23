use crate::NodeId;

pub type SubmissionOpResult<T> = Result<T, SubmissionOpError>;

pub enum SubmissionOpError {
    AlreadyCompleted,
    NoPreviousNode,
    SameNodeNavigation(NodeId),
}
