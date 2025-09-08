use crate::core::{condition::Condition, screen::Screen};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub type GraphId = String;
pub type NodeId = u64;

pub struct Graph {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
    pub start: NodeId,
    pub end: Vec<NodeId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Node {
    pub id: NodeId,
    pub screen: Screen,
}

pub enum Edge {
    Unconditional { to: NodeId },
    Conditional { to: NodeId, condition: Condition },
}
