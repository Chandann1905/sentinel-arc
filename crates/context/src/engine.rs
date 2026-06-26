use crate::discovery::discover_roots;
use crate::enrichment::fetch_enrichment;
use crate::expansion::expand_topology;
use crate::impact::calculate_impact;
use crate::types::{ContextPackage, ContextRequest, ImpactReport};
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_graph::projection::GraphProjection;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;

pub struct ContextEngine;

impl ContextEngine {
    pub fn new() -> Self {
        Self
    }

    /// Generates a highly compressed AI context package based on the given intent.
    pub async fn generate_context(
        &self,
        knowledge: &KnowledgeEngine,
        projection: &GraphProjection,
        request: ContextRequest,
    ) -> BrainResult<ContextPackage> {
        // 1. Discovery
        let root_nodes = discover_roots(knowledge, &request.intent, request.max_nodes).await?;
        if root_nodes.is_empty() {
            return Ok(ContextPackage {
                intent: request.intent,
                ..Default::default()
            });
        }

        // 2. Expansion
        let root_ids: Vec<NodeId> = root_nodes.iter().map(|n| n.id.clone()).collect();
        let expanded_ids = expand_topology(projection, &root_ids, request.max_depth);

        // 3. Enrichment
        let enrichment = fetch_enrichment(knowledge, &expanded_ids).await?;

        // 4. Impact
        let mut overall_impact = ImpactReport::default();
        for root_id in &root_ids {
            let impact = calculate_impact(knowledge, projection, root_id).await?;
            overall_impact
                .affected_features
                .extend(impact.affected_features);
            overall_impact.affected_files.extend(impact.affected_files);
            overall_impact.risk_score = overall_impact.risk_score.max(impact.risk_score);
            overall_impact.complexity_score =
                overall_impact.complexity_score.max(impact.complexity_score);
        }

        // Remove duplicates from aggregated impact
        overall_impact
            .affected_features
            .sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        overall_impact
            .affected_features
            .dedup_by(|a, b| a.id == b.id);

        overall_impact
            .affected_files
            .sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        overall_impact.affected_files.dedup_by(|a, b| a.id == b.id);

        Ok(ContextPackage {
            intent: request.intent,
            root_nodes,
            related_architecture: enrichment.related_architecture,
            relevant_rules: enrichment.relevant_rules,
            known_decisions: enrichment.known_decisions,
            impact_report: overall_impact,
        })
    }

    /// Analyzes the impact of modifying a specific node.
    pub async fn analyze_impact(
        &self,
        knowledge: &KnowledgeEngine,
        projection: &GraphProjection,
        node_id: &NodeId,
    ) -> BrainResult<ImpactReport> {
        calculate_impact(knowledge, projection, node_id).await
    }
}

impl Default for ContextEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_core::domain::node::Node;
    use sentinel_arc_core::domain::relationship::Relationship;
    use sentinel_arc_core::types::node_type::NodeType;
    use sentinel_arc_core::types::relationship_type::RelationshipType;
    use sentinel_arc_graph::engine::GraphEngine;
    use sentinel_arc_knowledge::database::Database;
    use tempfile::tempdir;

    async fn setup_env() -> (KnowledgeEngine, GraphProjection, Node) {
        let dir = tempdir().unwrap();
        let db = Database::init(dir.path()).await.unwrap();
        let knowledge = KnowledgeEngine::new(&db);

        let feature = Node::new(NodeType::Feature, "Authentication");
        let (feature_node, _) = knowledge.create_node(feature).await.unwrap();

        let file = Node::new(NodeType::File, "auth.rs");
        let (file_node, _) = knowledge.create_node(file).await.unwrap();

        let rel = Relationship::new(
            file_node.id.clone(),
            feature_node.id.clone(),
            RelationshipType::Implements,
        );
        knowledge.create_relationship(rel).await.unwrap();

        // Let the search index commit
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let proj = GraphEngine::build_projection(&knowledge).await.unwrap();

        (knowledge, proj, feature_node)
    }

    #[tokio::test]
    async fn test_analyze_impact() {
        let (knowledge, proj, feature_node) = setup_env().await;
        let engine = ContextEngine::new();

        let impact = engine
            .analyze_impact(&knowledge, &proj, &feature_node.id)
            .await
            .unwrap();

        // The file implements the feature, so modifying the feature impacts the file.
        assert_eq!(impact.affected_files.len(), 1);
        assert_eq!(impact.affected_files[0].title, "auth.rs");
    }

    #[tokio::test]
    async fn test_generate_context() {
        let (knowledge, proj, _) = setup_env().await;
        let engine = ContextEngine::new();

        let req = ContextRequest::new("Authentication");
        let pkg = engine
            .generate_context(&knowledge, &proj, req)
            .await
            .unwrap();

        assert_eq!(pkg.intent, "Authentication");
        assert_eq!(pkg.root_nodes.len(), 1);
        assert_eq!(pkg.root_nodes[0].title, "Authentication");

        // "auth.rs" is related
        assert_eq!(pkg.related_architecture.len(), 1);
        assert_eq!(pkg.related_architecture[0].title, "auth.rs");
    }
}
