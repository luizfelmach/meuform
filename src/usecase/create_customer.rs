use crate::core::Customer;
use anyhow::Result;

pub struct CreateCustomerInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct CreateCustomerOutput {
    pub customer: Customer,
}

pub trait CreateCustomer {
    async fn create(&self, data: CreateCustomerInput) -> Result<CreateCustomerOutput>;
}
