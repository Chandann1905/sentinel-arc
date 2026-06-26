use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::search::{SearchEntityKind, SearchQuery};
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;

/// Discovers the root nodes matching the user's intent.
pub async fn discover_roots(
    knowledge: &KnowledgeEngine,
    intent: &str,
    max_nodes: usize,
) -> BrainResult<Vec<Node>> {
    let query = SearchQuery {
        query: intent.to_string(),
        entity_kinds: Some(vec![SearchEntityKind::Node]),
        node_types: None,
        statuses: None,
        tags: None,
        offset: 0,
        limit: max_nodes,
        fuzzy: true, // Allow fuzzy matching for intent discovery
    };

    let result = knowledge.search_advanced(&query)?;

    let mut nodes = Vec::new();
    for hit in result.hits {
        let id = NodeId::from_string(hit.entity_id);
        // We use get_node to ensure we fetch the complete object from SQLite
        if let Ok(node) = knowledge.get_node(&id).await {
            nodes.push(node);
        }
    }

    Ok(nodes)
}
