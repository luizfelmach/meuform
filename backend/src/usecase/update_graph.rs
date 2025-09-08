use crate::core::{CustomerId, Edge, FormId, Graph, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub struct Output {
    pub graph: Graph,
}

pub trait UpdateGraph {
    async fn update(&self, data: Input) -> Result<Output>;
}
