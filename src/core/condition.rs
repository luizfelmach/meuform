use crate::core::graph::NodeId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Condition {
    TextEquals(NodeId, String),
    TextNotEquals(NodeId, String),
    TextIn(NodeId, Vec<String>),
    TextNotIn(NodeId, Vec<String>),

    NumberEquals(NodeId, f64),
    NumberNotEquals(NodeId, f64),
    NumberGreaterThan(NodeId, f64),
    NumberLessThan(NodeId, f64),

    BooleanEquals(NodeId, bool),

    DateEquals(NodeId, DateTime<Utc>),
    DateNotEquals(NodeId, DateTime<Utc>),
    DateBefore(NodeId, DateTime<Utc>),
    DateAfter(NodeId, DateTime<Utc>),
}
