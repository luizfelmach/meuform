use crate::core::{Customer, CustomerId};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct Output {
    pub customer: Customer,
}

pub trait UpdateCustomer {
    async fn update(&self, data: Input) -> Result<Output>;
}
