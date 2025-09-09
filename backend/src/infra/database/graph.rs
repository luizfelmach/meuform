use crate::{
    core::Graph,
    protocols::{CreateGraphRepository, GraphRepository},
};
use anyhow::Result;
use chrono::Utc;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

pub struct MongoGraphRepository {
    collection: Collection<Graph>,
}

impl MongoGraphRepository {
    pub fn new(collection: Collection<Graph>) -> Self {
        Self { collection }
    }
}

impl GraphRepository for MongoGraphRepository {
    async fn create(&self, data: CreateGraphRepository) -> Result<Graph> {
        let now = Utc::now();

        let graph = Graph {
            id: ObjectId::new().to_string(),
            nodes: data.nodes,
            edges: data.edges,
            start: data.start,
            end: data.end,
            created_at: now,
            updated_at: now,
        };

        let _ = self.collection.insert_one(graph.clone()).await?;

        Ok(graph)
    }

    async fn get_by_id(&self, id: &String) -> Result<Option<Graph>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let graph = self.collection.find_one(filter).await?;
        Ok(graph)
    }
}
