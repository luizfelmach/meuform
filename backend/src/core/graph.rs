use crate::core::{condition::Condition, screen::Screen};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type GraphId = String;
pub type NodeId = u64;

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    #[serde(rename = "_id")]
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
    pub start: NodeId,
    pub end: Vec<NodeId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub screen: Screen,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Edge {
    Unconditional { to: NodeId },
    Conditional { to: NodeId, condition: Condition },
}
