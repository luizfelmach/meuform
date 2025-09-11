use anyhow::{Result, anyhow};
use domain::Customer;
use protocols::FindById;
use usecases::{GetCustomer, GetCustomerInput, GetCustomerOutput};

pub struct GetCustomerUseCase<R> {
    repo: R,
}

impl<R> GetCustomerUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> GetCustomer for GetCustomerUseCase<R>
where
    R: FindById<Customer>,
{
    async fn execute(&self, data: GetCustomerInput) -> Result<GetCustomerOutput> {
        let GetCustomerInput { id } = data;

        let customer = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        Ok(GetCustomerOutput { customer })
    }
}
