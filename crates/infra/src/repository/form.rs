use crate::MongoRepository;
use domain::{CustomerId, Form, FormId, InfraError, Pagination, EvaluateAnswerResult};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use protocols::FormRepository;
use std::sync::Arc;

pub struct FormRepositoryImpl {
    mongo: Arc<MongoRepository>,
}

impl FormRepositoryImpl {
    pub fn new(mongo: Arc<MongoRepository>) -> Self {
        Self { mongo }
    }
}

#[async_trait::async_trait]
impl FormRepository for FormRepositoryImpl {
    async fn uuid(&self) -> EvaluateAnswerResult<FormId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &FormId) -> EvaluateAnswerResult<Option<Form>> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        let data = self
            .mongo
            .form
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn find_by_slug(&self, slug: &String) -> EvaluateAnswerResult<Option<Form>> {
        let filter = doc! { "slug": slug };
        let data = self
            .mongo
            .form
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn save(&self, data: &Form) -> EvaluateAnswerResult<Form> {
        let _ = self
            .mongo
            .form
            .insert_one(data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn update(&self, data: &Form) -> EvaluateAnswerResult<Form> {
        let oid = ObjectId::parse_str(data.id.clone()).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": &oid };
        self.mongo
            .form
            .replace_one(filter, data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &FormId) -> EvaluateAnswerResult<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        self.mongo
            .form
            .delete_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(())
    }

    async fn list(&self, customer_id: &CustomerId, pag: Option<Pagination>) -> EvaluateAnswerResult<Vec<Form>> {
        let Pagination { limit, offset } = pag.unwrap_or_default();
        let filter = doc! { "customer_id": customer_id };

        let cursor = self
            .mongo
            .form
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
