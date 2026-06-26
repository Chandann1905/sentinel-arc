use crate::types::ImpactReport;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_core::types::node_type::NodeType;
use sentinel_arc_graph::projection::GraphProjection;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::collections::HashSet;

/// Calculates the impact report for a given node.
pub async fn calculate_impact(
    knowledge: &KnowledgeEngine,
    projection: &GraphProjection,
    root_id: &NodeId,
) -> BrainResult<ImpactReport> {
    // Traverse all nodes that DEPEND ON this root (direct & transitive impact).
    // We pass None for max_depth to find the full blast radius.
    let impacted_ids = projection.find_impact(root_id, None);

    let mut affected_features = Vec::new();
    let mut affected_files = Vec::new();

    let mut unique_nodes = HashSet::new();

    for id in impacted_ids {
        unique_nodes.insert(id.clone());
        if let Ok(node) = knowledge.get_node(&id).await {
            match node.node_type {
                NodeType::Feature => affected_features.push(node),
                NodeType::File => affected_files.push(node),
                _ => {}
            }
        }
    }

    // A simple heuristic for Risk and Complexity
    let complexity_score = (unique_nodes.len() * 5).min(100) as u8;

    // Risk goes up if many features or files are affected
    let risk_score = ((affected_features.len() * 10) + (affected_files.len() * 2)).min(100) as u8;

    Ok(ImpactReport {
        affected_features,
        affected_files,
        risk_score,
        complexity_score,
    })
}
