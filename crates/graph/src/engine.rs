use crate::projection::GraphProjection;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;

/// Engine responsible for projecting relational knowledge into an optimized graph structure.
pub struct GraphEngine;

impl GraphEngine {
    /// Builds a full in-memory graph projection from the current state of the Knowledge Engine.
    pub async fn build_projection(knowledge: &KnowledgeEngine) -> BrainResult<GraphProjection> {
        let mut projection = GraphProjection::new();

        // 1. Load all nodes
        let nodes = knowledge.list_nodes().await?;
        for node in nodes {
            projection.add_node(node.id);
        }

        // 2. Load all relationships
        let relationships = knowledge.list_all_relationships().await?;
        for rel in relationships {
            projection.add_edge(rel.source_node, rel.target_node, rel.relationship_type);
        }

        Ok(projection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_core::domain::node::Node;
    use sentinel_arc_core::domain::relationship::Relationship;
    use sentinel_arc_core::types::node_type::NodeType;
    use sentinel_arc_core::types::relationship_type::RelationshipType;
    use sentinel_arc_knowledge::database::Database;
    use tempfile::tempdir;

    async fn setup_engine() -> KnowledgeEngine {
        let dir = tempdir().unwrap();
        let db = Database::init(dir.path()).await.unwrap();
        KnowledgeEngine::new(&db)
    }

    #[tokio::test]
    async fn test_build_projection() {
        let engine = setup_engine().await;

        let n1 = Node::new(NodeType::Feature, "F1");
        let n2 = Node::new(NodeType::Task, "T1");

        let (node1, _) = engine.create_node(n1.clone()).await.unwrap();
        let (node2, _) = engine.create_node(n2.clone()).await.unwrap();

        let rel = Relationship::new(
            node1.id.clone(),
            node2.id.clone(),
            RelationshipType::DependsOn,
        );
        engine.create_relationship(rel).await.unwrap();

        let proj = GraphEngine::build_projection(&engine).await.unwrap();

        let deps = proj.find_dependencies(&node1.id, None);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], node2.id);

        let impact = proj.find_impact(&node2.id, None);
        assert_eq!(impact.len(), 1);
        assert_eq!(impact[0], node1.id);
    }
}
