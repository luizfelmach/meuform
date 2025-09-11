use anyhow::{Result, anyhow};
use domain::Customer;
use protocols::{FindById, ListForms as ListFormsTrait};
use usecases::{ListForms, ListFormsInput, ListFormsOutput};

pub struct ListFormsUseCase<R> {
    repo: R,
}

impl<R> ListFormsUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> ListForms for ListFormsUseCase<R>
where
    R: FindById<Customer> + ListFormsTrait,
{
    async fn execute(&self, data: ListFormsInput) -> Result<ListFormsOutput> {
        let ListFormsInput {
            customer_id,
            limit,
            offset,
        } = data;

        let _ = self
            .repo
            .find_by_id(&customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        let forms = self.repo.list_forms(&customer_id, limit, offset).await?;

        Ok(ListFormsOutput { forms })
    }
}
