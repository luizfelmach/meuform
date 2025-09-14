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
pub trait CreateForm: Send + Sync {
    async fn execute(
        &self,
        name: &String,
        slug: &String,
        customer_id: &CustomerId,
        flow_id: &FlowId,
    ) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait GetForm: Send + Sync {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait UpdateForm: Send + Sync {
    async fn execute(&self, data: UpdateFormInput) -> Result<Form>;
}

#[async_trait::async_trait]
pub trait DeleteForm: Send + Sync {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ListForms: Send + Sync {
    async fn execute(
        &self,
        customer_id: CustomerId,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait UpdateFormFlow: Send + Sync {
    async fn execute(&self, id: FormId, customer_id: CustomerId, flow_id: &FlowId) -> Result<Form>;
}

pub struct UpdateFormInput {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub slug: Option<String>,
    pub name: Option<String>,
}
