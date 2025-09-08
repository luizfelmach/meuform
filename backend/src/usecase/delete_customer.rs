use crate::core::customer::CustomerId;
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
}

pub trait DeleteCustomer {
    async fn delete(&self, data: Input) -> Result<()>;
}
