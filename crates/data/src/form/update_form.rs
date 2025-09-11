use anyhow::{Result, anyhow, bail};
use domain::{Customer, Form};
use protocols::{FindById, FindBySlug, Update};
use usecases::{UpdateForm, UpdateFormInput, UpdateFormOutput};

pub struct UpdateFormUseCase<R> {
    repo: R,
}

impl<R> UpdateFormUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> UpdateForm for UpdateFormUseCase<R>
where
    R: FindById<Customer> + FindById<Form> + FindBySlug<Form> + Update<Form>,
{
    async fn execute(&self, data: UpdateFormInput) -> Result<UpdateFormOutput> {
        let UpdateFormInput {
            id,
            customer_id,
            name,
            slug,
        } = data;

        let customer: Customer = self
            .repo
            .find_by_id(&customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        let mut form: Form = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Form does not exists"))?;

        if form.customer_id != customer.id {
            bail!("Unauthorized")
        }

        if let Some(name) = name {
            form.set_name(name);
        }

        if let Some(slug) = slug {
            let exists = self.repo.find_by_slug(&slug).await?;

            if let Some(_) = exists {
                bail!("Slug already exists")
            }

            form.set_slug(slug);
        }

        let form = self.repo.update(form).await?;

        Ok(UpdateFormOutput { form })
    }
}
