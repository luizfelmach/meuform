use domain::{
    Customer, CustomerId, Flow, FlowId, Form, FormId, Pagination, Result, Submission, SubmissionId,
};
use std::sync::Arc;

pub type DynCustomerRepository = Arc<dyn CustomerRepository>;
pub type DynFormRepository = Arc<dyn FormRepository>;
pub type DynSubmissionRepository = Arc<dyn SubmissionRepository>;
pub type DynFlowRepository = Arc<dyn FlowRepository>;

#[async_trait::async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn uuid(&self) -> Result<CustomerId>;
    async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>>;
    async fn find_by_email(&self, email: &String) -> Result<Option<Customer>>;
    async fn save(&self, data: &Customer) -> Result<Customer>;
    async fn update(&self, data: &Customer) -> Result<Customer>;
    async fn delete(&self, id: &CustomerId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait FormRepository: Send + Sync {
    async fn uuid(&self) -> Result<FormId>;
    async fn find_by_id(&self, id: &FormId) -> Result<Option<Form>>;
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Form>>;
    async fn save(&self, data: &Form) -> Result<Form>;
    async fn update(&self, data: &Form) -> Result<Form>;
    async fn delete(&self, id: &FormId) -> Result<()>;
    async fn list(&self, customer_id: &CustomerId, pag: Option<Pagination>) -> Result<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn uuid(&self) -> Result<SubmissionId>;
    async fn find_by_id(&self, id: &SubmissionId) -> Result<Option<Submission>>;
    async fn save(&self, data: &Submission) -> Result<Submission>;
    async fn update(&self, data: &Submission) -> Result<Submission>;
    async fn delete(&self, id: &SubmissionId) -> Result<()>;
    async fn list(&self, form_id: &FormId, pag: Option<Pagination>) -> Result<Vec<Submission>>;
}

#[async_trait::async_trait]
pub trait FlowRepository: Send + Sync {
    async fn uuid(&self) -> Result<FlowId>;
    async fn find_by_id(&self, id: &FlowId) -> Result<Option<Flow>>;
    async fn save(&self, data: &Flow) -> Result<Flow>;
    async fn delete(&self, id: &FlowId) -> Result<()>;
}
