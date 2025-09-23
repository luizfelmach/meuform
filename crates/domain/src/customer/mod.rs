use crate::CustomerId;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Customer {
    pub fn create(id: CustomerId, name: String, email: String, password: String) -> Self {
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

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.touch();
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
        self.touch();
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
        self.touch();
    }
}
