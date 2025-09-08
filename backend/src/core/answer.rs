use crate::core::{
    form::FormId,
    graph::{GraphId, NodeId},
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub type AnswerId = String;

pub enum AnswerType {
    Text(String),
    TextArea(String),
    Number(f64),
    Radio(String),
    Checkbox(Vec<String>),
    Boolean(bool),
    Date(DateTime<Utc>),
    Empty,
}

pub struct Answer {
    pub id: AnswerId,
    pub form_id: FormId,
    pub graph_id: GraphId,
    pub completed: bool,
    pub current_node: NodeId,
    pub history: Vec<NodeId>,
    pub responses: HashMap<NodeId, AnswerType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
