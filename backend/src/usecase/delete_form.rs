use anyhow::Result;

use crate::core::{customer::CustomerId, form::FormId};

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub trait DeleteForm {
    async fn delete(&self, data: Input) -> Result<()>;
}
