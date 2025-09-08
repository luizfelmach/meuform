use crate::core::form::Form;
use anyhow::Result;

pub struct Input {
    pub customer_id: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
}

pub struct Output {
    pub form: Form,
}

pub trait UpdateForm {
    async fn update(&self, data: Input) -> Result<Output>;
}
