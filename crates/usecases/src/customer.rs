use anyhow::Result;
use domain::{Customer, CustomerId};

#[async_trait::async_trait]
pub trait CustomerUseCase {
    async fn auth(&self, data: AuthCustomerInput) -> Result<AuthCustomerOutput>;
    async fn create(&self, data: CreateCustomerInput) -> Result<CreateCustomerOutput>;
    async fn get(&self, data: GetCustomerInput) -> Result<GetCustomerOutput>;
    async fn update(&self, data: UpdateCustomerInput) -> Result<UpdateCustomerOutput>;
    async fn delete(&self, data: DeleteCustomerInput) -> Result<()>;
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
    pub password: Option<String>,
}

pub struct UpdateCustomerOutput {
    pub customer: Customer,
}

pub struct DeleteCustomerInput {
    pub id: CustomerId,
}
