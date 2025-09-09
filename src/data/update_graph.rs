use crate::{
    protocols::{
        CreateGraphRepository, EdgeValidator, FormRepository, GraphRepository, GraphTopology,
    },
    usecase::{UpdateGraph, UpdateGraphInput, UpdateGraphOutput},
};
use anyhow::{Result, anyhow, bail};

pub struct UpdateGraphUseCase<F, G, GT, EV> {
    pub form: F,
    pub graph: G,
    pub graph_topology: GT,
    pub edge_validator: EV,
}

impl<F, G, GT, EV> UpdateGraphUseCase<F, G, GT, EV>
where
    F: FormRepository,
    G: GraphRepository,
    GT: GraphTopology,
    EV: EdgeValidator,
{
    pub fn new(form: F, graph: G, graph_topology: GT, edge_validator: EV) -> Self {
        Self {
            form,
            graph,
            graph_topology,
            edge_validator,
        }
    }
}

impl<F, G, GT, EV> UpdateGraph for UpdateGraphUseCase<F, G, GT, EV>
where
    F: FormRepository,
    G: GraphRepository,
    GT: GraphTopology,
    EV: EdgeValidator,
{
    async fn update(&self, data: UpdateGraphInput) -> Result<UpdateGraphOutput> {
        let mut form = self
            .form
            .get_by_id(&data.form_id)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        if form.customer_id != data.customer_id {
            bail!("Unauthorized")
        }

        if !self.graph_topology.is_dag(&data.nodes, &data.edges)? {
            bail!("Graph has cycle")
        }

        let _ = self.edge_validator.validate(&data.nodes, &data.edges)?;

        let start_node = self.graph_topology.start_node(&data.nodes, &data.edges)?;
        let end_nodes = self.graph_topology.end_nodes(&data.nodes, &data.edges)?;

        let graph = self
            .graph
            .create(CreateGraphRepository {
                nodes: data.nodes,
                edges: data.edges,
                start: start_node,
                end: end_nodes,
            })
            .await?;

        form.graph_id = graph.id.clone();
        let _ = self.form.update(form).await?;

        Ok(UpdateGraphOutput { graph })
    }
}
