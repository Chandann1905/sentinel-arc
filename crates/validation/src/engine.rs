use crate::context::{ValidationContext, Validator};
use crate::types::{Severity, ValidationReport};
use crate::validators::graph::GraphValidator;
use crate::validators::integrity::IntegrityValidator;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_graph::projection::GraphProjection;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_scanner::engine::ScannerEngine;

pub struct ValidationEngine {
    validators: Vec<Box<dyn Validator>>,
}

impl ValidationEngine {
    pub fn new() -> Self {
        Self {
            validators: vec![Box::new(IntegrityValidator), Box::new(GraphValidator)],
        }
    }

    /// Runs all validators and builds a full validation report.
    pub async fn run_full_validation(
        &self,
        knowledge: &KnowledgeEngine,
        projection: &GraphProjection,
        scanner: &ScannerEngine,
    ) -> BrainResult<ValidationReport> {
        let ctx = ValidationContext::new(knowledge, projection, scanner).await?;

        let mut all_issues = Vec::new();

        for validator in &self.validators {
            let mut issues = validator.validate(&ctx).await?;
            all_issues.append(&mut issues);
        }

        // Sort issues by severity (Error first)
        all_issues.sort_by(|a, b| {
            let rank = |sev: &Severity| match sev {
                Severity::Error => 0,
                Severity::Warning => 1,
                Severity::Info => 2,
            };
            rank(&a.severity).cmp(&rank(&b.severity))
        });

        Ok(ValidationReport::build(all_issues))
    }
}

impl Default for ValidationEngine {
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

    async fn setup_env() -> (KnowledgeEngine, GraphProjection, ScannerEngine) {
        let dir = tempdir().unwrap();
        let db = Database::init(dir.path()).await.unwrap();
        let knowledge = KnowledgeEngine::new(&db);
        let scanner = ScannerEngine::new();

        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let n3 = Node::new(NodeType::Feature, "N3"); // Orphan
        let (n1, _) = knowledge.create_node(n1).await.unwrap();
        let (n2, _) = knowledge.create_node(n2).await.unwrap();
        let _ = knowledge.create_node(n3).await.unwrap();

        // N1 -> N2
        let rel1 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        knowledge.create_relationship(rel1).await.unwrap();

        // Circular: N2 -> N1
        let rel2 = Relationship::new(n2.id.clone(), n1.id.clone(), RelationshipType::DependsOn);
        knowledge.create_relationship(rel2).await.unwrap();

        // Let the search index commit
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let proj = GraphEngine::build_projection(&knowledge).await.unwrap();

        (knowledge, proj, scanner)
    }

    #[tokio::test]
    async fn test_validation_engine() {
        let (knowledge, proj, scanner) = setup_env().await;
        let engine = ValidationEngine::new();

        let report = engine
            .run_full_validation(&knowledge, &proj, &scanner)
            .await
            .unwrap();

        assert!(report.total_issues > 0);

        // We expect errors for circular dependencies (N1 and N2)
        assert!(report.error_count >= 2);

        // We expect a warning for orphan node (N3)
        assert!(report.warning_count >= 1);
    }
}
