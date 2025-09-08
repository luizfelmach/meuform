use crate::core::{CustomerId, FormId};
use anyhow::Result;

pub struct DeleteFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub trait DeleteForm {
    async fn delete(&self, data: DeleteFormInput) -> Result<()>;
}
