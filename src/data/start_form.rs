use crate::{
    protocols::{AnswerRepository, CreateAnswerRepository, FormRepository, GraphRepository},
    usecase::{StartForm, StartFormInput, StartFormOutput},
};
use anyhow::anyhow;

pub struct StartFormUseCase<F, A, G> {
    pub form: F,
    pub answer: A,
    pub graph: G,
}

impl<F, A, G> StartFormUseCase<F, A, G>
where
    F: FormRepository,
    A: AnswerRepository,
    G: GraphRepository,
{
    pub fn new(form: F, answer: A, graph: G) -> Self {
        Self {
            form,
            answer,
            graph,
        }
    }
}

impl<F, A, G> StartForm for StartFormUseCase<F, A, G>
where
    F: FormRepository,
    A: AnswerRepository,
    G: GraphRepository,
{
    async fn start(&self, data: StartFormInput) -> anyhow::Result<StartFormOutput> {
        let form = self
            .form
            .get_by_slug(&data.slug)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        let graph = self
            .graph
            .get_by_id(&form.graph_id)
            .await?
            .ok_or_else(|| anyhow!("Graph not found"))?;

        let answer = self
            .answer
            .create(CreateAnswerRepository {
                form_id: form.id.clone(),
                graph_id: graph.id.clone(),
                current_node: graph.start,
            })
            .await?;

        Ok(StartFormOutput {
            answer_id: answer.id,
            form,
        })
    }
}
