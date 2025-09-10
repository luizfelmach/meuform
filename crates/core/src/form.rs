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
