use crate::context::{ValidationContext, Validator};
use crate::types::{Severity, ValidationIssue};
use async_trait::async_trait;
use sentinel_arc_core::error::BrainResult;

pub struct GraphValidator;

#[async_trait]
impl Validator for GraphValidator {
    fn name(&self) -> &'static str {
        "GraphValidator"
    }

    async fn validate(&self, ctx: &ValidationContext<'_>) -> BrainResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        // 1. Direct self-referential cycles
        for rel in &ctx.all_relationships {
            if rel.source_node == rel.target_node {
                if let Some(node) = ctx.node_map.get(&rel.source_node) {
                    issues.push(ValidationIssue {
                        rule_name: "circular_dependency".to_string(),
                        severity: Severity::Error,
                        message: format!(
                            "Node {} ({}) has a self-referential relationship",
                            node.title, node.id
                        ),
                        node_id: Some(node.id.clone()),
                        relationship_id: Some(rel.id.clone()),
                    });
                }
            }
        }

        // 2. Larger circular dependencies.
        // If a node's child eventually depends back on the node, it's a cycle.
        for node in &ctx.all_nodes {
            let descendants = ctx.projection.get_descendants(&node.id);
            for child in descendants {
                let child_deps = ctx.projection.find_dependencies(&child, None);
                if child_deps.contains(&node.id) {
                    issues.push(ValidationIssue {
                        rule_name: "circular_dependency".to_string(),
                        severity: Severity::Error,
                        message: format!(
                            "Node {} ({}) is part of a circular dependency",
                            node.title, node.id
                        ),
                        node_id: Some(node.id.clone()),
                        relationship_id: None,
                    });
                    break; // Report at most once per node for cycles
                }
            }
        }

        Ok(issues)
    }
}
