use crate::{
    protocols::{AnswerRepository, GraphRepository},
    usecase::{BackAnswer, BackAnswerInput},
};
use anyhow::{anyhow, bail};

pub struct NextAnswerUseCase<A> {
    pub answer: A,
}

impl<A> NextAnswerUseCase<A>
where
    A: AnswerRepository,
{
    pub fn new(answer: A) -> Self {
        Self { answer }
    }
}

impl<A> BackAnswer for NextAnswerUseCase<A>
where
    A: AnswerRepository,
{
    async fn back(&self, data: BackAnswerInput) -> anyhow::Result<()> {
        let mut answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        if answer.completed {
            bail!("Answer is completed")
        }

        let last = answer
            .history
            .pop()
            .ok_or_else(|| anyhow!("No previous steps to go back to"))?;

        answer.current_node = last;

        let _ = self.answer.update(answer).await?;

        Ok(())
    }
}
