pub mod result;
pub mod status;

use crate::{
    Answer, FlowId, FormId, NodeId, SubmissionId,
    result::{SubmissionError, SubmissionResult},
    status::SubmissionStatus,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Submission {
    pub id: SubmissionId,
    pub form_id: FormId,
    pub flow_id: FlowId,
    pub status: SubmissionStatus,
    pub traverse: Vec<NodeId>,
    pub answers: HashMap<NodeId, Answer>,
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
            traverse: vec![node],
            answers: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Submission {
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn respond(&mut self, answer: Answer) -> SubmissionResult<()> {
        use SubmissionError::*;
        use SubmissionStatus::*;

        match self.status {
            Completed => Err(AlreadyCompleted),
            InProgress(node) => {
                self.answers.insert(node, answer);
                self.touch();
                Ok(())
            }
        }
    }

    pub fn withdraw(&mut self, node: &NodeId) -> SubmissionResult<()> {
        use SubmissionError::*;
        use SubmissionStatus::*;

        match self.status {
            Completed => Err(AlreadyCompleted),
            InProgress(_) => {
                self.answers.remove(node);
                self.touch();
                Ok(())
            }
        }
    }

    pub fn goto(&mut self, node: NodeId) -> SubmissionResult<()> {
        use SubmissionError::*;
        use SubmissionStatus::*;

        match self.status {
            Completed => Err(AlreadyCompleted),
            InProgress(active) if active == node => Err(SameNodeNavigation(active)),
            InProgress(_) => {
                self.traverse.push(node);
                self.status = InProgress(node);
                self.touch();
                Ok(())
            }
        }
    }

    pub fn back(&mut self) -> SubmissionResult<()> {
        use SubmissionError::*;
        use SubmissionStatus::*;

        match self.status {
            Completed => Err(AlreadyCompleted),
            InProgress(_) if self.traverse.len() <= 1 => Err(NoPreviousNode),
            InProgress(_) => {
                self.traverse.pop();
                let previous = *self.traverse.last().unwrap();
                self.status = InProgress(previous);
                self.touch();
                Ok(())
            }
        }
    }

    pub fn complete(&mut self) -> SubmissionResult<()> {
        use SubmissionError::*;
        use SubmissionStatus::*;

        match self.status {
            Completed => Err(AlreadyCompleted),
            InProgress(_) => {
                self.status = Completed;
                self.touch();
                Ok(())
            }
        }
    }

    pub fn response(&self, node: &NodeId) -> Option<Answer> {
        self.answers.get(node).cloned()
    }
}
