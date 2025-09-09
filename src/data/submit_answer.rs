use crate::{
    protocols::AnswerRepository,
    usecase::{SubmitAnswer, SubmitAnswerInput},
};
use anyhow::{anyhow, bail};

pub struct SubmitAnswerUseCase<A> {
    pub answer: A,
}

impl<A> SubmitAnswerUseCase<A>
where
    A: AnswerRepository,
{
    pub fn new(answer: A) -> Self {
        Self { answer }
    }
}

impl<A> SubmitAnswer for SubmitAnswerUseCase<A>
where
    A: AnswerRepository,
{
    async fn submit(&self, data: SubmitAnswerInput) -> anyhow::Result<()> {
        let mut answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        if answer.completed {
            bail!("Answer is completed")
        }

        let _ = answer.responses.insert(answer.current_node, data.answer);
        let _ = self.answer.update(answer).await?;

        Ok(())
    }
}
