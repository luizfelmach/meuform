use crate::core::{Customer, CustomerId};
use anyhow::Result;

pub struct GetCustomerInput {
    pub customer_id: CustomerId,
}

pub struct GetCustomerOutput {
    pub customer: Customer,
}

pub trait GetCustomer {
    async fn get(&self, data: GetCustomerInput) -> Result<GetCustomerOutput>;
}
