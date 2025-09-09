use crate::{
    protocols::{AnswerRepository, GraphRepository},
    usecase::{GetAnswer, GetAnswerInput, GetAnswerOutput},
};
use anyhow::{Ok, anyhow};

pub struct GetAnswerUseCase<G, A> {
    pub graph: G,
    pub answer: A,
}

impl<G, A> GetAnswerUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    pub fn new(graph: G, answer: A) -> Self {
        Self { graph, answer }
    }
}

impl<G, A> GetAnswer for GetAnswerUseCase<G, A>
where
    G: GraphRepository,
    A: AnswerRepository,
{
    async fn get(&self, data: GetAnswerInput) -> anyhow::Result<GetAnswerOutput> {
        let answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        let graph = self
            .graph
            .get_by_id(&answer.id)
            .await?
            .ok_or_else(|| anyhow!("Graph not found"))?;

        Ok(GetAnswerOutput { answer, graph })
    }
}
