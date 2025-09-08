use crate::core::graph::{Edge, Graph, GraphId, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct UpdateGraph {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub trait GraphRepository {
    fn create(&self) -> Result<Graph>;
    fn get_by_id(&self, id: &String) -> Result<Option<Graph>>;
    fn clone(&self, id: &String) -> Result<Graph>;
    fn update(&self, data: UpdateGraph) -> Result<Graph>;
}
