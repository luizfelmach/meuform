use crate::{
    core::Customer,
    protocols::{CreateCustomerRepository, CustomerRepository},
};
use anyhow::Result;
use chrono::Utc;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};

pub struct MongoCustomerRepository {
    collection: Collection<Customer>,
}

impl MongoCustomerRepository {
    pub fn new(collection: Collection<Customer>) -> Self {
        Self { collection }
    }
}

impl CustomerRepository for MongoCustomerRepository {
    async fn create(&self, data: CreateCustomerRepository) -> Result<Customer> {
        let now = Utc::now();

        let customer = Customer {
            id: ObjectId::new().to_string(),
            name: data.name,
            email: data.email,
            password: data.password,
            created_at: now,
            updated_at: now,
        };

        let _ = self.collection.insert_one(customer.clone()).await?;

        Ok(customer)
    }

    async fn get_by_id(&self, id: &String) -> Result<Option<Customer>> {
        let oid = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": oid };
        let customer = self.collection.find_one(filter).await?;
        Ok(customer)
    }

    async fn find_by_email(&self, email: &String) -> Result<Option<Customer>> {
        let filter = doc! { "email": email };
        let customer = self.collection.find_one(filter).await?;
        Ok(customer)
    }

    async fn update(&self, data: Customer) -> Result<Customer> {
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
}
