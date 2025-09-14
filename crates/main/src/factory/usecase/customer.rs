use data::{AuthCustomerImpl, CreateCustomerImpl};
use std::sync::Arc;
use usecases::{DynAuthCustomer, DynCreateCustomer};

use crate::factory;

pub fn auth() -> DynAuthCustomer {
    let usecase = AuthCustomerImpl {
        customer: factory::infra::repository::customer(),
        tokenizer: factory::infra::crypto::tokenizer(),
    };
    return Arc::new(usecase);
}

pub fn create() -> DynCreateCustomer {
    let usecase = CreateCustomerImpl;
    return Arc::new(usecase);
}
