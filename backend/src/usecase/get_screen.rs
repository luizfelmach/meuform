use crate::core::{AnswerId, Screen};
use anyhow::Result;

pub struct Input {
    answer_id: AnswerId,
}

pub struct Output {
    screen: Screen,
}

pub trait GetScreen {
    async fn get(&self, data: Input) -> Result<Output>;
}
