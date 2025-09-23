use domain::{
    Customer, CustomerId, Flow, FlowId, Form, FormId, Pagination, EvaluateAnswerResult, Submission, SubmissionId,
};
use std::sync::Arc;

pub type DynCustomerRepository = Arc<dyn CustomerRepository>;
pub type DynFormRepository = Arc<dyn FormRepository>;
pub type DynSubmissionRepository = Arc<dyn SubmissionRepository>;
pub type DynFlowRepository = Arc<dyn FlowRepository>;

#[async_trait::async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn uuid(&self) -> EvaluateAnswerResult<CustomerId>;
    async fn find_by_id(&self, id: &CustomerId) -> EvaluateAnswerResult<Option<Customer>>;
    async fn find_by_email(&self, email: &String) -> EvaluateAnswerResult<Option<Customer>>;
    async fn save(&self, data: &Customer) -> EvaluateAnswerResult<Customer>;
    async fn update(&self, data: &Customer) -> EvaluateAnswerResult<Customer>;
    async fn delete(&self, id: &CustomerId) -> EvaluateAnswerResult<()>;
}

#[async_trait::async_trait]
pub trait FormRepository: Send + Sync {
    async fn uuid(&self) -> EvaluateAnswerResult<FormId>;
    async fn find_by_id(&self, id: &FormId) -> EvaluateAnswerResult<Option<Form>>;
    async fn find_by_slug(&self, slug: &String) -> EvaluateAnswerResult<Option<Form>>;
    async fn save(&self, data: &Form) -> EvaluateAnswerResult<Form>;
    async fn update(&self, data: &Form) -> EvaluateAnswerResult<Form>;
    async fn delete(&self, id: &FormId) -> EvaluateAnswerResult<()>;
    async fn list(&self, customer_id: &CustomerId, pag: Option<Pagination>) -> EvaluateAnswerResult<Vec<Form>>;
}

#[async_trait::async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn uuid(&self) -> EvaluateAnswerResult<SubmissionId>;
    async fn find_by_id(&self, id: &SubmissionId) -> EvaluateAnswerResult<Option<Submission>>;
    async fn save(&self, data: &Submission) -> EvaluateAnswerResult<Submission>;
    async fn update(&self, data: &Submission) -> EvaluateAnswerResult<Submission>;
    async fn delete(&self, id: &SubmissionId) -> EvaluateAnswerResult<()>;
    async fn list(&self, form_id: &FormId, pag: Option<Pagination>) -> EvaluateAnswerResult<Vec<Submission>>;
}

#[async_trait::async_trait]
pub trait FlowRepository: Send + Sync {
    async fn uuid(&self) -> EvaluateAnswerResult<FlowId>;
    async fn find_by_id(&self, id: &FlowId) -> EvaluateAnswerResult<Option<Flow>>;
    async fn save(&self, data: &Flow) -> EvaluateAnswerResult<Flow>;
    async fn delete(&self, id: &FlowId) -> EvaluateAnswerResult<()>;
}
