use crate::core::{Edge, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub trait GraphTopology {
    fn is_dag(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<bool>;

    fn start_node(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<NodeId>;

    fn end_nodes(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<Vec<NodeId>>;
}
