use crate::core::{CustomerId, Form, FormId};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
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
