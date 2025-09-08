use crate::core::{customer::CustomerId, graph::GraphId};
use chrono::{DateTime, Utc};

pub type FormId = String;

pub struct Form {
    pub id: FormId,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub owner_id: CustomerId,
    pub graph_id: GraphId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
