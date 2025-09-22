use crate::NodeId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum SubmissionStatus {
    InProgress(NodeId),
    Completed,
}
