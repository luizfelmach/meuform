use anyhow::Result;

use crate::core::{
    customer::CustomerId,
    form::{Form, FormId},
};

pub struct Input {
    pub customer_id: CustomerId,
    pub form_id: FormId,
}

pub struct Output {
    pub form: Form,
}

pub trait GetForm {
    async fn get(&self, data: Input) -> Result<Output>;
}
