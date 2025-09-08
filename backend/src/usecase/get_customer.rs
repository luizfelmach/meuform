use anyhow::Result;

use crate::core::customer::{Customer, CustomerId};

pub struct Input {
    pub customer_id: CustomerId,
}

pub struct Output {
    pub customer: Customer,
}

pub trait GetCustomer {
    async fn get(&self, data: Input) -> Result<Output>;
}
