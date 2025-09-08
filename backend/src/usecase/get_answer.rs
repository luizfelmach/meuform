use crate::core::{
    answer::{Answer, AnswerId},
    graph::Graph,
};
use anyhow::Result;

pub struct Input {
    pub answer_id: AnswerId,
}

pub struct Output {
    graph: Graph,
    answer: Answer,
}

pub trait GetAnswer {
    async fn get(&self, data: Input) -> Result<Output>;
}
