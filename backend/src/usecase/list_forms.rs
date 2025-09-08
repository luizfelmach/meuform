use crate::core::{CustomerId, Form};
use anyhow::Result;

pub struct ListFormsInput {
    pub customer_id: CustomerId,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub struct ListFormsOutput {
    pub forms: Vec<Form>,
}

pub trait ListForms {
    async fn list(&self, data: ListFormsInput) -> Result<ListFormsOutput>;
}
