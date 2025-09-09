use crate::{
    protocols::{CreateCustomerRepository, CustomerRepository, Hasher},
    usecase::{CreateCustomer, CreateCustomerInput, CreateCustomerOutput},
};
use anyhow::{Result, bail};

pub struct CreateCustomerUseCase<R, H> {
    pub customer: R,
    pub hasher: H,
}

impl<R, H> CreateCustomerUseCase<R, H>
where
    R: CustomerRepository,
    H: Hasher,
{
    pub fn new(customer: R, hasher: H) -> Self {
        Self { customer, hasher }
    }
}

impl<R, H> CreateCustomer for CreateCustomerUseCase<R, H>
where
    R: CustomerRepository,
    H: Hasher,
{
    async fn create(&self, data: CreateCustomerInput) -> Result<CreateCustomerOutput> {
        let exists = self.customer.find_by_email(&data.email).await?;

        match exists {
            None => {
                let password = self.hasher.hash(data.password)?;

                let customer = self
                    .customer
                    .create(CreateCustomerRepository {
                        name: data.name,
                        email: data.email,
                        password,
                    })
                    .await?;

                Ok(CreateCustomerOutput { customer })
            }
            Some(_) => bail!("User already exists"),
        }
    }
}
