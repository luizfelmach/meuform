use crate::core::Graph;
use anyhow::Result;

pub trait GraphRepository {
    async fn create(&self) -> Result<Graph>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Graph>>;
}
