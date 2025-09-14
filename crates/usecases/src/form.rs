use anyhow::Result;
use domain::{CustomerId, FlowId, Form, FormId};
use std::sync::Arc;

pub type DynCreateForm = Arc<dyn CreateForm>;
pub type DynGetForm = Arc<dyn GetForm>;
pub type DynUpdateForm = Arc<dyn UpdateForm>;
pub type DynDeleteForm = Arc<dyn DeleteForm>;
pub type DynListForms = Arc<dyn ListForms>;
pub type DynUpdateFormFlow = Arc<dyn UpdateFormFlow>;

#[async_trait::async_trait]
pub trait CreateForm {
    async fn execute(
        &self,
        name: &String,
        slug: &String,
        customer_id: &CustomerId,
        flow_id: &FlowId,
    ) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait GetForm {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait UpdateForm {
    async fn execute(&self, data: UpdateFormInput) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait DeleteForm {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListForms {
    async fn execute(
        &self,
        customer_id: CustomerId,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait UpdateFormFlow {
    async fn execute(&self, id: FormId, customer_id: CustomerId, flow_id: &FlowId) -> Result<()>;
}

pub struct UpdateFormInput {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub slug: Option<String>,
    pub name: Option<String>,
}
