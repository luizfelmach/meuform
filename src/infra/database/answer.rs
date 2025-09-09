use crate::{
    core::Answer,
    protocols::{AnswerRepository, CreateAnswerRepository},
};
use anyhow::Result;
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use std::collections::HashMap;

pub struct MongoAnswerRepository {
    collection: Collection<Answer>,
}

impl MongoAnswerRepository {
    pub fn new(collection: Collection<Answer>) -> Self {
        Self { collection }
    }
}

impl AnswerRepository for MongoAnswerRepository {
    async fn create(&self, data: CreateAnswerRepository) -> Result<Answer> {
        let now = Utc::now();

        let answer = Answer {
            id: ObjectId::new().to_string(),
            form_id: data.form_id,
            graph_id: data.graph_id,
            current_node: data.current_node,
            completed: false,
            history: Vec::new(),
            responses: HashMap::new(),
            created_at: now,
            updated_at: now,
        };

        let _ = self.collection.insert_one(answer.clone()).await?;

        Ok(answer)
    }

    async fn get_by_id(&self, id: &String) -> Result<Option<Answer>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let answer = self.collection.find_one(filter).await?;
        Ok(answer)
    }

    async fn update(&self, data: Answer) -> Result<Answer> {
        let oid = ObjectId::parse_str(data.id.clone())?;
        let filter = doc! { "_id": &oid };
        self.collection.replace_one(filter, &data).await?;
        Ok(data)
    }

    async fn list_by_form(&self, form_id: &String) -> Result<Vec<Answer>> {
        let filter = doc! { "form_id": form_id };

        let mut cursor = self.collection.find(filter).skip(0).limit(1000).await?;
        let mut answers = Vec::new();

        while let Some(answer) = cursor.try_next().await? {
            answers.push(answer);
        }

        Ok(answers)
    }
}
