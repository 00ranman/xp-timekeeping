use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{UtError, UtResult};
use crate::quant::Quant;

/// A node in the temporal DAG (Directed Acyclic Graph).
///
/// Each node carries a quant timestamp. Minting a Temporal Token IS a tick
/// of the system clock. The DAG provides topological sorting for temporal
/// ordering without a central authority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalNode {
    /// Unique node identifier.
    pub id: String,
    /// Quant timestamp of this node.
    pub timestamp: Quant,
    /// IDs of parent nodes (causal ancestors).
    pub parents: Vec<String>,
    /// Whether this is a checkpoint node (never pruned).
    pub is_checkpoint: bool,
    /// Whether this node has been pruned (archived but queryable).
    pub is_pruned: bool,
    /// Optional payload data.
    pub payload: Option<String>,
}

impl TemporalNode {
    /// Create a new temporal node.
    pub fn new(id: String, timestamp: Quant, parents: Vec<String>) -> Self {
        Self {
            id,
            timestamp,
            parents,
            is_checkpoint: false,
            is_pruned: false,
            payload: None,
        }
    }

    /// Create a genesis (root) node.
    pub fn genesis(id: String) -> Self {
        Self {
            id,
            timestamp: Quant::zero(),
            parents: Vec::new(),
            is_checkpoint: true,
            is_pruned: false,
            payload: None,
        }
    }

    /// Create a checkpoint node.
    pub fn checkpoint(id: String, timestamp: Quant, parents: Vec<String>) -> Self {
        Self {
            id,
            timestamp,
            parents,
            is_checkpoint: true,
            is_pruned: false,
            payload: None,
        }
    }

    /// Check if this is a merge node (has multiple parents).
    pub fn is_merge(&self) -> bool {
        self.parents.len() > 1
    }

    /// Check if this is a root node (no parents).
    pub fn is_root(&self) -> bool {
        self.parents.is_empty()
    }
}

/// An edge in the temporal DAG, representing a causal relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEdge {
    /// Source (ancestor) node ID.
    pub from: String,
    /// Target (descendant) node ID.
    pub to: String,
    /// Quant delta between the two nodes.
    pub quant_delta: Quant,
}

/// The temporal DAG substrate.
///
/// Enforces causal consistency rules:
/// - If A is ancestor of B, A's timestamp < B's timestamp
/// - Concurrent nodes may share the same quant range
/// - Merge nodes must have timestamp > all parent timestamps
/// - Checkpoint nodes at regular quant intervals are never pruned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDag {
    nodes: HashMap<String, TemporalNode>,
}

impl TemporalDag {
    /// Create an empty DAG.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Add a node to the DAG, validating causal consistency.
    pub fn add_node(&mut self, node: TemporalNode) -> UtResult<()> {
        // Validate: all parents must exist and have earlier timestamps
        for parent_id in &node.parents {
            let parent = self.nodes.get(parent_id).ok_or_else(|| {
                UtError::DagError(format!("parent node '{}' not found", parent_id))
            })?;
            if parent.timestamp >= node.timestamp {
                return Err(UtError::DagError(format!(
                    "causal violation: parent '{}' timestamp {} >= node '{}' timestamp {}",
                    parent_id, parent.timestamp, node.id, node.timestamp
                )));
            }
        }

        // For merge nodes: timestamp must be > all parent timestamps (already checked above)
        if self.nodes.contains_key(&node.id) {
            return Err(UtError::DagError(format!(
                "node '{}' already exists",
                node.id
            )));
        }

        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: &str) -> Option<&TemporalNode> {
        self.nodes.get(id)
    }

    /// Get all node IDs.
    pub fn node_ids(&self) -> Vec<&String> {
        self.nodes.keys().collect()
    }

    /// Get the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get all checkpoint nodes.
    pub fn checkpoints(&self) -> Vec<&TemporalNode> {
        self.nodes.values().filter(|n| n.is_checkpoint).collect()
    }

    /// Get edges implied by parent relationships.
    pub fn edges(&self) -> Vec<TemporalEdge> {
        let mut edges = Vec::new();
        for node in self.nodes.values() {
            for parent_id in &node.parents {
                if let Some(parent) = self.nodes.get(parent_id) {
                    edges.push(TemporalEdge {
                        from: parent_id.clone(),
                        to: node.id.clone(),
                        quant_delta: node.timestamp.abs_diff(parent.timestamp),
                    });
                }
            }
        }
        edges
    }

    /// Topological sort of nodes (ancestors before descendants).
    pub fn topological_sort(&self) -> UtResult<Vec<&TemporalNode>> {
        let mut sorted: Vec<&TemporalNode> = self.nodes.values().collect();
        sorted.sort_by_key(|n| n.timestamp);
        Ok(sorted)
    }

    /// Prune a non-checkpoint node (mark as pruned).
    pub fn prune_node(&mut self, id: &str) -> UtResult<()> {
        let node = self.nodes.get_mut(id).ok_or_else(|| {
            UtError::DagError(format!("node '{}' not found", id))
        })?;
        if node.is_checkpoint {
            return Err(UtError::DagError(format!(
                "cannot prune checkpoint node '{}'",
                id
            )));
        }
        node.is_pruned = true;
        Ok(())
    }
}

impl Default for TemporalDag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_node() {
        let node = TemporalNode::genesis("root".into());
        assert!(node.is_root());
        assert!(node.is_checkpoint);
        assert_eq!(node.timestamp, Quant::zero());
    }

    #[test]
    fn test_dag_add_nodes() {
        let mut dag = TemporalDag::new();
        let root = TemporalNode::genesis("root".into());
        dag.add_node(root).unwrap();

        let n1 = TemporalNode::new("n1".into(), Quant::new(100), vec!["root".into()]);
        dag.add_node(n1).unwrap();

        assert_eq!(dag.node_count(), 2);
    }

    #[test]
    fn test_dag_causal_violation() {
        let mut dag = TemporalDag::new();
        let root = TemporalNode::genesis("root".into());
        dag.add_node(root).unwrap();

        // Trying to add a node with timestamp <= parent should fail
        let bad = TemporalNode::new("bad".into(), Quant::new(0), vec!["root".into()]);
        assert!(dag.add_node(bad).is_err());
    }

    #[test]
    fn test_dag_missing_parent() {
        let mut dag = TemporalDag::new();
        let node = TemporalNode::new("n1".into(), Quant::new(100), vec!["nonexistent".into()]);
        assert!(dag.add_node(node).is_err());
    }

    #[test]
    fn test_dag_duplicate_node() {
        let mut dag = TemporalDag::new();
        let root = TemporalNode::genesis("root".into());
        dag.add_node(root.clone()).unwrap();
        assert!(dag.add_node(root).is_err());
    }

    #[test]
    fn test_merge_node() {
        let mut dag = TemporalDag::new();
        dag.add_node(TemporalNode::genesis("root".into())).unwrap();
        dag.add_node(TemporalNode::new("a".into(), Quant::new(100), vec!["root".into()])).unwrap();
        dag.add_node(TemporalNode::new("b".into(), Quant::new(200), vec!["root".into()])).unwrap();

        let merge = TemporalNode::new("merge".into(), Quant::new(300), vec!["a".into(), "b".into()]);
        assert!(merge.is_merge());
        dag.add_node(merge).unwrap();
        assert_eq!(dag.node_count(), 4);
    }

    #[test]
    fn test_topological_sort() {
        let mut dag = TemporalDag::new();
        dag.add_node(TemporalNode::genesis("root".into())).unwrap();
        dag.add_node(TemporalNode::new("a".into(), Quant::new(100), vec!["root".into()])).unwrap();
        dag.add_node(TemporalNode::new("b".into(), Quant::new(200), vec!["a".into()])).unwrap();

        let sorted = dag.topological_sort().unwrap();
        assert_eq!(sorted[0].id, "root");
        assert_eq!(sorted[1].id, "a");
        assert_eq!(sorted[2].id, "b");
    }

    #[test]
    fn test_checkpoint_cannot_be_pruned() {
        let mut dag = TemporalDag::new();
        dag.add_node(TemporalNode::genesis("root".into())).unwrap();
        assert!(dag.prune_node("root").is_err());
    }

    #[test]
    fn test_non_checkpoint_can_be_pruned() {
        let mut dag = TemporalDag::new();
        dag.add_node(TemporalNode::genesis("root".into())).unwrap();
        dag.add_node(TemporalNode::new("a".into(), Quant::new(100), vec!["root".into()])).unwrap();
        dag.prune_node("a").unwrap();
        assert!(dag.get_node("a").unwrap().is_pruned);
    }

    #[test]
    fn test_edges() {
        let mut dag = TemporalDag::new();
        dag.add_node(TemporalNode::genesis("root".into())).unwrap();
        dag.add_node(TemporalNode::new("a".into(), Quant::new(100), vec!["root".into()])).unwrap();
        let edges = dag.edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].from, "root");
        assert_eq!(edges[0].to, "a");
    }
}
