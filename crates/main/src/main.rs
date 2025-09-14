mod adapter;
mod factory;

use adapter::axum::adapt;
use axum::{Router, routing::post};
use data::{AuthCustomerImpl, CreateCustomerImpl};
use presentation::{SignInController, SignUpController, controller};
use usecases::usecase;

#[tokio::main]
async fn main() {
    let create_customer = usecase!(CreateCustomerImpl);
    let auth_customer = usecase!(AuthCustomerImpl);

    let signin_controller = controller!(SignInController {
        auth: auth_customer.clone()
    });
    let signup_controller = controller!(SignUpController {
        auth: auth_customer.clone(),
        create: create_customer
    });

    let app = Router::new()
        .route("/signin", post(adapt(signin_controller)))
        .route("/signup", post(adapt(signup_controller)));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
