use crate::{Condition, NodeId, Screen};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Nodes = HashMap<NodeId, Screen>;
pub type Edges = HashMap<NodeId, Vec<Edge>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    pub nodes: Nodes,
    pub edges: Edges,
    pub start: NodeId,
    pub end: Vec<NodeId>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Edge {
    Unconditional {
        to: NodeId,
    },
    Conditional {
        to: NodeId,
        r#where: NodeId,
        condition: Condition,
    },
}

impl Graph {
    pub fn init(nodes: Nodes, edges: Edges, start: NodeId, end: Vec<NodeId>) -> Result<Self> {
        let graph = Graph {
            nodes,
            edges,
            start,
            end,
        };
        let _ = graph.validate()?;

        Ok(graph)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            start: 1,
            end: Vec::new(),
        }
    }
}

impl Graph {
    fn validate(&self) -> Result<()> {
        if !self.nodes.contains_key(&self.start) {
            bail!("Start node '{}' does not exist in nodes", self.start);
        }

        for end in &self.end {
            if !self.nodes.contains_key(end) {
                bail!("End node '{}' does not exist in nodes", end);
            }
        }

        for (from, edges) in &self.edges {
            for edge in edges {
                let to = match edge {
                    Edge::Unconditional { to } | Edge::Conditional { to, .. } => to,
                };
                if !self.nodes.contains_key(to) {
                    bail!("Edge from node '{from}' references non-existent node '{to}'");
                }
            }
        }

        for (from, edges) in &self.edges {
            for edge in edges {
                if let Edge::Conditional {
                    condition, r#where, ..
                } = edge
                {
                    if !self.nodes.contains_key(r#where) {
                        bail!("Conditional from '{from}' references non-existent node '{where}'");
                    }

                    let screen = &self.nodes[r#where];
                    let _ = screen.accepts_condition(condition)?;
                }
            }
        }

        Ok(())
    }
}
