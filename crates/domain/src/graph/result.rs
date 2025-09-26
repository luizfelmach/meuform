use crate::{AcceptsConditionError, NodeId};

pub type GraphResult<T> = std::result::Result<T, GraphError>;

pub enum GraphError {
    NodeNotFound(NodeId),
    FromNodeNotFound(NodeId),
    ToNodeNotFound(NodeId),
    SelfLoop(NodeId),
    InvalidCondition(AcceptsConditionError),
    EdgeNotFound(NodeId, NodeId),
}
