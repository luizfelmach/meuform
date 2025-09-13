// mod customer;

use async_trait::async_trait;
use axum::{
    Router,
    body::Bytes,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

#[async_trait]
pub trait Controller: Send + Sync {
    async fn handle(&self, request: HttpRequest) -> HttpResponse;
}

pub type DynController = Arc<dyn Controller>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpRequest {
    pub body: Vec<u8>,
    pub params: HashMap<String, String>,
    pub queries: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpError {
    InvalidInput(String),
    Unauthorized,
    AccessDenied,
    Internal(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpResponse {
    Ok(Vec<u8>),
    Created(Vec<u8>),
    NoContent,
    Error(HttpError),
}

impl HttpResponse {
    pub fn ok<T: serde::Serialize>(data: &T) -> Self {
        let body = serde_json::to_vec(data).unwrap_or_default();
        HttpResponse::Ok(body)
    }

    pub fn created<T: serde::Serialize>(data: &T) -> Self {
        let body = serde_json::to_vec(data).unwrap_or_default();
        HttpResponse::Created(body)
    }

    pub fn error(e: HttpError) -> Self {
        HttpResponse::Error(e)
    }
}

impl HttpRequest {
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, HttpError> {
        serde_json::from_slice(&self.body).map_err(|e| HttpError::InvalidInput(e.to_string()))
    }

    pub fn param(&self, key: &str) -> Result<String, HttpError> {
        self.params
            .get(key)
            .cloned()
            .ok_or(HttpError::InvalidInput(format!("Missing param: {}", key)))
    }

    pub fn query(&self, key: &str) -> Result<String, HttpError> {
        self.queries
            .get(key)
            .cloned()
            .ok_or(HttpError::InvalidInput(format!("Missing query: {}", key)))
    }

    pub fn header(&self, key: &str) -> Result<String, HttpError> {
        self.headers
            .get(key)
            .cloned()
            .ok_or(HttpError::InvalidInput(format!("Missing header: {}", key)))
    }
}

fn axum_controller_adapt(
    controller: DynController,
) -> impl Fn(
    Path<HashMap<String, String>>,
    Query<HashMap<String, String>>,
    HeaderMap,
    Bytes,
) -> BoxFuture<'static, Response>
+ Clone
+ Send
+ Sync
+ 'static {
    move |Path(params): Path<HashMap<String, String>>,
          Query(queries): Query<HashMap<String, String>>,
          headers: HeaderMap,
          body: Bytes| {
        let controller = controller.clone();

        Box::pin(async move {
            let request = HttpRequest {
                body: body.into(),
                params: params.clone(),
                queries: queries.clone(),
                headers: headers
                    .iter()
                    .map(|(name, value)| {
                        (name.to_string(), value.to_str().unwrap_or("").to_string())
                    })
                    .collect(),
            };

            controller.handle(request).await.into_response()
        })
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> Response {
        use HttpError::*;
        use HttpResponse::*;

        match self {
            Ok(bytes) => (StatusCode::OK, bytes).into_response(),
            Created(bytes) => (StatusCode::CREATED, bytes).into_response(),
            NoContent => StatusCode::NO_CONTENT.into_response(),
            Error(err) => {
                let (status, msg) = match err {
                    InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
                    Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
                    AccessDenied => (StatusCode::FORBIDDEN, "Access denied".to_string()),
                    Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                };
                let body = serde_json::to_vec(&json!({ "error": msg })).unwrap_or_default();
                (status, body).into_response()
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct Teste {
    pub name: String,
}

struct HelloController;

#[async_trait]
impl Controller for HelloController {
    async fn handle(&self, request: HttpRequest) -> HttpResponse {
        let body: Result<Teste, HttpError> = request.json();

        match body {
            Ok(teste) => {
                println!("{:?}", teste);
                HttpResponse::NoContent
            }
            Err(err) => HttpResponse::Error(err),
        }
    }
}

#[tokio::main]
async fn main() {
    let hello_controller = Arc::new(HelloController);

    let app = Router::new().route(
        "/hello/{nameId}/{teste}",
        post(axum_controller_adapt(hello_controller)),
    );

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
