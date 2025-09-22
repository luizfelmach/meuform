pub mod condition;
pub mod graph;
pub mod screen;

use crate::{CustomerId, FlowId, graph::Graph};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Flow {
    pub id: FlowId,
    pub customer_id: CustomerId,
    pub graph: Graph,
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
