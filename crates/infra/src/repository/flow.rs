use crate::MongoRepository;
use anyhow::Result;
use domain::{Flow, FlowId};
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
    async fn uuid(&self) -> Result<FlowId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &FlowId) -> Result<Option<Flow>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.mongo.flow.find_one(filter).await?;
        Ok(data)
    }

    async fn save(&self, data: &Flow) -> Result<Flow> {
        let _ = self.mongo.flow.insert_one(data).await?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &FlowId) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.mongo.flow.delete_one(filter).await?;
        Ok(())
    }
}
