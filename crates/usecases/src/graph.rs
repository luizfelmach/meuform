use crate::UseCaseResult;

use domain::{Edge, FormId, Graph, NodeId, Screen};
use std::sync::Arc;

pub type DynCustomerGetGraph = Arc<dyn CustomerGetGraph>;
pub type DynCustomerSetNodeGraph = Arc<dyn CustomerSetNodeGraph>;
pub type DynCustomerSetEdgeGraph = Arc<dyn CustomerSetNodeGraph>;
pub type DynCustomerDeleteNodeGraph = Arc<dyn CustomerSetNodeGraph>;
pub type DynCustomerDeleteEdgeGraph = Arc<dyn CustomerSetNodeGraph>;

#[async_trait::async_trait]
pub trait CustomerGetGraph: Send + Sync {
    async fn execute(&self, id: &FormId) -> UseCaseResult<Graph>;
}

#[async_trait::async_trait]
pub trait CustomerSetNodeGraph: Send + Sync {
    async fn execute(&self, id: &FormId, data: SetNodeGraphInput) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerSetEdgeGraph: Send + Sync {
    async fn execute(&self, id: &FormId, data: SetEdgeGraphInput) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerDeleteNodeGraph: Send + Sync {
    async fn execute(&self, id: &FormId, node: &NodeId) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerDeleteEdgeGraph: Send + Sync {
    async fn execute(&self, id: &FormId, from: &NodeId, to: &NodeId) -> UseCaseResult<()>;
}

pub struct SetNodeGraphInput {
    pub node: NodeId,
    pub screen: Screen,
}

pub struct SetEdgeGraphInput {
    pub from: NodeId,
    pub edge: Edge,
}
