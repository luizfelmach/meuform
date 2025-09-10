use crate::{Answer, FlowId, FormId, NodeId, SubmissionId};

use anyhow::{Ok, Result, bail};
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
    pub form_id: FormId,
    pub flow_id: FlowId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SubmissionStatus {
    InProgress(NodeId),
    Completed,
}

impl Submission {
    pub fn new(id: SubmissionId, form_id: FormId, flow_id: FlowId, node: NodeId) -> Self {
        let now = Utc::now();
        Self {
            id,
            form_id,
            flow_id,
            status: SubmissionStatus::InProgress(node),
            history: vec![node],
            answers: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn respond(&mut self, answer: Answer) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => bail!("Cannot add a response to a completed submission"),
            SubmissionStatus::InProgress(node) => {
                self.answers.insert(node, answer);
                self.updated_at = Utc::now();
                Ok(())
            }
        }
    }

    pub fn withdraw(&mut self, node: &NodeId) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => {
                bail!("Cannot remove a response to a completed submission")
            }
            SubmissionStatus::InProgress(_) => {
                self.updated_at = Utc::now();
                self.answers.remove(node);
                Ok(())
            }
        }
    }

    pub fn goto(&mut self, node: NodeId) -> Result<()> {
        match self.status {
            SubmissionStatus::Completed => {
                bail!("Cannot move to another node in a completed submission")
            }
            SubmissionStatus::InProgress(active) if active == node => {
                bail!("Cannot move to active node")
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
            SubmissionStatus::Completed => {
                bail!("Cannot go back in a completed submission")
            }
            SubmissionStatus::InProgress(_) if self.history.len() <= 1 => {
                bail!("No previous node to go back to")
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
            SubmissionStatus::Completed => {
                bail!("Submission is already completed")
            }
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
