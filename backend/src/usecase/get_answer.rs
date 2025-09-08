use crate::core::{Answer, AnswerId, CustomerId, FormId, Graph};
use anyhow::Result;

pub struct GetAnswerInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub answer_id: AnswerId,
}

pub struct GetAnswerOutput {
    pub graph: Graph,
    pub answer: Answer,
}

pub trait GetAnswer {
    async fn get(&self, data: GetAnswerInput) -> Result<GetAnswerOutput>;
}
