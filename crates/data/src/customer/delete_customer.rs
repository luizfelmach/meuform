use anyhow::{Result, anyhow};
use domain::Customer;
use protocols::{DeleteById, FindById};
use usecases::{DeleteCustomer, DeleteCustomerInput};

pub struct DeleteCustomerUseCase<R> {
    repo: R,
}

impl<R> DeleteCustomerUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> DeleteCustomer for DeleteCustomerUseCase<R>
where
    R: FindById<Customer> + DeleteById<Customer>,
{
    async fn execute(&self, data: DeleteCustomerInput) -> Result<()> {
        let DeleteCustomerInput { id } = data;

        let _ = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        let _ = self.repo.delete_by_id(&id).await?;

        Ok(())
    }
}
