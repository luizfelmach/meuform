use crate::core::{Edge, Graph, GraphId, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct UpdateGraphRepository {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub trait GraphRepository {
    async fn create(&self) -> Result<Graph>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Graph>>;
    async fn clone(&self, id: &String) -> Result<Graph>;
    async fn update(&self, data: UpdateGraphRepository) -> Result<Graph>;
}
