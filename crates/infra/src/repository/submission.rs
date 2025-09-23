use crate::MongoRepository;
use domain::{FormId, InfraError, Pagination, EvaluateAnswerResult, Submission, SubmissionId};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use protocols::SubmissionRepository;
use std::sync::Arc;

pub struct SubmissionRepositoryImpl {
    mongo: Arc<MongoRepository>,
}

impl SubmissionRepositoryImpl {
    pub fn new(mongo: Arc<MongoRepository>) -> Self {
        Self { mongo }
    }
}

#[async_trait::async_trait]
impl SubmissionRepository for SubmissionRepositoryImpl {
    async fn uuid(&self) -> EvaluateAnswerResult<SubmissionId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &SubmissionId) -> EvaluateAnswerResult<Option<Submission>> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        let data = self
            .mongo
            .submission
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn save(&self, data: &Submission) -> EvaluateAnswerResult<Submission> {
        let _ = self
            .mongo
            .submission
            .insert_one(data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn update(&self, data: &Submission) -> EvaluateAnswerResult<Submission> {
        let oid = ObjectId::parse_str(data.id.clone()).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": &oid };
        self.mongo
            .submission
            .replace_one(filter, data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &SubmissionId) -> EvaluateAnswerResult<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        self.mongo
            .submission
            .delete_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(())
    }

    async fn list(&self, form_id: &FormId, pag: Option<Pagination>) -> EvaluateAnswerResult<Vec<Submission>> {
        let Pagination { limit, offset } = pag.unwrap_or_default();
        let filter = doc! { "form_id": form_id };

        let cursor = self
            .mongo
            .submission
            .find(filter)
            .limit(limit as i64)
            .skip(offset as u64)
            .await
            .map_err(|_| InfraError::DatabaseError)?;

        let result = cursor
            .try_collect()
            .await
            .map_err(|_| InfraError::DatabaseError)?;

        Ok(result)
    }
}
