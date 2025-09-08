use chrono::{DateTime, Utc};

use crate::core::{condition::Condition, screen::Screen};
use std::collections::HashMap;

pub type GraphId = String;
pub type NodeId = u64;

pub struct Graph {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Node {
    pub id: NodeId,
    pub screen: Screen,
}

pub enum Edge {
    Unconditional {
        from: NodeId,
        to: NodeId,
    },
    Conditional {
        from: NodeId,
        to: NodeId,
        condition: Condition,
    },
}
