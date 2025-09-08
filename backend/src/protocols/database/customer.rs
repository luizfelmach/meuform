use crate::core::Customer;
use anyhow::Result;

pub struct CreateCustomerRepository {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateCustomerRepository {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub trait CustomerRepository {
    async fn create(&self, data: CreateCustomerRepository) -> Result<Customer>;
    async fn get_by_id(&self, id: &String) -> Result<Option<Customer>>;
    async fn find_by_email(&self, email: &String) -> Result<Option<Customer>>;
    async fn update(&self, data: UpdateCustomerRepository) -> Result<Customer>;
    async fn delete(&self, id: &String) -> Result<()>;
}
