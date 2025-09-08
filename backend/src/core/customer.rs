use chrono::{DateTime, Utc};

pub type CustomerId = String;

pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
