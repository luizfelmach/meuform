use crate::core::{CustomerId, Form, Graph};
use anyhow::Result;

pub struct CreateFormInput {
    pub customer_id: CustomerId,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct CreateFormOutput {
    pub form: Form,
    pub graph: Graph,
}

pub trait CreateForm {
    async fn create(&self, data: CreateFormInput) -> Result<CreateFormOutput>;
}
