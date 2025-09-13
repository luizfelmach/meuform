mod adapters;

use adapters::axum::adapt;
use axum::{Router, routing::post};
use presentation::{SignInController, SignUpController, controller};

#[tokio::main]
async fn main() {
    let signin_controller = controller!(SignInController);
    let signup_controller = controller!(SignUpController);

    let app = Router::new()
        .route("/signin", post(adapt(signin_controller)))
        .route("/signup", post(adapt(signup_controller)));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
