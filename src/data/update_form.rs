use crate::{
    protocols::FormRepository,
    usecase::{UpdateForm, UpdateFormInput, UpdateFormOutput},
};
use anyhow::{Result, anyhow, bail};

pub struct UpdateFormUseCase<F> {
    pub form: F,
}

impl<F> UpdateFormUseCase<F>
where
    F: FormRepository,
{
    pub fn new(form: F) -> Self {
        Self { form }
    }
}

impl<F> UpdateForm for UpdateFormUseCase<F>
where
    F: FormRepository,
{
    async fn update(&self, data: UpdateFormInput) -> Result<UpdateFormOutput> {
        let mut form = self
            .form
            .get_by_id(&data.form_id)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        if form.customer_id != data.customer_id {
            bail!("Unauthorized")
        }

        if let Some(slug) = data.slug {
            form.slug = slug;
        }

        if let Some(title) = data.title {
            form.title = title
        }

        if let Some(description) = data.description {
            form.description = description
        }

        let form = self.form.update(form).await?;

        Ok(UpdateFormOutput { form })
    }
}
