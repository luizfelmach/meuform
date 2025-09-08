use crate::core::{customer::CustomerId, form::Form};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub struct Output {
    pub forms: Vec<Form>,
}

pub trait ListForms {
    async fn list(&self, data: Input) -> Result<Output>;
}
