use crate::MongoRepository;
use domain::{Flow, FlowId, InfraError, EvaluateAnswerResult};
use mongodb::bson::{doc, oid::ObjectId};
use protocols::FlowRepository;
use std::sync::Arc;

pub struct FlowRepositoryImpl {
    mongo: Arc<MongoRepository>,
}

impl FlowRepositoryImpl {
    pub fn new(mongo: Arc<MongoRepository>) -> Self {
        Self { mongo }
    }
}

#[async_trait::async_trait]
impl FlowRepository for FlowRepositoryImpl {
    async fn uuid(&self) -> EvaluateAnswerResult<FlowId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &FlowId) -> EvaluateAnswerResult<Option<Flow>> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        let data = self
            .mongo
            .flow
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn save(&self, data: &Flow) -> EvaluateAnswerResult<Flow> {
        let _ = self
            .mongo
            .flow
            .insert_one(data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &FlowId) -> EvaluateAnswerResult<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        self.mongo
            .flow
            .delete_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(())
    }
}
