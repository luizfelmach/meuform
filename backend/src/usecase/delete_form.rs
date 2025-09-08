use crate::core::{CustomerId, FormId};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub trait DeleteForm {
    async fn delete(&self, data: Input) -> Result<()>;
}
