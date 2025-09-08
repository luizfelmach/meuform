use crate::core::{Answer, AnswerType, NodeId};
use anyhow::Result;
use std::collections::HashMap;

pub struct CreateAnswerRepository {
    pub form_id: String,
    pub graph_id: String,
}

pub trait AnswerRepository {
    async fn create(&self, data: CreateAnswerRepository) -> Result<Answer>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Answer>>;
    async fn update(&self, data: Answer) -> Result<Answer>;
    async fn delete(&self, id: &String) -> Result<()>;
}
