pub mod prompts;
pub mod resources;
pub mod tools;

use crate::types::{JsonRpcRequest, JsonRpcResponse};
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use sentinel_arc_timeline::engine::TimelineEngine;
use std::sync::Arc;
use std::time::Duration;

pub async fn handle_request(
    request: JsonRpcRequest,
    ke: Arc<KnowledgeEngine>,
    te: Arc<TimelineEngine>,
    default_timeout: u64,
) -> Option<JsonRpcResponse> {
    // According to MCP spec, the methods are defined as e.g. "tools/call", "resources/read", etc.
    // Plus "initialize"

    // Wrap the core logic in a timeout
    let timeout_duration = Duration::from_secs(default_timeout);
    let req_id = request.id.clone();
    let method = request.method.clone();

    let result = tokio::time::timeout(timeout_duration, async move {
        match request.method.as_str() {
            "initialize" => {
                let result = serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {},
                        "resources": {},
                        "prompts": {}
                    },
                    "serverInfo": {
                        "name": "sentinel-arc-mcp",
                        "version": "1.0.0"
                    }
                });
                Some(JsonRpcResponse::success(request.id?, result))
            }
            "notifications/initialized" => {
                // Client is initialized, no response needed
                None
            }
            "tools/list" => {
                let tools = tools::list_tools();
                Some(JsonRpcResponse::success(
                    request.id?,
                    serde_json::json!({ "tools": tools }),
                ))
            }
            "tools/call" => {
                if let Some(params) = request.params {
                    if let Some(name) = params.get("name").and_then(|n| n.as_str()) {
                        let arguments = params
                            .get("arguments")
                            .cloned()
                            .unwrap_or(serde_json::json!({}));
                        match tools::call_tool(name, arguments, ke.clone(), te.clone()).await {
                            Ok(result) => Some(JsonRpcResponse::success(request.id?, result)),
                            Err(err) => Some(JsonRpcResponse::error(
                                request.id,
                                err.code,
                                err.message,
                                err.data,
                            )),
                        }
                    } else {
                        Some(JsonRpcResponse::error(
                            request.id,
                            -32602,
                            "Missing 'name' in params".to_string(),
                            None,
                        ))
                    }
                } else {
                    Some(JsonRpcResponse::error(
                        request.id,
                        -32602,
                        "Missing params".to_string(),
                        None,
                    ))
                }
            }
            "resources/list" => {
                let resources = resources::list_resources();
                Some(JsonRpcResponse::success(
                    request.id?,
                    serde_json::json!({ "resources": resources }),
                ))
            }
            "resources/read" => {
                if let Some(params) = request.params {
                    if let Some(uri) = params.get("uri").and_then(|u| u.as_str()) {
                        match resources::read_resource(uri, ke.clone(), te.clone()).await {
                            Ok(result) => Some(JsonRpcResponse::success(request.id?, result)),
                            Err(err) => Some(JsonRpcResponse::error(
                                request.id,
                                err.code,
                                err.message,
                                err.data,
                            )),
                        }
                    } else {
                        Some(JsonRpcResponse::error(
                            request.id,
                            -32602,
                            "Missing 'uri' in params".to_string(),
                            None,
                        ))
                    }
                } else {
                    Some(JsonRpcResponse::error(
                        request.id,
                        -32602,
                        "Missing params".to_string(),
                        None,
                    ))
                }
            }
            "prompts/list" => {
                let prompts = prompts::list_prompts();
                Some(JsonRpcResponse::success(
                    request.id?,
                    serde_json::json!({ "prompts": prompts }),
                ))
            }
            "prompts/get" => {
                if let Some(params) = request.params {
                    if let Some(name) = params.get("name").and_then(|n| n.as_str()) {
                        let arguments = params
                            .get("arguments")
                            .cloned()
                            .unwrap_or(serde_json::json!({}));
                        match prompts::get_prompt(name, arguments).await {
                            Ok(result) => Some(JsonRpcResponse::success(request.id?, result)),
                            Err(err) => Some(JsonRpcResponse::error(
                                request.id,
                                err.code,
                                err.message,
                                err.data,
                            )),
                        }
                    } else {
                        Some(JsonRpcResponse::error(
                            request.id,
                            -32602,
                            "Missing 'name' in params".to_string(),
                            None,
                        ))
                    }
                } else {
                    Some(JsonRpcResponse::error(
                        request.id,
                        -32602,
                        "Missing params".to_string(),
                        None,
                    ))
                }
            }
            _ => Some(JsonRpcResponse::error(
                request.id,
                -32601,
                format!("Method '{}' not found", request.method),
                None,
            )),
        }
    })
    .await;

    match result {
        Ok(opt_resp) => {
            if let Some(resp) = opt_resp {
                Some(resp)
            } else {
                // If it's a notification, or missing ID but not a notification
                if req_id.is_none()
                    && method != "notifications/initialized"
                    && method != "notifications/cancelled"
                {
                    Some(JsonRpcResponse::error(
                        None,
                        -32600,
                        "Invalid Request: missing id".to_string(),
                        None,
                    ))
                } else {
                    None
                }
            }
        }
        Err(_) => {
            // Timeout occurred
            Some(JsonRpcResponse::error(
                req_id,
                -32000,
                "Request timed out".to_string(),
                None,
            ))
        }
    }
}
