use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub enable_write_tools: bool,
    pub server_name: String,
    pub server_version: String,
    pub default_timeout_seconds: u64,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            enable_write_tools: false,
            server_name: "sentinel-arc-mcp".to_string(),
            server_version: "1.0.0".to_string(), // Keep in sync with Cargo.toml
            default_timeout_seconds: 60,
        }
    }
}
