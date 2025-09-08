use crate::core::answer::AnswerId;
use anyhow::Result;

pub struct Input {
    answer_id: AnswerId,
}

pub trait BackAnswer {
    async fn back(&self, data: Input) -> Result<()>;
}
