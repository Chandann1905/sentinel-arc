use crate::types::JsonRpcError;
use serde_json::Value;

pub fn list_prompts() -> Value {
    serde_json::json!([
        {
            "name": "architecture_review",
            "description": "Reviews the project architecture against active rules.",
            "arguments": []
        },
        {
            "name": "feature_implementation",
            "description": "Prepares context for implementing a new feature safely.",
            "arguments": [
                {
                    "name": "feature_name",
                    "description": "Name of the feature to implement",
                    "required": true
                }
            ]
        }
    ])
}

pub async fn get_prompt(name: &str, arguments: Value) -> Result<Value, JsonRpcError> {
    match name {
        "architecture_review" => Ok(serde_json::json!({
            "description": "Perform a complete architecture review.",
            "messages": [
                {
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": "Please review the architecture graph and validation report. Identify any existing drift or rule violations and explain how we can address them based on the active rules."
                    }
                }
            ]
        })),
        "feature_implementation" => {
            let feature = arguments
                .get("feature_name")
                .and_then(|v| v.as_str())
                .unwrap_or("New Feature");
            Ok(serde_json::json!({
                "description": "Safe feature implementation guide.",
                "messages": [
                    {
                        "role": "user",
                        "content": {
                            "type": "text",
                            "text": format!("I need to implement '{}'. First, use the 'generate_context' tool to gather context about this domain. Then suggest an implementation plan that avoids architectural drift.", feature)
                        }
                    }
                ]
            }))
        }
        _ => Err(JsonRpcError {
            code: -32602,
            message: format!("Prompt '{}' not found", name),
            data: None,
        }),
    }
}
