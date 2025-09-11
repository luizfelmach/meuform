use anyhow::Result;
use domain::{Edges, NodeId, Nodes};

pub trait GraphValidate: Send + Sync {
    fn has_cycle(&self, nodes: &Nodes, edges: &Edges) -> Result<bool>;

    fn start_node(&self, nodes: &Nodes, edges: &Edges) -> Result<NodeId>;

    fn end_nodes(&self, nodes: &Nodes, edges: &Edges) -> Result<Vec<NodeId>>;
}
