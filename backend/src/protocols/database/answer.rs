use crate::core::{Answer, AnswerType, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct CreateAnswerRepository {
    pub form_id: String,
}

pub struct UpdateAnswerRepository {
    pub id: String,
    pub form_id: String,
    pub completed: bool,
    pub current_node: NodeId,
    pub responses: HashMap<NodeId, AnswerType>,
}

pub trait AnswerRepository {
    fn create(&self, data: CreateAnswerRepository) -> Result<Answer>;
    fn get_by_id(&self, id: &String) -> Result<Option<Answer>>;
    fn update(&self, data: UpdateAnswerRepository) -> Result<Answer>;
    fn delete(&self, id: &String) -> Result<()>;
}
