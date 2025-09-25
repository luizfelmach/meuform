use crate::{CustomerId, FlowId, FormId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub flow_id: Option<FlowId>,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Form {
    pub fn new(id: String, customer_id: CustomerId, slug: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            customer_id,
            flow_id: None,
            slug,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.touch();
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = slug;
        self.touch();
    }

    pub fn set_flow_id(&mut self, flow_id: FlowId) {
        self.flow_id = Some(flow_id);
        self.touch();
    }
}
