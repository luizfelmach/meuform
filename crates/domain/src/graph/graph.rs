use crate::{Condition, GraphError, GraphId, GraphResult, NodeId, Screen};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Graph {
    pub id: GraphId,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub screen: Screen,
    pub edges: HashMap<NodeId, EdgeCondition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EdgeCondition {
    Conditional { to: NodeId, condition: Condition },
    None { to: NodeId },
}

impl EdgeCondition {
    fn to(&self) -> NodeId {
        match self {
            EdgeCondition::Conditional { to, .. } => *to,
            EdgeCondition::None { to } => *to,
        }
    }
}

impl Graph {
    pub fn new(id: GraphId) -> Self {
        Self {
            id,
            nodes: HashMap::new(),
        }
    }

    pub fn clone(&self, id: GraphId) -> Self {
        Self {
            id,
            nodes: self.nodes.clone(),
        }
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

    pub fn upsert_edge(&mut self, from: NodeId, edge: EdgeCondition) -> GraphResult<()> {
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

        if let EdgeCondition::Conditional { condition, .. } = &edge {
            node.screen.accepts(condition).map_err(InvalidCondition)?;
        }

        node.edges.insert(to, edge);

        Ok(())
    }

    pub fn delete_edge(&mut self, from: NodeId, to: NodeId) -> GraphResult<()> {
        use GraphError::*;

        let node = self
            .nodes
            .get_mut(&from)
            .ok_or_else(|| FromNodeNotFound(from))?;

        if !node.detach_edge(to) {
            return Err(EdgeNotFound(from, to));
        }

        Ok(())
    }

    pub fn delete_node(&mut self, node: NodeId) -> GraphResult<()> {
        use GraphError::*;

        self.nodes.remove(&node).ok_or_else(|| NodeNotFound(node))?;
        for n in self.nodes.values_mut() {
            n.detach_edge(node);
        }

        Ok(())
    }
}

impl Node {
    fn new(screen: Screen) -> Self {
        Self {
            screen,
            edges: HashMap::new(),
        }
    }

    fn detach_edge(&mut self, target: NodeId) -> bool {
        self.edges.remove(&target).is_some()
    }
}
