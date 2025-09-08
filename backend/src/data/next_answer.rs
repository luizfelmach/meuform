use crate::{
    protocols::AnswerRepository,
    usecase::{NextAnswer, NextAnswerInput, NextAnswerOutput},
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

impl<A> NextAnswer for NextAnswerUseCase<A>
where
    A: AnswerRepository,
{
    async fn next(&self, data: NextAnswerInput) -> anyhow::Result<NextAnswerOutput> {
        let answer = self
            .answer
            .get_by_id(&data.answer_id)
            .await?
            .ok_or_else(|| anyhow!("Answer not found"))?;

        if answer.completed {
            bail!("Answer is completed")
        }

        // Todo: Implementar logica para avancar...

        Ok(NextAnswerOutput { completed: false })
    }
}
