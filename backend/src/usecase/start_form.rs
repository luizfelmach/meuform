use crate::core::{AnswerId, Form};
use anyhow::Result;

pub struct Input {
    slug: String,
}

pub struct Output {
    form: Form,
    answer_id: AnswerId,
}

pub trait StartForm {
    async fn start(&self, data: Input) -> Result<Output>;
}
