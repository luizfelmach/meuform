use crate::{
    core::Form,
    protocols::{CreateFormRepository, FormRepository},
};
use anyhow::Result;
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

pub struct MongoFormRepository {
    collection: Collection<Form>,
}

impl MongoFormRepository {
    pub fn new(collection: Collection<Form>) -> Self {
        Self { collection }
    }
}

impl FormRepository for MongoFormRepository {
    async fn create(&self, data: CreateFormRepository) -> Result<Form> {
        let now = Utc::now();

        let form = Form {
            id: ObjectId::new().to_string(),
            slug: data.slug,
            title: data.title,
            description: data.description,
            customer_id: data.customer_id,
            graph_id: data.graph_id,
            created_at: now,
            updated_at: now,
        };

        let _ = self.collection.insert_one(form.clone()).await?;

        Ok(form)
    }

    async fn get_by_id(&self, id: &String) -> Result<Option<Form>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let form = self.collection.find_one(filter).await?;
        Ok(form)
    }

    async fn get_by_slug(&self, slug: &String) -> Result<Option<Form>> {
        let filter = doc! { "slug": slug };
        let form = self.collection.find_one(filter).await?;
        Ok(form)
    }

    async fn update(&self, data: Form) -> Result<Form> {
        let oid = ObjectId::parse_str(&data.id)?;
        let filter = doc! { "_id": &oid };
        self.collection.replace_one(filter, &data).await?;
        Ok(data)
    }

    async fn delete(&self, id: &String) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": &oid };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn list_by_customer(&self, customer_id: &String) -> Result<Vec<Form>> {
        let filter = doc! { "customer_id": customer_id };
        let mut cursor = self.collection.find(filter).skip(0).limit(100).await?;
        let mut forms = Vec::new();

        while let Some(form) = cursor.try_next().await? {
            forms.push(form);
        }

        Ok(forms)
    }
}
