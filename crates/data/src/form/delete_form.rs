use anyhow::{Result, anyhow, bail};
use domain::{Customer, Form};
use protocols::{DeleteById, FindById};
use usecases::{DeleteForm, DeleteFormInput};

pub struct DeleteFormUseCase<R> {
    repo: R,
}

impl<R> DeleteFormUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> DeleteForm for DeleteFormUseCase<R>
where
    R: FindById<Customer> + FindById<Form> + DeleteById<Form>,
{
    async fn execute(&self, data: DeleteFormInput) -> Result<()> {
        let DeleteFormInput { id, customer_id } = data;

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

        let _ = self.repo.delete_by_id(&id).await?;

        Ok(())
    }
}
