use anyhow::{Result, anyhow, bail};
use domain::{Customer, Form};
use protocols::FindById;
use usecases::{GetForm, GetFormInput, GetFormOutput};

pub struct GetFormUseCase<R> {
    repo: R,
}

impl<R> GetFormUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> GetForm for GetFormUseCase<R>
where
    R: FindById<Customer> + FindById<Form>,
{
    async fn execute(&self, data: GetFormInput) -> Result<GetFormOutput> {
        let GetFormInput { id, customer_id } = data;

        let customer: Customer = self
            .repo
            .find_by_id(&customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        let form: Form = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Form does not exists"))?;

        if form.customer_id != customer.id {
            bail!("Unauthorized")
        }

        Ok(GetFormOutput { form })
    }
}
