use crate::core::{AnswerId, AnswerType};
use anyhow::Result;

pub struct SubmitAnswerInput {
    pub answer_id: AnswerId,
    pub answer: AnswerType,
}

pub trait SubmitAnswer {
    async fn submit(&self, data: SubmitAnswerInput) -> Result<()>;
}
