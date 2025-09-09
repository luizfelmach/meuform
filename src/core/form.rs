use crate::core::{customer::CustomerId, graph::GraphId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type FormId = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    #[serde(rename = "_id")]
    pub id: FormId,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub customer_id: CustomerId,
    pub graph_id: GraphId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
