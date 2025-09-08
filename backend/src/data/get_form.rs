use crate::{
    protocols::{FormRepository, GraphRepository},
    usecase::{GetForm, GetFormInput, GetFormOutput},
};
use anyhow::{Result, anyhow, bail};

pub struct GetFormUseCase<F, G> {
    pub form: F,
    pub graph: G,
}

impl<F, G> GetFormUseCase<F, G>
where
    F: FormRepository,
    G: GraphRepository,
{
    pub fn new(form: F, graph: G) -> Self {
        Self { form, graph }
    }
}

impl<F, G> GetForm for GetFormUseCase<F, G>
where
    F: FormRepository,
    G: GraphRepository,
{
    async fn get(&self, data: GetFormInput) -> Result<GetFormOutput> {
        let form = self
            .form
            .get_by_id(&data.form_id)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        if form.customer_id != data.customer_id {
            bail!("Unauthorized")
        }

        let graph = self
            .graph
            .get_by_id(&form.id)
            .await?
            .ok_or_else(|| anyhow!("Graph not found"))?;

        Ok(GetFormOutput { form, graph })
    }
}
