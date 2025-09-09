use crate::core::{CustomerId, Form, FormId, Graph};
use anyhow::Result;

pub struct GetFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub struct GetFormOutput {
    pub form: Form,
    pub graph: Graph,
}

pub trait GetForm {
    async fn get(&self, data: GetFormInput) -> Result<GetFormOutput>;
}
