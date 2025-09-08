use anyhow::Result;

use crate::core::customer::{Customer, CustomerId};

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
