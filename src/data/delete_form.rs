use crate::{
    protocols::FormRepository,
    usecase::{DeleteForm, DeleteFormInput},
};
use anyhow::{Result, anyhow, bail};

pub struct DeleteFormUseCase<F> {
    pub form: F,
}

impl<F> DeleteFormUseCase<F>
where
    F: FormRepository,
{
    pub fn new(form: F) -> Self {
        Self { form }
    }
}

impl<F> DeleteForm for DeleteFormUseCase<F>
where
    F: FormRepository,
{
    async fn delete(&self, data: DeleteFormInput) -> Result<()> {
        let form = self
            .form
            .get_by_id(&data.form_id)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        if form.customer_id != data.customer_id {
            bail!("Unauthorized")
        }

        let _ = self.form.delete(&form.id).await?;

        Ok(())
    }
}
