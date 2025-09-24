use crate::{Paging, UseCaseResult};

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
pub trait StartSubmission: Send + Sync {
    async fn execute(&self, slug: &String) -> UseCaseResult<Submission>;
}

#[async_trait::async_trait]
pub trait SubmitAnswerSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId, answer: &Answer) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait GoNextSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait GoBackSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<()>;
}

#[async_trait::async_trait]
pub trait IsCompletedSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<bool>;
}

#[async_trait::async_trait]
pub trait ScreenSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<Screen>;
}

#[async_trait::async_trait]
pub trait CanGoBackSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<bool>;
}

#[async_trait::async_trait]
pub trait CanGoNextSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId) -> UseCaseResult<bool>;
}

#[async_trait::async_trait]
pub trait GetSubmission: Send + Sync {
    async fn execute(&self, id: &SubmissionId, form_id: &FormId) -> UseCaseResult<Submission>;
}

#[async_trait::async_trait]
pub trait ListSubmissions: Send + Sync {
    async fn execute(
        &self,
        form_id: &FormId,
        customer_id: &CustomerId,
        paging: Option<Paging>,
    ) -> UseCaseResult<Vec<Submission>>;
}
