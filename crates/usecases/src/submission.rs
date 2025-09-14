use anyhow::Result;
use domain::{Answer, CustomerId, FormId, Screen, Submission, SubmissionId};
use std::sync::Arc;

pub type DynStartSubmission = Arc<dyn StartSubmission>;
pub type DynSubmitAnswerSubmission = Arc<dyn SubmitAnswerSubmission>;
pub type DynGoNextSubmission = Arc<dyn GoNextSubmission>;
pub type DynGoBackSubmission = Arc<dyn GoBackSubmission>;
pub type DynIsCompletedSubmission = Arc<dyn IsCompletedSubmission>;
pub type DynScreenSubmission = Arc<dyn ScreenSubmission>;
pub type DynCanGoBackSubmission = Arc<dyn CanGoBackSubmission>;
pub type DynCanGoNextSubmission = Arc<dyn CanGoNextSubmission>;
pub type DynGetSubmission = Arc<dyn GetSubmission>;
pub type DynListSubmissions = Arc<dyn ListSubmissions>;

#[async_trait::async_trait]
pub trait StartSubmission {
    async fn execute(&self, slug: &String) -> Result<Submission>;
}

#[async_trait::async_trait]
pub trait SubmitAnswerSubmission {
    async fn execute(&self, id: &SubmissionId, answer: Answer) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GoNextSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GoBackSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait IsCompletedSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<bool>;
}

#[async_trait::async_trait]
pub trait ScreenSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<Screen>;
}

#[async_trait::async_trait]
pub trait CanGoBackSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<bool>;
}

#[async_trait::async_trait]
pub trait CanGoNextSubmission {
    async fn execute(&self, id: &SubmissionId) -> Result<bool>;
}

#[async_trait::async_trait]
pub trait GetSubmission {
    async fn execute(&self, id: &SubmissionId, customer_id: &CustomerId) -> Result<Submission>;
}

#[async_trait::async_trait]
pub trait ListSubmissions {
    async fn execute(
        &self,
        form_id: &FormId,
        customer_id: &CustomerId,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Submission>>;
}
