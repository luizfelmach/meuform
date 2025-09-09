use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type CustomerId = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    #[serde(rename = "_id")]
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
