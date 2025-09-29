use crate::{Paging, UseCaseResult};

use domain::{CustomerId, Form, FormId, GraphId};
use std::sync::Arc;

pub type DynCustomerCreateForm = Arc<dyn CustomerCreateForm>;
pub type DynCustomerGetForm = Arc<dyn CustomerGetForm>;
pub type DynCustomerUpdateForm = Arc<dyn CustomerUpdateForm>;
pub type DynCustomerDeleteForm = Arc<dyn CustomerDeleteForm>;
pub type DynCustomerListForms = Arc<dyn CustomerListForms>;

#[async_trait::async_trait]
pub trait CustomerCreateForm: Send + Sync {
    async fn execute(
        &self,
        name: &String,
        slug: &String,
        customer_id: &CustomerId,
        flow_id: &GraphId,
    ) -> UseCaseResult<Form>;
}

#[async_trait::async_trait]
pub trait CustomerGetForm: Send + Sync {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> UseCaseResult<Form>;
}

#[async_trait::async_trait]
pub trait CustomerUpdateForm: Send + Sync {
    async fn execute(
        &self,
        id: &FormId,
        customer_id: &CustomerId,
        data: UpdateFormInput,
    ) -> UseCaseResult<Form>;
}

#[async_trait::async_trait]
pub trait CustomerDeleteForm: Send + Sync {
    async fn execute(&self, id: &FormId, customer_id: &CustomerId) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerListForms: Send + Sync {
    async fn execute(
        &self,
        customer_id: &CustomerId,
        paging: Option<Paging>,
    ) -> UseCaseResult<Vec<Form>>;
}

pub struct UpdateFormInput {
    pub slug: Option<String>,
    pub name: Option<String>,
}
