use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::rule::Rule;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_core::types::node_type::NodeType;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::collections::HashSet;

pub struct EnrichmentData {
    pub related_architecture: Vec<Node>,
    pub known_decisions: Vec<Node>,
    pub relevant_rules: Vec<Rule>,
}

/// Fetches detailed nodes for the expanded topology and global context (rules, decisions).
pub async fn fetch_enrichment(
    knowledge: &KnowledgeEngine,
    expanded_ids: &HashSet<NodeId>,
) -> BrainResult<EnrichmentData> {
    let mut related_architecture = Vec::new();
    let mut known_decisions = Vec::new();

    for id in expanded_ids {
        if let Ok(node) = knowledge.get_node(id).await {
            match node.node_type {
                NodeType::Decision => known_decisions.push(node),
                _ => related_architecture.push(node),
            }
        }
    }

    // Currently we just fetch all enabled rules.
    // In the future, we could filter by tags or category based on the root nodes.
    let relevant_rules = knowledge.list_rules().await?;

    Ok(EnrichmentData {
        related_architecture,
        known_decisions,
        relevant_rules,
    })
}
