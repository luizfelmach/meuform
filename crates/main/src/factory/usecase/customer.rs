use data::{AuthCustomerImpl, CreateCustomerImpl};
use std::sync::Arc;
use usecases::{DynAuthCustomer, DynCreateCustomer};

pub fn auth() -> DynAuthCustomer {
    let usecase = AuthCustomerImpl;
    return Arc::new(usecase);
}

pub fn create() -> DynCreateCustomer {
    let usecase = CreateCustomerImpl;
    return Arc::new(usecase);
}
