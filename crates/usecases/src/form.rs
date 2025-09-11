use anyhow::Result;
use domain::{CustomerId, Edges, Flow, Form, FormId, Nodes};

#[async_trait::async_trait]
pub trait CreateForm {
    async fn execute(&self, data: CreateFormInput) -> Result<CreateFormOutput>;
}

#[async_trait::async_trait]
pub trait GetForm {
    async fn execute(&self, data: GetFormInput) -> Result<GetFormOutput>;
}

#[async_trait::async_trait]
pub trait UpdateForm {
    async fn execute(&self, data: UpdateFormInput) -> Result<UpdateFormOutput>;
}

#[async_trait::async_trait]
pub trait DeleteForm {
    async fn execute(&self, data: DeleteFormInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListForms {
    async fn execute(&self, data: ListFormsInput) -> Result<ListFormsOutput>;
}

#[async_trait::async_trait]
pub trait UpdateFormFlow {
    async fn execute(&self, data: UpdateFormFlowInput) -> Result<UpdateFormFlowOutput>;
}

pub struct CreateFormInput {
    pub customer_id: CustomerId,
    pub name: String,
    pub slug: String,
    pub nodes: Nodes,
    pub edges: Edges,
}

pub struct CreateFormOutput {
    pub form: Form,
}

pub struct GetFormInput {
    pub id: FormId,
    pub customer_id: CustomerId,
}

pub struct GetFormOutput {
    pub form: Form,
}

pub struct UpdateFormInput {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub slug: Option<String>,
    pub name: Option<String>,
}

pub struct UpdateFormOutput {
    pub form: Form,
}

pub struct DeleteFormInput {
    pub id: FormId,
    pub customer_id: CustomerId,
}

pub struct ListFormsInput {
    pub customer_id: CustomerId,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub struct ListFormsOutput {
    pub forms: Vec<Form>,
}

pub struct UpdateFormFlowInput {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub nodes: Nodes,
    pub edges: Edges,
}

pub struct UpdateFormFlowOutput {
    pub flow: Flow,
}
