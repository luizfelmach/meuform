use crate::{Condition, GraphError, NodeId, Result, Screen};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Nodes = HashMap<NodeId, Screen>;
pub type Edges = HashMap<NodeId, Vec<Edge>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    pub nodes: Nodes,
    pub edges: Edges,
    pub start: NodeId,
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
    pub fn new(nodes: Nodes, edges: Edges, start: NodeId) -> Result<Self> {
        let graph = Graph {
            nodes,
            edges,
            start,
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
        }
    }
}

impl Graph {
    fn validate(&self) -> Result<()> {
        if !self.nodes.contains_key(&self.start) {
            return Err(GraphError::StartNodeMissing(self.start))?;
        }

        for (from, edges) in &self.edges {
            for edge in edges {
                let to = match edge {
                    Edge::Unconditional { to } | Edge::Conditional { to, .. } => to,
                };
                if !self.nodes.contains_key(to) {
                    return Err(GraphError::EdgeToNonExistentNode {
                        from: *from,
                        to: *to,
                    })?;
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
                        return Err(GraphError::ConditionalEdgeNodeMissing {
                            from: *from,
                            r#where: *r#where,
                        })?;
                    }

                    let screen = &self.nodes[r#where];
                    screen
                        .accepts_condition(condition)
                        .map_err(|_| GraphError::InvalidCondition(*r#where))?;
                }
            }
        }

        Ok(())
    }
}
