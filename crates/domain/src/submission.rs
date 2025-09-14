use crate::{Answer, CustomerId, FlowId, FormId, NodeId, Result, SubmissionError, SubmissionId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Submission {
    #[serde(rename = "_id")]
    pub id: SubmissionId,
    pub status: SubmissionStatus,
    pub history: Vec<NodeId>,
    pub answers: HashMap<NodeId, Answer>,
    pub flow_id: FlowId,
    pub form_id: FormId,
    pub customer_id: CustomerId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SubmissionStatus {
    InProgress(NodeId),
    Completed,
}

impl Submission {
    pub fn new(
        id: SubmissionId,
        form_id: FormId,
        flow_id: FlowId,
        customer_id: CustomerId,
        node: NodeId,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            form_id,
            flow_id,
            status: SubmissionStatus::InProgress(node),
            history: vec![node],
            answers: HashMap::new(),
            customer_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn respond(&mut self, answer: Answer) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => Err(SubmissionError::AlreadyCompleted)?,
            SubmissionStatus::InProgress(node) => {
                self.answers.insert(node, answer);
                self.updated_at = Utc::now();
                Ok(())
            }
        }
    }

    pub fn withdraw(&mut self, node: &NodeId) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => Err(SubmissionError::AlreadyCompleted)?,
            SubmissionStatus::InProgress(_) => {
                self.updated_at = Utc::now();
                self.answers.remove(node);
                Ok(())
            }
        }
    }

    pub fn goto(&mut self, node: NodeId) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => Err(SubmissionError::AlreadyCompleted)?,
            SubmissionStatus::InProgress(active) if active == node => {
                Err(SubmissionError::SameNodeNavigation)?
            }
            SubmissionStatus::InProgress(_) => {
                self.history.push(node);
                self.status = SubmissionStatus::InProgress(node);
                self.updated_at = Utc::now();
                Ok(())
            }
        }
    }

    pub fn back(&mut self) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => Err(SubmissionError::AlreadyCompleted)?,
            SubmissionStatus::InProgress(_) if self.history.len() <= 1 => {
                Err(SubmissionError::NoPreviousNode)?
            }
            SubmissionStatus::InProgress(_) => {
                self.history.pop();
                let previous = *self.history.last().unwrap();
                self.status = SubmissionStatus::InProgress(previous);
                self.updated_at = Utc::now();
                Ok(())
            }
        }
    }

    pub fn complete(&mut self) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => Err(SubmissionError::AlreadyCompleted)?,
            SubmissionStatus::InProgress(_) => {
                self.status = SubmissionStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            }
        }
    }

    pub fn response(&self, node: &NodeId) -> Option<Answer> {
        self.answers.get(node).cloned()
    }
}
