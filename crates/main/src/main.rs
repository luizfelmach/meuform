mod adapter;
mod factory;

use adapter::axum::adapt;
use axum::{Router, routing::post};

#[tokio::main]
async fn main() {
    let signin = factory::controller::auth::signin();
    let signup = factory::controller::auth::signup();

    let app = Router::new()
        .route("/signin", post(adapt(signin)))
        .route("/signup", post(adapt(signup)));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
