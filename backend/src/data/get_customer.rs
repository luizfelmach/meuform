use crate::{
    protocols::CustomerRepository,
    usecase::{GetCustomer, GetCustomerInput, GetCustomerOutput},
};
use anyhow::anyhow;

pub struct GetCustomerUseCase<R> {
    pub customer: R,
}

impl<R> GetCustomerUseCase<R>
where
    R: CustomerRepository,
{
    pub fn new(customer: R) -> Self {
        Self { customer }
    }
}

impl<R> GetCustomer for GetCustomerUseCase<R>
where
    R: CustomerRepository,
{
    async fn get(&self, data: GetCustomerInput) -> anyhow::Result<GetCustomerOutput> {
        let customer = self
            .customer
            .get_by_id(&data.customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer not found"))?;

        Ok(GetCustomerOutput { customer })
    }
}
