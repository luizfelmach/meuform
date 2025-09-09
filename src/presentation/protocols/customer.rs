use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::Customer;

#[derive(Deserialize, Validate)]
pub struct AuthCustomerRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(length(min = 1, max = 50, message = "Senha deve ter entre 1 e 50 caracteres"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthCustomerResponse {
    pub token: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateCustomerRequest {
    #[validate(length(min = 1, max = 80, message = "Nome deve ter entre 1 e 80 caracteres"))]
    pub name: String,

    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(length(min = 1, max = 50, message = "Senha deve ter entre 1 e 50 caracteres"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateCustomerResponse {
    pub customer: Customer,
}

#[derive(Serialize)]
pub struct GetCustomerResponse {
    pub customer: Customer,
}

#[derive(Deserialize, Validate)]
pub struct UpdateCustomerRequest {
    #[validate(length(min = 1, max = 80, message = "Nome deve ter entre 1 e 80 caracteres"))]
    pub name: String,

    #[validate(email(message = "Email inválido"))]
    pub email: String,
}

#[derive(Serialize)]
pub struct UpdateCustomerResponse {
    pub customer: Customer,
}
