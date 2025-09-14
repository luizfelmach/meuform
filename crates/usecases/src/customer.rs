use anyhow::Result;
use domain::{Customer, CustomerId};
use std::sync::Arc;

pub type DynAuthCustomer = Arc<dyn AuthCustomer>;
pub type DynCreateCustomer = Arc<dyn CreateCustomer>;
pub type DynGetCustomer = Arc<dyn GetCustomer>;
pub type DynUpdateCustomer = Arc<dyn UpdateCustomer>;
pub type DynDeleteCustomer = Arc<dyn DeleteCustomer>;
pub type DynForgotPasswordCustomer = Arc<dyn ForgotPasswordCustomer>;
pub type DynUpdatePasswordCustomer = Arc<dyn UpdatePasswordCustomer>;

#[async_trait::async_trait]
pub trait AuthCustomer: Send + Sync {
    async fn execute(&self, email: &String, password: &String) -> Result<String>;
}

#[async_trait::async_trait]
pub trait CreateCustomer: Send + Sync {
    async fn execute(&self, name: &String, email: &String, password: &String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GetCustomer: Send + Sync {
    async fn execute(&self, data: GetCustomerInput) -> Result<GetCustomerOutput>;
}

#[async_trait::async_trait]
pub trait UpdateCustomer: Send + Sync {
    async fn execute(&self, data: UpdateCustomerInput) -> Result<UpdateCustomerOutput>;
}

#[async_trait::async_trait]
pub trait DeleteCustomer: Send + Sync {
    async fn execute(&self, data: DeleteCustomerInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ForgotPasswordCustomer: Send + Sync {
    async fn execute(&self, data: ForgotPasswordCustomerInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UpdatePasswordCustomer: Send + Sync {
    async fn execute(&self, data: UpdatePasswordCustomerInput) -> Result<()>;
}

pub struct GetCustomerInput {
    pub id: CustomerId,
}

pub struct GetCustomerOutput {
    pub customer: Customer,
}

pub struct UpdateCustomerInput {
    pub id: CustomerId,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct UpdateCustomerOutput {
    pub customer: Customer,
}

pub struct DeleteCustomerInput {
    pub id: CustomerId,
}

pub struct ForgotPasswordCustomerInput {
    pub id: CustomerId,
}

pub struct UpdatePasswordCustomerInput {
    pub id: CustomerId,
    pub token: String,
    pub password: String,
}
