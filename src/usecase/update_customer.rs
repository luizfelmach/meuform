use crate::core::{Customer, CustomerId};
use anyhow::Result;

pub struct UpdateCustomerInput {
    pub customer_id: CustomerId,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct UpdateCustomerOutput {
    pub customer: Customer,
}

pub trait UpdateCustomer {
    async fn update(&self, data: UpdateCustomerInput) -> Result<UpdateCustomerOutput>;
}
