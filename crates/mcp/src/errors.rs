use crate::types::JsonRpcError;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Parse error")]
    ParseError,
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Method not found")]
    MethodNotFound,
    #[error("Invalid params: {0}")]
    InvalidParams(String),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Sentinel Arc error: {0}")]
    SentinelError(#[from] sentinel_arc_core::error::BrainError),
    #[error("Security error: {0}")]
    SecurityError(String),
}

impl McpError {
    pub fn to_json_rpc_error(&self) -> JsonRpcError {
        match self {
            McpError::ParseError => JsonRpcError {
                code: -32700,
                message: "Parse error".to_string(),
                data: None,
            },
            McpError::InvalidRequest => JsonRpcError {
                code: -32600,
                message: "Invalid Request".to_string(),
                data: None,
            },
            McpError::MethodNotFound => JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            },
            McpError::InvalidParams(msg) => JsonRpcError {
                code: -32602,
                message: "Invalid params".to_string(),
                data: Some(Value::String(msg.clone())),
            },
            McpError::InternalError(msg) => JsonRpcError {
                code: -32603,
                message: "Internal error".to_string(),
                data: Some(Value::String(msg.clone())),
            },
            McpError::SentinelError(err) => JsonRpcError {
                code: -32000,
                message: "Server error".to_string(),
                data: Some(Value::String(err.to_string())),
            },
            McpError::SecurityError(msg) => JsonRpcError {
                code: -32001,
                message: "Security error".to_string(),
                data: Some(Value::String(msg.clone())),
            },
        }
    }
}

impl From<anyhow::Error> for McpError {
    fn from(err: anyhow::Error) -> Self {
        McpError::InternalError(err.to_string())
    }
}
