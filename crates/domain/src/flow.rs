use crate::{CustomerId, FlowId, Graph};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Flow {
    #[serde(rename = "_id")]
    pub id: FlowId,
    pub graph: Graph,
    pub customer_id: CustomerId,
    pub created_at: DateTime<Utc>,
}

impl Flow {
    pub fn new(id: FlowId, customer_id: CustomerId, graph: Graph) -> Self {
        let now = Utc::now();
        Self {
            id,
            graph,
            customer_id,
            created_at: now,
        }
    }
}
