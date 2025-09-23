use crate::{Condition, NodeId, Screen};

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
    pub fn new(nodes: Nodes, edges: Edges, start: NodeId) -> Self {
        Self {
            nodes,
            edges,
            start,
        }
    }
}
