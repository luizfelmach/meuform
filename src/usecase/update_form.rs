use crate::core::{CustomerId, Form, FormId};
use anyhow::Result;

pub struct UpdateFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
}

pub struct UpdateFormOutput {
    pub form: Form,
}

pub trait UpdateForm {
    async fn update(&self, data: UpdateFormInput) -> Result<UpdateFormOutput>;
}
