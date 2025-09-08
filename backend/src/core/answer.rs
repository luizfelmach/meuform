use crate::core::{form::FormId, graph::NodeId};
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
    pub completed: bool,
    pub current_node: NodeId,
    pub responses: HashMap<NodeId, AnswerType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
