use anyhow::{Result, anyhow, bail};
use domain::Customer;
use protocols::{FindByEmail, Hasher, Tokenizer};
use usecases::{AuthCustomer, AuthCustomerInput, AuthCustomerOutput};

pub struct AuthCustomerUseCase<R, H, T> {
    repo: R,
    hasher: H,
    tokenizer: T,
}

impl<R, H, T> AuthCustomerUseCase<R, H, T> {
    pub fn new(repo: R, hasher: H, tokenizer: T) -> Self {
        Self {
            repo,
            hasher,
            tokenizer,
        }
    }
}

#[async_trait::async_trait]
impl<R, H, T> AuthCustomer for AuthCustomerUseCase<R, H, T>
where
    R: FindByEmail<Customer>,
    H: Hasher,
    T: Tokenizer,
{
    async fn execute(&self, data: AuthCustomerInput) -> Result<AuthCustomerOutput> {
        let customer = self
            .repo
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
