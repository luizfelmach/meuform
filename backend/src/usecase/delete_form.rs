use crate::core::{customer::CustomerId, form::FormId};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub trait DeleteForm {
    async fn delete(&self, data: Input) -> Result<()>;
}
