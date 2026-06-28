use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<Value>, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_request_parsing() {
        let raw = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "search_nodes"
            }
        }"#;

        let req: JsonRpcRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, Some(serde_json::json!(1)));
        assert_eq!(req.method, "tools/call");
        assert!(req.params.is_some());
    }

    #[test]
    fn test_jsonrpc_response_success() {
        let res =
            JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({"status": "ok"}));
        let serialized = serde_json::to_string(&res).unwrap();

        assert!(serialized.contains(r#""jsonrpc":"2.0""#));
        assert!(serialized.contains(r#""id":1"#));
        assert!(serialized.contains(r#""result":{"status":"ok"}"#));
        assert!(!serialized.contains("error"));
    }

    #[test]
    fn test_jsonrpc_response_error() {
        let res = JsonRpcResponse::error(
            Some(serde_json::json!(1)),
            -32600,
            "Invalid Request".to_string(),
            None,
        );
        let serialized = serde_json::to_string(&res).unwrap();

        assert!(serialized.contains(r#""jsonrpc":"2.0""#));
        assert!(serialized.contains(r#""id":1"#));
        assert!(serialized.contains(r#""error":{"code":-32600,"message":"Invalid Request"}"#));
        assert!(!serialized.contains("result"));
    }
    #[test]
    fn test_jsonrpc_request_missing_id_is_valid_struct() {
        // According to JSON-RPC 2.0, requests without an ID are valid structures (notifications)
        let raw = r#"{
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }"#;
        let req: JsonRpcRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, None);
        assert_eq!(req.method, "notifications/initialized");
    }
}
