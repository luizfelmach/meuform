use anyhow::Result;

pub struct AuthCustomerInput {
    pub email: String,
    pub password: String,
}

pub struct AuthCustomerOutput {
    pub token: String,
}

pub trait AuthCustomer {
    async fn auth(&self, data: AuthCustomerInput) -> Result<AuthCustomerOutput>;
}
