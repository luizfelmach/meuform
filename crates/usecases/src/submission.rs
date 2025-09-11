use anyhow::Result;
use domain::{Answer, CustomerId, FormId, Screen, Submission, SubmissionId};

#[async_trait::async_trait]
pub trait StartSubmission {
    async fn execute(&self, data: StartSubmissionInput) -> Result<StartSubmissionOutput>;
}

#[async_trait::async_trait]
pub trait SubmitAnswerSubmission {
    async fn execute(&self, data: SubmitAnswerSubmissionInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GoNextSubmission {
    async fn execute(&self, data: GoNextSubmissionInput) -> Result<GoNextSubmissionOutput>;
}

#[async_trait::async_trait]
pub trait ScreenSubmission {
    async fn execute(&self, data: ScreenSubmissionInput) -> Result<ScreenSubmissionOutput>;
}

#[async_trait::async_trait]
pub trait GoBackSubmission {
    async fn execute(&self, data: GoBackSubmissionInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait CanGoBackSubmission {
    async fn execute(&self, data: CanGoBackSubmissionInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait CanGoNextSubmission {
    async fn execute(&self, data: CanGoNextSubmissionInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GetSubmission {
    async fn execute(&self, data: GetSubmissionInput) -> Result<GetSubmissionOutput>;
}

#[async_trait::async_trait]
pub trait ListSubmissions {
    async fn execute(&self, data: ListSubmissionsInput) -> Result<ListSubmissionsOutput>;
}

pub struct StartSubmissionInput {
    pub slug: String,
}

pub struct StartSubmissionOutput {
    pub submission: Submission,
}

pub struct SubmitAnswerSubmissionInput {
    pub id: SubmissionId,
    pub answer: Answer,
}

pub struct GoNextSubmissionInput {
    pub id: SubmissionId,
}

pub struct GoNextSubmissionOutput {
    pub completed: bool,
}

pub struct ScreenSubmissionInput {
    pub id: SubmissionId,
}

pub struct ScreenSubmissionOutput {
    pub screen: Screen,
}

pub struct GoBackSubmissionInput {
    pub id: SubmissionId,
}

pub struct CanGoBackSubmissionInput {
    pub id: SubmissionId,
}

pub struct CanGoNextSubmissionInput {
    pub id: SubmissionId,
}

pub struct GetSubmissionInput {
    pub id: SubmissionId,
    pub customer_id: CustomerId,
}

pub struct GetSubmissionOutput {
    pub submission: Submission,
}

pub struct ListSubmissionsInput {
    pub customer_id: CustomerId,
    pub form_id: FormId,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub struct ListSubmissionsOutput {
    pub submissions: Vec<Submission>,
}
