use anyhow::Result;

use crate::core::customer::CustomerId;

pub struct Input {
    pub customer_id: CustomerId,
}

pub trait DeleteCustomer {
    async fn delete(&self, data: Input) -> Result<()>;
}
