use crate::core::{Edge, Node, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub trait EdgeValidator {
    fn validate(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<()> {
        self.nodes_exist(nodes, edges)?;
        self.conditions(nodes, edges)?;
        Ok(())
    }

    fn nodes_exist(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<()>;

    fn conditions(
        &self,
        nodes: &HashMap<NodeId, Node>,
        edges: &HashMap<NodeId, Vec<Edge>>,
    ) -> Result<()>;
}
