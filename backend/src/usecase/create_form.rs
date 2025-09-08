use crate::core::form::Form;
use anyhow::Result;

pub struct Input {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub owner_id: String,
}

pub struct Output {
    form: Form,
}

pub trait CreateForm {
    async fn create(&self, data: Input) -> Result<Output>;
}
