use crate::types::JsonRpcError;
use sentinel_arc_context::engine::ContextEngine;
use sentinel_arc_context::types::ContextRequest;
use sentinel_arc_core::{NodeId, NodeType, SearchEntityKind, SearchQuery};
use sentinel_arc_graph::engine::GraphEngine;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_timeline::engine::TimelineEngine;
use serde_json::Value;
use std::sync::Arc;

pub fn list_tools() -> Value {
    serde_json::json!([
        {
            "name": "search_nodes",
            "description": "Search for nodes in the knowledge graph by title or tags.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "node_type": { "type": "string", "description": "Optional type (e.g. Feature, Bug, Decision)" },
                    "limit": { "type": "integer", "description": "Max results" }
                },
                "required": ["query"]
            }
        },
        {
            "name": "get_node",
            "description": "Get a specific node and its details by ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "node_id": { "type": "string" }
                },
                "required": ["node_id"]
            }
        },
        {
            "name": "generate_context",
            "description": "Generate a context package based on a natural language intent.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "intent": { "type": "string" }
                },
                "required": ["intent"]
            }
        },
        {
            "name": "generate_timeline",
            "description": "Generate a chronological timeline of events.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "node_id": { "type": "string", "description": "Optional specific node" },
                    "decisions_only": { "type": "boolean" }
                }
            }
        }
    ])
}

pub async fn call_tool(
    name: &str,
    arguments: Value,
    ke: Arc<KnowledgeEngine>,
    te: Arc<TimelineEngine>,
) -> Result<Value, JsonRpcError> {
    match name {
        "search_nodes" => {
            let query = arguments
                .get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let limit = arguments
                .get("limit")
                .and_then(|v| v.as_u64())
                .unwrap_or(10) as usize;

            let node_type_filter = arguments
                .get("node_type")
                .and_then(|v| v.as_str())
                .and_then(|s| match s.to_lowercase().as_str() {
                    "feature" => Some(NodeType::Feature),
                    "bug" => Some(NodeType::Bug),
                    "decision" => Some(NodeType::Decision),
                    _ => None,
                });

            let mut node_types = Vec::new();
            if let Some(nt) = node_type_filter {
                node_types.push(nt);
            }

            let sq = SearchQuery {
                query: query.to_string(),
                entity_kinds: Some(vec![SearchEntityKind::Node]),
                node_types: if node_types.is_empty() {
                    None
                } else {
                    Some(node_types)
                },
                statuses: None,
                tags: None,
                offset: 0,
                limit,
                fuzzy: false,
            };

            match ke.search_advanced(&sq) {
                Ok(results) => {
                    let mut output = Vec::new();
                    for res in results.hits {
                        if let Ok(node) = ke.get_node(&NodeId::from(res.entity_id.as_str())).await {
                            output.push(serde_json::json!({
                                "id": node.id.as_str(),
                                "type": format!("{:?}", node.node_type),
                                "title": node.title,
                                "status": format!("{:?}", node.status),
                                "score": res.score
                            }));
                        }
                    }
                    Ok(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&output).unwrap()
                            }
                        ]
                    }))
                }
                Err(e) => Err(JsonRpcError {
                    code: -32000,
                    message: format!("Search failed: {}", e),
                    data: None,
                }),
            }
        }
        "get_node" => {
            let node_id_str = arguments
                .get("node_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| JsonRpcError {
                    code: -32602,
                    message: "Missing node_id".to_string(),
                    data: None,
                })?;

            let node_id = NodeId::from(node_id_str);
            match ke.get_node(&node_id).await {
                Ok(node) => Ok(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&node).unwrap()
                        }
                    ]
                })),
                Err(e) => Err(JsonRpcError {
                    code: -32000,
                    message: format!("Node not found: {}", e),
                    data: None,
                }),
            }
        }
        "generate_context" => {
            let intent = arguments
                .get("intent")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let proj = GraphEngine::build_projection(&ke)
                .await
                .map_err(|e| JsonRpcError {
                    code: -32000,
                    message: format!("Graph projection failed: {}", e),
                    data: None,
                })?;

            let context_engine = ContextEngine::new();
            let req = ContextRequest::new(intent);

            match context_engine.generate_context(&ke, &proj, req).await {
                Ok(pkg) => Ok(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&pkg).unwrap_or_default()
                        }
                    ]
                })),
                Err(e) => Err(JsonRpcError {
                    code: -32000,
                    message: format!("Context generation failed: {}", e),
                    data: None,
                }),
            }
        }
        "generate_timeline" => {
            let node_id_opt = arguments.get("node_id").and_then(|v| v.as_str());
            let decisions_only = arguments
                .get("decisions_only")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let result = if decisions_only {
                te.generate_decision_history().await
            } else if let Some(id_str) = node_id_opt {
                let node_id = NodeId::from(id_str);
                te.generate_node_timeline(&node_id).await
            } else {
                te.generate_project_timeline(50).await
            };

            match result {
                Ok(timeline) => Ok(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&timeline).unwrap()
                        }
                    ]
                })),
                Err(e) => Err(JsonRpcError {
                    code: -32000,
                    message: format!("Timeline generation failed: {}", e),
                    data: None,
                }),
            }
        }
        _ => Err(JsonRpcError {
            code: -32601,
            message: format!("Tool '{}' not found", name),
            data: None,
        }),
    }
}
