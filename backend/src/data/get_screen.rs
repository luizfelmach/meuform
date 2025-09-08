use crate::{
    protocols::{AnswerRepository, GraphRepository},
    usecase::{GetScreen, GetScreenInput, GetScreenOutput},
};
use anyhow::anyhow;

pub struct GetScreenUseCase<G, A> {
    pub graph: G,
    pub answer: A,
}

impl<G, A> GetScreenUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    pub fn new(graph: G, answer: A) -> Self {
        Self { graph, answer }
    }
}

impl<G, A> GetScreen for GetScreenUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    async fn get(&self, data: GetScreenInput) -> anyhow::Result<GetScreenOutput> {
        let answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        let graph = self
            .graph
            .get_by_id(&answer.graph_id)
            .await?
            .ok_or_else(|| anyhow!("Graph not found"))?;

        let node = graph
            .nodes
            .get(&answer.current_node)
            .ok_or_else(|| anyhow!("Node not found"))?;

        Ok(GetScreenOutput {
            screen: node.screen.clone(),
        })
    }
}
