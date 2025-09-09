pub mod answer;
pub mod customer;
pub mod form;

use axum::Router;

pub fn api() -> Router {
    Router::new().nest("/customer", customer::routes())
}
