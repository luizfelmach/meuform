use crate::CustomerId;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomerWithoutPassword {
    #[serde(rename = "_id")]
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Customer {
    pub fn new(id: CustomerId, name: String, email: String, password: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            email,
            password,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<Customer> for CustomerWithoutPassword {
    fn from(customer: Customer) -> Self {
        CustomerWithoutPassword {
            id: customer.id,
            name: customer.name,
            email: customer.email,
            created_at: customer.created_at,
            updated_at: customer.updated_at,
        }
    }
}
