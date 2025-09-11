use anyhow::{Result, anyhow, bail};
use domain::{Customer, Flow, Form, Graph};
use protocols::{FindById, FindBySlug, GenerateUuid, GraphValidate, Save};
use usecases::{CreateForm, CreateFormInput, CreateFormOutput};

pub struct CreateFormUseCase<R, G> {
    repo: R,
    graph: G,
}

impl<R, G> CreateFormUseCase<R, G> {
    pub fn new(repo: R, graph: G) -> Self {
        Self { repo, graph }
    }
}

#[async_trait::async_trait]
impl<R, G> CreateForm for CreateFormUseCase<R, G>
where
    R: FindById<Customer> + FindBySlug<Form> + Save<Form> + Save<Flow> + GenerateUuid,
    G: GraphValidate,
{
    async fn execute(&self, data: CreateFormInput) -> Result<CreateFormOutput> {
        let CreateFormInput {
            customer_id,
            name,
            slug,
            nodes,
            edges,
        } = data;

        let customer = self
            .repo
            .find_by_id(&customer_id)
            .await?
            .ok_or_else(|| anyhow!("Customer does not exists"))?;

        let form = self.repo.find_by_slug(&slug).await?;

        if let Some(_) = form {
            bail!("Slug already in use")
        }

        if self.graph.has_cycle(&nodes, &edges)? {
            bail!("Graph has cycle")
        }

        let start = self.graph.start_node(&nodes, &edges)?;
        let end = self.graph.end_nodes(&nodes, &edges)?;
        let graph = Graph::init(nodes, edges, start, end)?;

        let flow_id = self.repo.generate_uuid()?;
        let flow = Flow::new(flow_id.clone(), customer_id, graph);

        let _ = self.repo.save(flow).await?;

        let id = self.repo.generate_uuid()?;
        let form = Form::new(id, slug, name, customer.id, flow_id);

        let form = self.repo.save(form).await?;

        Ok(CreateFormOutput { form })
    }
}
