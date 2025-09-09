use std::collections::HashMap;

use crate::core::{Edge, Graph, Node, NodeId};
use anyhow::Result;

pub struct CreateGraphRepository {
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
    pub start: NodeId,
    pub end: Vec<NodeId>,
}

pub trait GraphRepository {
    async fn create(&self, data: CreateGraphRepository) -> Result<Graph>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Graph>>;
}
