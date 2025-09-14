use anyhow::Result;
use domain::{CustomerId, CustomerWithoutPassword};
use std::sync::Arc;

pub type DynAuthCustomer = Arc<dyn AuthCustomer>;
pub type DynCreateCustomer = Arc<dyn CreateCustomer>;
pub type DynGetCustomer = Arc<dyn GetCustomer>;
pub type DynUpdateCustomer = Arc<dyn UpdateCustomer>;
pub type DynDeleteCustomer = Arc<dyn DeleteCustomer>;
pub type DynForgotPasswordCustomer = Arc<dyn ForgotPasswordCustomer>;
pub type DynUpdatePasswordCustomer = Arc<dyn UpdatePasswordCustomer>;
pub type DynAuthenticatedCustomer = Arc<dyn AuthenticatedCustomer>;

#[async_trait::async_trait]
pub trait AuthCustomer: Send + Sync {
    async fn execute(&self, email: &String, password: &String) -> Result<String>;
}

#[async_trait::async_trait]
pub trait CreateCustomer: Send + Sync {
    async fn execute(
        &self,
        name: &String,
        email: &String,
        password: &String,
    ) -> Result<CustomerWithoutPassword>;
}

#[async_trait::async_trait]
pub trait GetCustomer: Send + Sync {
    async fn execute(&self, id: &CustomerId) -> Result<CustomerWithoutPassword>;
}

#[async_trait::async_trait]
pub trait UpdateCustomer: Send + Sync {
    async fn execute(&self, data: UpdateCustomerInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait DeleteCustomer: Send + Sync {
    async fn execute(&self, id: &CustomerId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ForgotPasswordCustomer: Send + Sync {
    async fn execute(&self, id: &CustomerId) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UpdatePasswordCustomer: Send + Sync {
    async fn execute(&self, token: &String, password: &String) -> Result<CustomerWithoutPassword>;
}

#[async_trait::async_trait]
pub trait AuthenticatedCustomer: Send + Sync {
    async fn execute(&self, token: &String) -> Result<CustomerId>;
}

pub struct UpdateCustomerInput {
    pub id: CustomerId,
    pub name: Option<String>,
    pub email: Option<String>,
}
