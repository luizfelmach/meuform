use crate::core::answer::{AnswerId, AnswerType};
use anyhow::Result;

pub struct Input {
    answer_id: AnswerId,
    answer: AnswerType,
}

pub struct Output {
    completed: bool,
}

pub trait SubmitAnswer {
    async fn submit(&self, data: Input) -> Result<Output>;
}
