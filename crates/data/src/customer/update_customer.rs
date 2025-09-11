use anyhow::{Result, anyhow, bail};
use domain::Customer;
use protocols::{FindByEmail, FindById, Update};
use usecases::{UpdateCustomer, UpdateCustomerInput, UpdateCustomerOutput};

pub struct UpdateCustomerUseCase<R> {
    repo: R,
}

impl<R> UpdateCustomerUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> UpdateCustomer for UpdateCustomerUseCase<R>
where
    R: FindById<Customer> + FindByEmail<Customer> + Update<Customer>,
{
    async fn execute(&self, data: UpdateCustomerInput) -> Result<UpdateCustomerOutput> {
        let UpdateCustomerInput { id, name, email } = data;

        let mut customer = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        if let Some(name) = name {
            customer.set_name(name);
        }

        if let Some(email) = email {
            let exists = self.repo.find_by_email(&email).await?;

            if let Some(_) = exists {
                bail!("Customer already exists.")
            }

            customer.set_email(email);
        }

        let customer = self.repo.update(customer).await?;

        Ok(UpdateCustomerOutput { customer })
    }
}
