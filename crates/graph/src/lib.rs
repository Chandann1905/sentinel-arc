pub mod engine;
pub mod projection;

pub use engine::GraphEngine;
pub use projection::GraphProjection;

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_core::domain::node::Node;
    use sentinel_arc_core::types::node_type::NodeType;
    use sentinel_arc_core::types::relationship_type::RelationshipType;
    use std::collections::HashSet;

    #[test]
    fn test_graph_projection_add_nodes_edges() {
        let mut proj = GraphProjection::new();
        let n1 = Node::new(NodeType::Feature, "Feature A");
        let n2 = Node::new(NodeType::Task, "Task B");
        let n3 = Node::new(NodeType::Api, "API C");

        proj.add_node(n1.id.clone());
        proj.add_node(n2.id.clone());
        proj.add_node(n3.id.clone());

        proj.add_edge(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        proj.add_edge(n2.id.clone(), n3.id.clone(), RelationshipType::Uses);

        let deps = proj.find_dependencies(&n1.id, None);
        // n1 -> n2 -> n3
        // Should find n2 and n3.
        assert_eq!(deps.len(), 2);

        // Use HashSet for unordered matching
        let deps_set: HashSet<_> = deps.into_iter().collect();
        assert!(deps_set.contains(&n2.id));
        assert!(deps_set.contains(&n3.id));

        let impact = proj.find_impact(&n3.id, None);
        // n3 is used by n2, n2 is depended on by n1.
        // So n3 impacts n2 and n1.
        assert_eq!(impact.len(), 2);

        let impact_set: HashSet<_> = impact.into_iter().collect();
        assert!(impact_set.contains(&n1.id));
        assert!(impact_set.contains(&n2.id));

        // Shortest path
        let path = proj.find_path(&n1.id, &n3.id).unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], n1.id);
        assert_eq!(path[1], n2.id);
        assert_eq!(path[2], n3.id);
    }
}
