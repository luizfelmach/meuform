mod core;
mod data;
mod presentation;
mod protocols;
mod usecase;

#[tokio::main]
async fn main() {
    let app = presentation::routes::api();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4910").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
