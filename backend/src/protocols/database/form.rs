use crate::core::{CustomerId, Form, GraphId};
use anyhow::Result;

pub struct CreateFormRepository {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub customer_id: CustomerId,
    pub graph_id: GraphId,
}

pub struct UpdateFormRepository {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub customer_id: CustomerId,
    pub graph_id: GraphId,
}

pub trait FormRepository {
    async fn create(&self, data: CreateFormRepository) -> Result<Form>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Form>>;
    async fn get_by_slug(&self, slug: &String) -> Result<Option<Form>>;
    async fn update(&self, data: UpdateFormRepository) -> Result<Form>;
    async fn delete(&self, id: &String) -> Result<()>;
    async fn list_by_owner(&self, customer_id: &String) -> Result<Vec<Form>>;
}
