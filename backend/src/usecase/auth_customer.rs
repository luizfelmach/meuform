use anyhow::Result;

pub struct Input {
    pub email: String,
    pub password: String,
}

pub struct Output {
    pub token: String,
}

pub trait AuthCustomer {
    async fn auth(&self, data: Input) -> Result<Output>;
}
