use crate::presentation::protocols::customer::{
    AuthCustomerRequest, AuthCustomerResponse, CreateCustomerRequest, CreateCustomerResponse,
    UpdateCustomerRequest, UpdateCustomerResponse,
};
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use chrono::Utc;
use validator::Validate;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_customer))
        .route("/", get(get_customer))
        .route("/", put(update_customer))
        .route("/", delete(delete_customer))
        .route("/auth", post(auth_customer))
}

async fn auth_customer(Json(body): Json<AuthCustomerRequest>) -> impl IntoResponse {
    if let Err(errors) = body.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let response = AuthCustomerResponse {
        token: "123".into(),
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn create_customer(Json(body): Json<CreateCustomerRequest>) -> impl IntoResponse {
    if let Err(errors) = body.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let response = CreateCustomerResponse {
        customer: crate::core::Customer {
            id: "123".into(),
            name: "Luiz".into(),
            email: "luiz.f.machado@edu.ufes.br".into(),
            password: "123".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn get_customer() -> impl IntoResponse {
    StatusCode::OK
}

async fn update_customer(Json(body): Json<UpdateCustomerRequest>) -> impl IntoResponse {
    if let Err(errors) = body.validate() {
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let response = UpdateCustomerResponse {
        customer: crate::core::Customer {
            id: "123".into(),
            name: "Luiz".into(),
            email: "luiz.f.machado@edu.ufes.br".into(),
            password: "123".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn delete_customer() -> impl IntoResponse {
    StatusCode::OK
}
