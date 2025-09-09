use crate::core::CustomerId;
use anyhow::Result;

pub struct DeleteCustomerInput {
    pub customer_id: CustomerId,
}

pub trait DeleteCustomer {
    async fn delete(&self, data: DeleteCustomerInput) -> Result<()>;
}
