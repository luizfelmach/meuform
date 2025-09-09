use crate::core::{AnswerId, Form};
use anyhow::Result;

pub struct StartFormInput {
    pub slug: String,
}

pub struct StartFormOutput {
    pub form: Form,
    pub answer_id: AnswerId,
}

pub trait StartForm {
    async fn start(&self, data: StartFormInput) -> Result<StartFormOutput>;
}
