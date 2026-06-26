use crate::context::{ValidationContext, Validator};
use crate::types::{Severity, ValidationIssue};
use async_trait::async_trait;
use sentinel_arc_core::error::BrainResult;

pub struct IntegrityValidator;

#[async_trait]
impl Validator for IntegrityValidator {
    fn name(&self) -> &'static str {
        "IntegrityValidator"
    }

    async fn validate(&self, ctx: &ValidationContext<'_>) -> BrainResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // 1. Check broken links
        for rel in &ctx.all_relationships {
            if !ctx.node_map.contains_key(&rel.source_node) {
                issues.push(ValidationIssue {
                    rule_name: "broken_relationship".to_string(),
                    severity: Severity::Error,
                    message: format!(
                        "Relationship source node {} does not exist",
                        rel.source_node
                    ),
                    node_id: Some(rel.source_node.clone()),
                    relationship_id: Some(rel.id.clone()),
                });
            }
            if !ctx.node_map.contains_key(&rel.target_node) {
                issues.push(ValidationIssue {
                    rule_name: "broken_relationship".to_string(),
                    severity: Severity::Error,
                    message: format!(
                        "Relationship target node {} does not exist",
                        rel.target_node
                    ),
                    node_id: Some(rel.target_node.clone()),
                    relationship_id: Some(rel.id.clone()),
                });
            }
        }

        // 2. Check orphans
        // A node is an orphan if it does not appear as source or target in any relationship.
        for node in &ctx.all_nodes {
            let has_rels = ctx
                .all_relationships
                .iter()
                .any(|r| r.source_node == node.id || r.target_node == node.id);
            if !has_rels {
                issues.push(ValidationIssue {
                    rule_name: "orphan_node".to_string(),
                    severity: Severity::Warning,
                    message: format!("Node {} ({}) has no relationships", node.title, node.id),
                    node_id: Some(node.id.clone()),
                    relationship_id: None,
                });
            }
        }

        Ok(issues)
    }
}
