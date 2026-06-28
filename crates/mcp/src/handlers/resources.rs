use crate::types::JsonRpcError;
use sentinel_arc_graph::engine::GraphEngine;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_scanner::engine::ScannerEngine;
use sentinel_arc_timeline::engine::TimelineEngine;
use sentinel_arc_validation::engine::ValidationEngine;
use serde_json::Value;
use std::sync::Arc;

pub fn list_resources() -> Value {
    serde_json::json!([
        {
            "uri": "arc://architecture",
            "name": "Project Architecture Graph",
            "description": "Complete snapshot of the dependency graph in minimized format."
        },
        {
            "uri": "arc://rules",
            "name": "Active Rules",
            "description": "All currently active workspace rules."
        },
        {
            "uri": "arc://timeline/global",
            "name": "Global Timeline",
            "description": "The most recent 50 events in the workspace."
        },
        {
            "uri": "arc://validation/report",
            "name": "Validation Report",
            "description": "The current validation and drift status of the workspace."
        }
    ])
}

pub async fn read_resource(
    uri: &str,
    ke: Arc<KnowledgeEngine>,
    te: Arc<TimelineEngine>,
) -> Result<Value, JsonRpcError> {
    match uri {
        "arc://architecture" => {
            let nodes = ke.list_nodes().await.unwrap_or_default();
            let edges = ke.list_all_relationships().await.unwrap_or_default();
            Ok(serde_json::json!({
                "contents": [
                    {
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string_pretty(&serde_json::json!({
                            "nodes": nodes,
                            "edges": edges
                        })).unwrap_or_default()
                    }
                ]
            }))
        }
        "arc://rules" => match ke.list_rules().await {
            Ok(rules) => Ok(serde_json::json!({
                "contents": [
                    {
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string_pretty(&rules).unwrap_or_default()
                    }
                ]
            })),
            Err(e) => Err(JsonRpcError {
                code: -32000,
                message: e.to_string(),
                data: None,
            }),
        },
        "arc://timeline/global" => match te.generate_project_timeline(50).await {
            Ok(timeline) => Ok(serde_json::json!({
                "contents": [
                    {
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string_pretty(&timeline).unwrap_or_default()
                    }
                ]
            })),
            Err(e) => Err(JsonRpcError {
                code: -32000,
                message: e.to_string(),
                data: None,
            }),
        },
        "arc://validation/report" => {
            let proj = GraphEngine::build_projection(&ke)
                .await
                .map_err(|e| JsonRpcError {
                    code: -32000,
                    message: format!("Graph projection failed: {}", e),
                    data: None,
                })?;
            let scanner = ScannerEngine::new();
            let val_engine = ValidationEngine::new();
            match val_engine.run_full_validation(&ke, &proj, &scanner).await {
                Ok(report) => Ok(serde_json::json!({
                    "contents": [
                        {
                            "uri": uri,
                            "mimeType": "application/json",
                            "text": serde_json::to_string_pretty(&report).unwrap_or_default()
                        }
                    ]
                })),
                Err(e) => Err(JsonRpcError {
                    code: -32000,
                    message: e.to_string(),
                    data: None,
                }),
            }
        }
        _ => Err(JsonRpcError {
            code: -32602,
            message: format!("Resource '{}' not found", uri),
            data: None,
        }),
    }
}
