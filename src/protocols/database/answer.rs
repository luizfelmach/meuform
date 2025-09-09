use crate::core::{Answer, NodeId};
use anyhow::Result;

pub struct CreateAnswerRepository {
    pub form_id: String,
    pub graph_id: String,
    pub current_node: NodeId,
}

pub trait AnswerRepository {
    async fn create(&self, data: CreateAnswerRepository) -> Result<Answer>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Answer>>;
    async fn update(&self, data: Answer) -> Result<Answer>;
    async fn list_by_form(&self, form_id: &String) -> Result<Vec<Answer>>;
}
