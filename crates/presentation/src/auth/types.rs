use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SignInBody {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 1, max = 50, message = "Senha deve ter entre 1 e 50 caracteres"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Deserialize, Validate)]
pub struct SignUpBody {
    #[validate(length(min = 1, max = 80, message = "Nome deve ter entre 1 e 80 caracteres"))]
    pub name: String,
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 1, max = 50, message = "Senha deve ter entre 1 e 50 caracteres"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub token: String,
}
