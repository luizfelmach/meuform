use crate::{
    protocols::FormRepository,
    usecase::{ListForms, ListFormsInput, ListFormsOutput},
};
use anyhow::Result;

pub struct ListFormsUseCase<F> {
    pub form: F,
}

impl<F> ListFormsUseCase<F>
where
    F: FormRepository,
{
    pub fn new(form: F) -> Self {
        Self { form }
    }
}

impl<F> ListForms for ListFormsUseCase<F>
where
    F: FormRepository,
{
    async fn list(&self, data: ListFormsInput) -> Result<ListFormsOutput> {
        let forms = self.form.list_by_customer(&data.customer_id).await?;
        Ok(ListFormsOutput { forms })
    }
}
