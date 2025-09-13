use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateCustomerResponse {
    pub token: String,
}
