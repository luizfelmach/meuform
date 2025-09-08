use crate::core::{Answer, AnswerId, CustomerId, FormId, Graph};
use anyhow::Result;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub answer_id: AnswerId,
}

pub struct Output {
    graph: Graph,
    answer: Answer,
}

pub trait GetAnswer {
    async fn get(&self, data: Input) -> Result<Output>;
}
