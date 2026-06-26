//! The Node entity — the primary object of Sentinel Arc.
//!
//! A Node represents any important entity inside the system: projects,
//! features, files, functions, decisions, bugs, tasks, etc.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ids::NodeId;
use crate::types::node_type::NodeType;
use crate::types::source::{Confidence, Source};

/// The status of a node in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    /// Active and current.
    #[default]
    Active,
    /// Archived (soft-deleted, still queryable).
    Archived,
    /// Draft, not yet finalized.
    Draft,
    /// Deprecated but still present.
    Deprecated,
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Archived => write!(f, "Archived"),
            Self::Draft => write!(f, "Draft"),
            Self::Deprecated => write!(f, "Deprecated"),
        }
    }
}

/// A knowledge node — the primary object of Sentinel Arc.
///
/// Every important entity in the system is a Node. The universal schema
/// is defined in DOMAIN_MODEL.md.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier.
    pub id: NodeId,

    /// The type of this node (Project, Feature, File, etc.).
    #[serde(rename = "type")]
    pub node_type: NodeType,

    /// Human-readable title.
    pub title: String,

    /// Detailed description.
    pub description: String,

    /// Current status.
    pub status: NodeStatus,

    /// Where this knowledge originated.
    pub source: Source,

    /// How trustworthy this knowledge is (0–100).
    pub confidence: Confidence,

    /// Arbitrary key-value metadata (JSON object).
    /// Tags are stored here under the "tags" key as a JSON array.
    pub metadata: serde_json::Value,

    /// Version number, incremented on every update.
    pub version: u32,

    /// When this node was first created.
    pub created_at: DateTime<Utc>,

    /// When this node was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Node {
    /// Create a new Node with the given type and title.
    ///
    /// All other fields are set to sensible defaults:
    /// - Status: Active
    /// - Source: Manual
    /// - Confidence: from source default
    /// - Version: 1
    /// - Timestamps: now
    pub fn new(node_type: NodeType, title: impl Into<String>) -> Self {
        let now = Utc::now();
        let source = Source::Manual;
        Self {
            id: NodeId::new(),
            node_type,
            title: title.into(),
            description: String::new(),
            status: NodeStatus::Active,
            source,
            confidence: source.default_confidence(),
            metadata: serde_json::json!({}),
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }

    /// Retrieve tags from metadata, if present.
    pub fn tags(&self) -> Vec<String> {
        self.metadata
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Set tags in metadata.
    pub fn set_tags(&mut self, tags: Vec<String>) {
        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(
                "tags".to_string(),
                serde_json::Value::Array(tags.into_iter().map(serde_json::Value::String).collect()),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_has_correct_defaults() {
        let node = Node::new(NodeType::Feature, "Wallet");
        assert_eq!(node.node_type, NodeType::Feature);
        assert_eq!(node.title, "Wallet");
        assert_eq!(node.status, NodeStatus::Active);
        assert_eq!(node.source, Source::Manual);
        assert_eq!(node.confidence.value(), 40); // Manual default
        assert_eq!(node.version, 1);
        assert!(node.description.is_empty());
    }

    #[test]
    fn tags_roundtrip() {
        let mut node = Node::new(NodeType::Feature, "Test");
        assert!(node.tags().is_empty());

        node.set_tags(vec!["backend".into(), "wallet".into()]);
        let tags = node.tags();
        assert_eq!(tags, vec!["backend", "wallet"]);
    }

    #[test]
    fn node_serde_roundtrip() {
        let mut node = Node::new(NodeType::Api, "POST /wallet");
        node.description = "Create wallet endpoint".into();
        node.set_tags(vec!["api".into()]);

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: Node = serde_json::from_str(&json).unwrap();

        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.node_type, deserialized.node_type);
        assert_eq!(node.title, deserialized.title);
        assert_eq!(node.description, deserialized.description);
        assert_eq!(node.tags(), deserialized.tags());
    }

    #[test]
    fn node_status_default_is_active() {
        assert_eq!(NodeStatus::default(), NodeStatus::Active);
    }
}
