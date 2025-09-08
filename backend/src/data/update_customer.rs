use crate::{
    protocols::CustomerRepository,
    usecase::{UpdateCustomer, UpdateCustomerInput, UpdateCustomerOutput},
};
use anyhow::anyhow;

pub struct UpdateCustomerUseCase<R> {
    pub customer: R,
}

impl<R> UpdateCustomerUseCase<R>
where
    R: CustomerRepository,
{
    pub fn new(customer: R) -> Self {
        Self { customer }
    }
}

impl<R> UpdateCustomer for UpdateCustomerUseCase<R>
where
    R: CustomerRepository,
{
    async fn update(&self, data: UpdateCustomerInput) -> anyhow::Result<UpdateCustomerOutput> {
        let mut customer = self
            .customer
            .get_by_id(&data.customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer not found"))?;

        if let Some(email) = data.email {
            customer.email = email
        }

        if let Some(name) = data.name {
            customer.name = name
        }

        let customer = self.customer.update(customer).await?;

        Ok(UpdateCustomerOutput { customer })
    }
}
