use crate::core::{
    answer::{Answer, AnswerType},
    customer::CustomerId,
    form::FormId,
    graph::{Graph, NodeId},
};
use anyhow::Result;
use std::collections::HashMap;

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub struct Output {
    pub answers: Vec<Answer>,
}

pub trait ListAnswers {
    async fn list(&self, data: Input) -> Result<Output>;
}
