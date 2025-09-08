use crate::core::Customer;
use anyhow::Result;

pub struct CreateCustomer {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct UpdateCustomer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub trait CustomerRepository {
    fn create(&self, data: CreateCustomer) -> Result<Customer>;
    fn get_by_id(&self, id: &String) -> Result<Option<Customer>>;
    fn update(&self, data: UpdateCustomer) -> Result<Customer>;
    fn delete(&self, id: &String) -> Result<()>;
}
