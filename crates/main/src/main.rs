fn main() {
    println!("Hello, world!");
}

// fn axum_controller_adapt(
//     controller: DynController,
// ) -> impl Fn(
//     Path<HashMap<String, String>>,
//     Query<HashMap<String, String>>,
//     HeaderMap,
//     Bytes,
// ) -> BoxFuture<'static, Response>
// + Clone
// + Send
// + Sync
// + 'static {
//     move |Path(params): Path<HashMap<String, String>>,
//           Query(queries): Query<HashMap<String, String>>,
//           headers: HeaderMap,
//           body: Bytes| {
//         let controller = controller.clone();

//         Box::pin(async move {
//             let request = HttpRequest {
//                 body: body.into(),
//                 params: params.clone(),
//                 queries: queries.clone(),
//                 headers: headers
//                     .iter()
//                     .map(|(name, value)| {
//                         (name.to_string(), value.to_str().unwrap_or("").to_string())
//                     })
//                     .collect(),
//             };

//             controller.handle(request).await.into_response()
//         })
//     }
// }

// impl IntoResponse for HttpResponse {
//     fn into_response(self) -> Response {
//         use HttpError::*;
//         use HttpResponse::*;

//         match self {
//             Ok(bytes) => (StatusCode::OK, bytes).into_response(),
//             Created(bytes) => (StatusCode::CREATED, bytes).into_response(),
//             NoContent => StatusCode::NO_CONTENT.into_response(),
//             Error(err) => {
//                 let (status, msg) = match err {
//                     InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
//                     Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
//                     AccessDenied => (StatusCode::FORBIDDEN, "Access denied".to_string()),
//                     Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
//                 };
//                 let body = serde_json::to_vec(&json!({ "error": msg })).unwrap_or_default();
//                 (status, body).into_response()
//             }
//         }
//     }
// }
