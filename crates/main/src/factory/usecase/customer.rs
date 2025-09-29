use data::{AuthCustomerImpl, CreateCustomerImpl};
use std::sync::Arc;
use usecases::{DynAuthenticateCustomer, DynCustomerRetrieveProfile};

use crate::factory;

pub fn auth() -> DynAuthenticateCustomer {
    let usecase = AuthCustomerImpl {
        customer: factory::infra::repository::customer(),
        tokenizer: factory::infra::crypto::tokenizer(),
    };
    return Arc::new(usecase);
}

pub fn create() -> DynCustomerRetrieveProfile {
    let usecase = CreateCustomerImpl;
    return Arc::new(usecase);
}
