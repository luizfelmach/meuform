use anyhow::Result;
use domain::{CustomerId, Form, FormId};

#[async_trait::async_trait]
pub trait FormUseCase {
    async fn create(&self, data: CreateFormInput) -> Result<CreateFormOutput>;
    async fn get(&self, data: GetFormInput) -> Result<GetFormOutput>;
    async fn update(&self, data: UpdateFormInput) -> Result<UpdateFormOutput>;
    async fn delete(&self, data: DeleteFormInput) -> Result<()>;
}

pub struct CreateFormInput {
    pub customer_id: CustomerId,
    pub title: String,
    pub slug: String,
}

pub struct CreateFormOutput {
    pub form: Form,
}

pub struct GetFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub struct GetFormOutput {
    pub form: Form,
}

pub struct UpdateFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
}

pub struct UpdateFormOutput {
    pub form: Form,
}

pub struct DeleteFormInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}
