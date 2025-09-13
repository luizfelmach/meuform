use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct SignUpBody {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub token: String,
}
