use crate::core::{Answer, CustomerId, FormId};
use anyhow::Result;

pub struct ListAnswersInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub struct ListAnswersOutput {
    pub answers: Vec<Answer>,
}

pub trait ListAnswers {
    async fn list(&self, data: ListAnswersInput) -> Result<ListAnswersOutput>;
}
