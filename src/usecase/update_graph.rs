use crate::core::{CustomerId, Edge, FormId, Graph, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct UpdateGraphInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub struct UpdateGraphOutput {
    pub graph: Graph,
}

pub trait UpdateGraph {
    async fn update(&self, data: UpdateGraphInput) -> Result<UpdateGraphOutput>;
}
