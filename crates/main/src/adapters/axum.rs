use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::future::BoxFuture;
use presentation::{DynController, HttpError, HttpRequest, HttpResponse};
use serde_json::json;
use std::collections::HashMap;

pub fn adapt(
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

            match controller.handle(request).await {
                Ok(resp) => AxumResponse(resp).into_response(),
                Err(err) => AxumError(err).into_response(),
            }
        })
    }
}

pub struct AxumResponse(pub HttpResponse);

pub struct AxumError(pub HttpError);

impl IntoResponse for AxumResponse {
    fn into_response(self) -> Response {
        match self.0 {
            HttpResponse::Ok(bytes) => (StatusCode::OK, bytes).into_response(),
            HttpResponse::Created(bytes) => (StatusCode::CREATED, bytes).into_response(),
            HttpResponse::NoContent => StatusCode::NO_CONTENT.into_response(),
        }
    }
}

impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        let (status, msg) = match self.0 {
            HttpError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            HttpError::AccessDenied => (StatusCode::FORBIDDEN, "Access denied".to_string()),
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = serde_json::to_vec(&json!({ "error": msg }))
            .unwrap_or_else(|_| b"{\"error\":\"internal serialization error\"}".to_vec());

        (status, body).into_response()
    }
}
