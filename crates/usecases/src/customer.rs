use anyhow::Result;
use domain::{Customer, CustomerId};

#[async_trait::async_trait]
pub trait AuthCustomer {
    async fn execute(&self, data: AuthCustomerInput) -> Result<AuthCustomerOutput>;
}

#[async_trait::async_trait]
pub trait CreateCustomer {
    async fn execute(&self, data: CreateCustomerInput) -> Result<CreateCustomerOutput>;
}

#[async_trait::async_trait]
pub trait GetCustomer {
    async fn execute(&self, data: GetCustomerInput) -> Result<GetCustomerOutput>;
}

#[async_trait::async_trait]
pub trait UpdateCustomer {
    async fn execute(&self, data: UpdateCustomerInput) -> Result<UpdateCustomerOutput>;
}

#[async_trait::async_trait]
pub trait DeleteCustomer {
    async fn execute(&self, data: DeleteCustomerInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ForgotPasswordCustomer {
    async fn execute(&self, data: ForgotPasswordCustomerInput) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UpdatePasswordCustomer {
    async fn execute(&self, data: UpdatePasswordCustomerInput) -> Result<()>;
}

pub struct AuthCustomerInput {
    pub email: String,
    pub password: String,
}

pub struct AuthCustomerOutput {
    pub token: String,
}

pub struct CreateCustomerInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct CreateCustomerOutput {
    pub customer: Customer,
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
