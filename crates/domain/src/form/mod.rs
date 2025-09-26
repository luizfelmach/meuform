use crate::{CustomerId, FormId, GraphId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: FormId,
    pub customer_id: CustomerId,
    pub graph_id: GraphId,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Form {
    pub fn new(
        id: String,
        customer_id: CustomerId,
        graph_id: GraphId,
        slug: String,
        name: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            customer_id,
            graph_id,
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

    pub fn set_graph_id(&mut self, graph_id: GraphId) {
        self.graph_id = graph_id;
        self.touch();
    }
}
