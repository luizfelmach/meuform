use chrono::{DateTime, Utc};
use serde::Serialize;

pub type CustomerId = String;

#[derive(Serialize)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
