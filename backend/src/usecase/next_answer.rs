use crate::core::AnswerId;
use anyhow::Result;

pub struct NextAnswerInput {
    pub answer_id: AnswerId,
}

pub struct NextAnswerOutput {
    pub completed: bool,
}

pub trait NextAnswer {
    async fn next(&self, data: NextAnswerInput) -> Result<NextAnswerOutput>;
}
