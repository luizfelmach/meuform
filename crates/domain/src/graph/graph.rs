use crate::{Condition, GraphError, GraphId, GraphResult, NodeId, Screen, Screenable};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
    pub answered: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub screen: Screen,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Edge {
    Unconditional {
        to: NodeId,
    },
    Conditional {
        to: NodeId,
        r#where: NodeId,
        condition: Condition,
    },
}

impl Graph {
    pub fn new(id: GraphId) -> Self {
        Self {
            id,
            nodes: HashMap::new(),
            answered: false,
        }
    }

    pub fn clone(&self, id: GraphId) -> Self {
        Self {
            id,
            nodes: self.nodes.clone(),
            answered: false,
        }
    }

    pub fn set_answered(&mut self) {
        self.answered = true
    }

    pub fn has_answered(&self) -> bool {
        return self.answered;
    }
}

impl Graph {
    pub fn upsert_node(&mut self, id: NodeId, screen: Screen) -> GraphResult<()> {
        self.nodes
            .entry(id)
            .and_modify(|n| n.screen = screen.clone())
            .or_insert_with(|| Node::new(screen));
        Ok(())
    }

    pub fn upsert_edge(&mut self, from: NodeId, edge: Edge) -> GraphResult<()> {
        use GraphError::*;

        let to = edge.to();

        if from == to {
            return Err(SelfLoop(from));
        }

        if !self.nodes.contains_key(&from) {
            return Err(FromNodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(ToNodeNotFound(to));
        }

        let node = self.nodes.get_mut(&from).unwrap();

        if let Edge::Conditional { condition, .. } = &edge {
            node.screen.accepts(condition).map_err(InvalidCondition)?;
        }

        if let Some(existing) = node.edges.iter_mut().find(|e| e.to() == to) {
            *existing = edge;
        } else {
            node.edges.push(edge);
        }

        Ok(())
    }

    pub fn delete_edge(&mut self, from: NodeId, to: NodeId) -> GraphResult<()> {
        use GraphError::*;

        let node = self.nodes.get_mut(&from).ok_or(FromNodeNotFound(from))?;
        let removed = node.remove_edge_to(to);

        if !removed {
            return Err(EdgeNotFound(from, to));
        }

        Ok(())
    }

    pub fn delete_node(&mut self, id: NodeId) -> GraphResult<()> {
        use GraphError::*;

        if self.nodes.remove(&id).is_none() {
            return Err(NodeNotFound(id));
        }

        for node in self.nodes.values_mut() {
            node.remove_edge_to(id);
        }

        Ok(())
    }
}

impl Edge {
    fn to(&self) -> NodeId {
        match self {
            Edge::Unconditional { to } => *to,
            Edge::Conditional { to, .. } => *to,
        }
    }
}

impl Node {
    fn new(screen: Screen) -> Self {
        Self {
            screen,
            edges: Vec::new(),
        }
    }

    fn remove_edge_to(&mut self, target: NodeId) -> bool {
        let before = self.edges.len();
        self.edges.retain(|e| e.to() != target);
        self.edges.len() != before
    }
}
