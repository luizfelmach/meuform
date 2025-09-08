use crate::{
    protocols::{CustomerRepository, UpdateCustomerRepository},
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
        let customer = self
            .customer
            .get_by_id(&data.customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer not found"))?;

        let customer = self
            .customer
            .update(UpdateCustomerRepository {
                id: customer.id,
                name: data.name,
                email: data.email,
                password: None,
            })
            .await?;

        Ok(UpdateCustomerOutput { customer })
    }
}
