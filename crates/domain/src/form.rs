use crate::{CustomerId, FlowId, FormId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    #[serde(rename = "_id")]
    pub id: FormId,
    pub slug: String,
    pub name: String,
    pub customer_id: CustomerId,
    pub flow_id: FlowId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Form {
    pub fn new(
        id: String,
        slug: String,
        name: String,
        customer_id: CustomerId,
        flow_id: FlowId,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            slug,
            name,
            customer_id,
            flow_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now()
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = slug;
        self.updated_at = Utc::now()
    }
}
