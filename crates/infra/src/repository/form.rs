use crate::MongoRepository;
use anyhow::Result;
use domain::{CustomerId, Form, FormId, Pagination};
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
    async fn uuid(&self) -> Result<FormId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &FormId) -> Result<Option<Form>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let data = self.mongo.form.find_one(filter).await?;
        Ok(data)
    }

    async fn find_by_slug(&self, slug: &String) -> Result<Option<Form>> {
        let filter = doc! { "slug": slug };
        let data = self.mongo.form.find_one(filter).await?;
        Ok(data)
    }

    async fn save(&self, data: &Form) -> Result<Form> {
        let _ = self.mongo.form.insert_one(data).await?;
        Ok(data.clone())
    }

    async fn update(&self, data: &Form) -> Result<Form> {
        let oid = ObjectId::parse_str(data.id.clone())?;
        let filter = doc! { "_id": &oid };
        self.mongo.form.replace_one(filter, data).await?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &FormId) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        self.mongo.form.delete_one(filter).await?;
        Ok(())
    }

    async fn list(&self, customer_id: &CustomerId, pag: Option<Pagination>) -> Result<Vec<Form>> {
        let Pagination { limit, offset } = pag.unwrap_or_default();
        let filter = doc! { "customer_id": customer_id };

        let cursor = self
            .mongo
            .form
            .find(filter)
            .limit(limit as i64)
            .skip(offset as u64)
            .await?;

        let result = cursor.try_collect().await?;
        Ok(result)
    }
}
