use crate::{Condition, NodeId, Screen};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Nodes = HashMap<NodeId, Node>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub screen: Screen,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    pub nodes: Nodes,
    pub start: Option<NodeId>,
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
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            start: None,
        }
    }
}
