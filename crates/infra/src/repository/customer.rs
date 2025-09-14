use crate::MongoRepository;
use domain::{Customer, CustomerId, InfraError, Result};
use mongodb::bson::{doc, oid::ObjectId};
use protocols::CustomerRepository;
use std::sync::Arc;

pub struct CustomerRepositoryImpl {
    mongo: Arc<MongoRepository>,
}

impl CustomerRepositoryImpl {
    pub fn new(mongo: Arc<MongoRepository>) -> Self {
        Self { mongo }
    }
}

#[async_trait::async_trait]
impl CustomerRepository for CustomerRepositoryImpl {
    async fn uuid(&self) -> Result<CustomerId> {
        Ok(ObjectId::new().to_string())
    }

    async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        let data = self
            .mongo
            .customer
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn find_by_email(&self, email: &String) -> Result<Option<Customer>> {
        let filter = doc! { "email": email };
        let data = self
            .mongo
            .customer
            .find_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data)
    }

    async fn save(&self, data: &Customer) -> Result<Customer> {
        let _ = self
            .mongo
            .customer
            .insert_one(data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn update(&self, data: &Customer) -> Result<Customer> {
        let oid = ObjectId::parse_str(data.id.clone()).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": &oid };
        self.mongo
            .customer
            .replace_one(filter, data)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(data.clone())
    }

    async fn delete(&self, id: &CustomerId) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InfraError::UuidParseError)?;
        let filter = doc! { "_id": oid };
        self.mongo
            .customer
            .delete_one(filter)
            .await
            .map_err(|_| InfraError::DatabaseError)?;
        Ok(())
    }
}
