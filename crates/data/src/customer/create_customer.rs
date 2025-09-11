use anyhow::{Result, bail};
use domain::Customer;
use protocols::{FindByEmail, GenerateUuid, Hasher, Save};
use usecases::{CreateCustomer, CreateCustomerInput, CreateCustomerOutput};

pub struct CreateCustomerUseCase<R, H> {
    repo: R,
    hasher: H,
}

impl<R, H> CreateCustomerUseCase<R, H> {
    pub fn new(repo: R, hasher: H) -> Self {
        Self { repo, hasher }
    }
}

#[async_trait::async_trait]
impl<R, H> CreateCustomer for CreateCustomerUseCase<R, H>
where
    R: FindByEmail<Customer> + Save<Customer> + GenerateUuid,
    H: Hasher,
{
    async fn execute(&self, data: CreateCustomerInput) -> Result<CreateCustomerOutput> {
        let CreateCustomerInput {
            name,
            email,
            password,
        } = data;

        let exists = self.repo.find_by_email(&email).await?;

        if let Some(_) = exists {
            bail!("Customer already exists.")
        }

        let id = self.repo.generate_uuid()?;
        let hashed = self.hasher.hash(password)?;
        let customer = Customer::new(id, name, email, hashed);
        let customer = self.repo.save(customer).await?;

        Ok(CreateCustomerOutput { customer })
    }
}
