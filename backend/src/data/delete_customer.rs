use crate::{
    protocols::CustomerRepository,
    usecase::{DeleteCustomer, DeleteCustomerInput},
};
use anyhow::anyhow;

pub struct DeleteCustomerUseCase<R> {
    pub customer: R,
}

impl<R> DeleteCustomerUseCase<R>
where
    R: CustomerRepository,
{
    pub fn new(customer: R) -> Self {
        Self { customer }
    }
}

impl<R> DeleteCustomer for DeleteCustomerUseCase<R>
where
    R: CustomerRepository,
{
    async fn delete(&self, data: DeleteCustomerInput) -> anyhow::Result<()> {
        let customer = self
            .customer
            .get_by_id(&data.customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer not found"))?;

        let _ = self.customer.delete(&customer.id).await?;

        Ok(())
    }
}
