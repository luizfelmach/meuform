use crate::{FlowId, Graph};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Flow {
    pub id: FlowId,
    pub graph: Graph,
    pub created_at: DateTime<Utc>,
}

impl Flow {
    pub fn new(id: FlowId, graph: Graph) -> Self {
        let now = Utc::now();
        Self {
            id,
            graph,
            created_at: now,
        }
    }
}
