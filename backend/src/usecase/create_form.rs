use crate::core::{CustomerId, Form};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct Output {
    form: Form,
}

pub trait CreateForm {
    async fn create(&self, data: Input) -> Result<Output>;
}
