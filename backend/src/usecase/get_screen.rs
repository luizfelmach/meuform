use crate::core::{AnswerId, Screen};
use anyhow::Result;

pub struct GetScreenInput {
    pub answer_id: AnswerId,
}

pub struct GetScreenOutput {
    pub screen: Screen,
}

pub trait GetScreen {
    async fn get(&self, data: GetScreenInput) -> Result<GetScreenOutput>;
}
