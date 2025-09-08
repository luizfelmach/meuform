use crate::{
    protocols::{CustomerRepository, Hasher, Tokenizer},
    usecase::{AuthCustomer, AuthCustomerInput, AuthCustomerOutput},
};
use anyhow::{Result, anyhow, bail};

pub struct AuthCustomerUseCase<R, H, T> {
    pub customer: R,
    pub hasher: H,
    pub tokenizer: T,
}

impl<R, H, T> AuthCustomerUseCase<R, H, T>
where
    R: CustomerRepository,
    H: Hasher,
    T: Tokenizer,
{
    pub fn new(customer: R, hasher: H, tokenizer: T) -> Self {
        Self {
            customer,
            hasher,
            tokenizer,
        }
    }
}

impl<R, H, T> AuthCustomer for AuthCustomerUseCase<R, H, T>
where
    R: CustomerRepository,
    H: Hasher,
    T: Tokenizer,
{
    async fn auth(&self, data: AuthCustomerInput) -> Result<AuthCustomerOutput> {
        let customer = self
            .customer
            .find_by_email(&data.email)
            .await?
            .ok_or_else(|| anyhow!("Invalid e-mail or password"))?;

        match self.hasher.compare(data.password, customer.password)? {
            true => {
                let token = self.tokenizer.encrypt(customer.id)?;
                Ok(AuthCustomerOutput { token })
            }
            false => bail!("Invalid e-mail or password"),
        }
    }
}
