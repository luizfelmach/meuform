use domain::{Customer, CustomerId};
use std::sync::Arc;

pub type DynAuthenticateCustomer = Arc<dyn AuthenticateCustomer>;
pub type DynCreateCustomer = Arc<dyn CreateCustomer>;
pub type DynCustomerRetrieveProfile = Arc<dyn CustomerRetrieveProfile>;
pub type DynCustomerUpdateProfile = Arc<dyn CustomerUpdateProfile>;
pub type DynCustomerDeleteAccount = Arc<dyn CustomerDeleteAccount>;
pub type DynCustomerRequestPasswordReset = Arc<dyn CustomerRequestPasswordReset>;
pub type DynCustomerUpdatePassword = Arc<dyn CustomerUpdatePassword>;
pub type DynCustomerValidateToken = Arc<dyn CustomerValidateToken>;

pub type CustomerResult<T> = std::result::Result<T, CustomerError>;

pub enum CustomerError {
    InvalidCredentials,
    CustomerNotFound,
    InvalidToken,
    EmailAlreadyInUse(String),
    Internal(String),
}

#[async_trait::async_trait]
pub trait AuthenticateCustomer: Send + Sync {
    async fn execute(&self, email: &String, password: &String) -> CustomerResult<String>;
}

#[async_trait::async_trait]
pub trait CreateCustomer: Send + Sync {
    async fn execute(
        &self,
        name: &String,
        email: &String,
        password: &String,
    ) -> CustomerResult<Customer>;
}

#[async_trait::async_trait]
pub trait CustomerRetrieveProfile: Send + Sync {
    async fn execute(&self, id: &CustomerId) -> CustomerResult<Customer>;
}

#[async_trait::async_trait]
pub trait CustomerUpdateProfile: Send + Sync {
    async fn execute(&self, id: &CustomerId, data: UpdateCustomerInput)
    -> CustomerResult<Customer>;
}

#[async_trait::async_trait]
pub trait CustomerDeleteAccount: Send + Sync {
    async fn execute(&self, id: &CustomerId) -> CustomerResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerRequestPasswordReset: Send + Sync {
    async fn execute(&self, email: &String) -> CustomerResult<()>;
}

#[async_trait::async_trait]
pub trait CustomerUpdatePassword: Send + Sync {
    async fn execute(&self, token: &String, password: &String) -> CustomerResult<Customer>;
}

#[async_trait::async_trait]
pub trait CustomerValidateToken: Send + Sync {
    async fn execute(&self, token: &String) -> CustomerResult<CustomerId>;
}

pub struct UpdateCustomerInput {
    pub name: Option<String>,
    pub email: Option<String>,
}
