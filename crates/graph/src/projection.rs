use petgraph::Direction;
use petgraph::algo::astar;
use petgraph::graph::{DiGraph, NodeIndex};
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_core::types::relationship_type::RelationshipType;
use std::collections::{HashMap, HashSet};

/// In-memory projection of the knowledge graph.
/// Optimized for fast relationship traversal and impact analysis.
pub struct GraphProjection {
    graph: DiGraph<NodeId, RelationshipType>,
    node_map: HashMap<NodeId, NodeIndex>,
}

impl Default for GraphProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphProjection {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Adds a node to the projection. If it already exists, returns the existing index.
    pub fn add_node(&mut self, id: NodeId) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(&id) {
            idx
        } else {
            let idx = self.graph.add_node(id.clone());
            self.node_map.insert(id, idx);
            idx
        }
    }

    /// Adds a directed edge between source and target.
    pub fn add_edge(&mut self, source: NodeId, target: NodeId, rel_type: RelationshipType) {
        let source_idx = self.add_node(source);
        let target_idx = self.add_node(target);
        self.graph.add_edge(source_idx, target_idx, rel_type);
    }

    /// Returns the NodeIndex if the node exists in the projection.
    pub fn get_node_index(&self, id: &NodeId) -> Option<NodeIndex> {
        self.node_map.get(id).copied()
    }

    // ─── Traversals ──────────────────────────────────────────────────────────

    /// Finds all downstream dependencies (things this node relies on).
    /// Follows outgoing edges up to `max_depth`.
    pub fn find_dependencies(&self, root: &NodeId, max_depth: Option<usize>) -> Vec<NodeId> {
        self.traverse(root, Direction::Outgoing, max_depth)
    }

    /// Finds upstream impact (things that rely on this node).
    /// Follows incoming edges up to `max_depth`.
    pub fn find_impact(&self, root: &NodeId, max_depth: Option<usize>) -> Vec<NodeId> {
        self.traverse(root, Direction::Incoming, max_depth)
    }

    /// Finds the shortest path between source and target, ignoring edge weights (weight = 1).
    pub fn find_path(&self, source: &NodeId, target: &NodeId) -> Option<Vec<NodeId>> {
        let start = self.get_node_index(source)?;
        let end = self.get_node_index(target)?;

        let res = astar(&self.graph, start, |finish| finish == end, |_e| 1, |_| 0);

        res.map(|(_cost, path)| {
            path.into_iter()
                .map(|idx| self.graph[idx].clone())
                .collect()
        })
    }

    /// Returns all direct children of a node.
    pub fn get_descendants(&self, root: &NodeId) -> Vec<NodeId> {
        self.traverse(root, Direction::Outgoing, Some(1))
    }

    /// Returns all direct parents of a node.
    pub fn get_ancestors(&self, root: &NodeId) -> Vec<NodeId> {
        self.traverse(root, Direction::Incoming, Some(1))
    }

    // ─── Internal Helpers ────────────────────────────────────────────────────

    /// Generalized BFS traversal.
    fn traverse(&self, root: &NodeId, dir: Direction, max_depth: Option<usize>) -> Vec<NodeId> {
        let start = match self.get_node_index(root) {
            Some(idx) => idx,
            None => return vec![],
        };

        let mut visited = HashSet::new();
        visited.insert(start);

        let mut current_level = vec![start];
        let mut depth = 0;
        let limit = max_depth.unwrap_or(usize::MAX);

        let mut result = Vec::new();

        while !current_level.is_empty() && depth < limit {
            let mut next_level = Vec::new();

            for &node in &current_level {
                let neighbors: Vec<NodeIndex> = match dir {
                    Direction::Outgoing => self
                        .graph
                        .neighbors_directed(node, Direction::Outgoing)
                        .collect(),
                    Direction::Incoming => self
                        .graph
                        .neighbors_directed(node, Direction::Incoming)
                        .collect(),
                };

                for neighbor in neighbors {
                    if visited.insert(neighbor) {
                        next_level.push(neighbor);
                        result.push(self.graph[neighbor].clone());
                    }
                }
            }

            current_level = next_level;
            depth += 1;
        }

        result
    }
}
