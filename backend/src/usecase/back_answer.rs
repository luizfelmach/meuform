use crate::core::AnswerId;
use anyhow::Result;

pub struct BackAnswerInput {
    pub answer_id: AnswerId,
}

pub trait BackAnswer {
    async fn back(&self, data: BackAnswerInput) -> Result<()>;
}
