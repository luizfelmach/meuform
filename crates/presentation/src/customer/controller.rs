use axum::{Json, extract::State};
use std::sync::Arc;
use usecases::{AuthCustomer, CreateCustomer};

use crate::{CreateCustomerRequest, CreateCustomerResponse};

pub async fn creater_customer(
    State(service): State<Services>,
    Json(data): Json<CreateCustomerRequest>,
) -> Json<CreateCustomerResponse> {
    let customer = service.customer.create.execute(data).await?;
}

pub struct Services {
    customer: CustomerServices,
    form: FormServices,
}

pub struct CustomerServices {
    auth: Arc<dyn AuthCustomer>,
    create: Arc<dyn CreateCustomer>,
}

pub struct FormServices {}
