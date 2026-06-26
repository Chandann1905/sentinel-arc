//! Search domain types — pure data structures for search queries and results.
//!
//! These types have no dependency on Tantivy or any search backend.
//! They define the contract between consumers and the SearchEngine.

use serde::{Deserialize, Serialize};

use crate::domain::node::NodeStatus;
use crate::types::node_type::NodeType;

/// The kind of entity a search hit represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchEntityKind {
    /// A knowledge node.
    Node,
    /// A relationship between two nodes.
    Relationship,
    /// A data-driven rule.
    Rule,
    /// An immutable event record.
    Event,
}

impl std::fmt::Display for SearchEntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node => write!(f, "node"),
            Self::Relationship => write!(f, "relationship"),
            Self::Rule => write!(f, "rule"),
            Self::Event => write!(f, "event"),
        }
    }
}

/// A single search result hit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchHit {
    /// The ID of the matched entity.
    pub entity_id: String,
    /// What kind of entity this is.
    pub entity_kind: SearchEntityKind,
    /// Relevance score (higher is better).
    pub score: f32,
    /// The matched title or name.
    pub title: String,
    /// Optional snippet with highlighted match context.
    pub snippet: Option<String>,
    /// Entity-specific metadata (node_type, status, etc.).
    pub metadata: serde_json::Value,
}

/// A paginated search response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    /// The matched hits for this page.
    pub hits: Vec<SearchHit>,
    /// Total number of matches across all pages.
    pub total_count: usize,
    /// Current page offset.
    pub offset: usize,
    /// Page size limit.
    pub limit: usize,
}

impl SearchResult {
    /// Create an empty search result.
    pub fn empty() -> Self {
        Self {
            hits: Vec::new(),
            total_count: 0,
            offset: 0,
            limit: 20,
        }
    }
}

/// An advanced search request with filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The raw text query.
    pub query: String,
    /// Filter by entity kind.
    pub entity_kinds: Option<Vec<SearchEntityKind>>,
    /// Filter by NodeType (only applies to node entities).
    pub node_types: Option<Vec<NodeType>>,
    /// Filter by NodeStatus (only applies to node entities).
    pub statuses: Option<Vec<NodeStatus>>,
    /// Filter by tags.
    pub tags: Option<Vec<String>>,
    /// Pagination offset.
    pub offset: usize,
    /// Pagination limit (default 20, max 100).
    pub limit: usize,
    /// Enable fuzzy matching.
    pub fuzzy: bool,
}

impl SearchQuery {
    /// Create a simple text query with defaults.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            entity_kinds: None,
            node_types: None,
            statuses: None,
            tags: None,
            offset: 0,
            limit: 20,
            fuzzy: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_query_defaults() {
        let q = SearchQuery::new("wallet");
        assert_eq!(q.query, "wallet");
        assert_eq!(q.offset, 0);
        assert_eq!(q.limit, 20);
        assert!(!q.fuzzy);
        assert!(q.entity_kinds.is_none());
    }

    #[test]
    fn search_result_empty() {
        let r = SearchResult::empty();
        assert!(r.hits.is_empty());
        assert_eq!(r.total_count, 0);
    }

    #[test]
    fn search_entity_kind_display() {
        assert_eq!(SearchEntityKind::Node.to_string(), "node");
        assert_eq!(SearchEntityKind::Relationship.to_string(), "relationship");
        assert_eq!(SearchEntityKind::Rule.to_string(), "rule");
        assert_eq!(SearchEntityKind::Event.to_string(), "event");
    }

    #[test]
    fn search_hit_serde_roundtrip() {
        let hit = SearchHit {
            entity_id: "node-123".to_string(),
            entity_kind: SearchEntityKind::Node,
            score: 0.95,
            title: "Wallet".to_string(),
            snippet: Some("The <b>Wallet</b> module handles...".to_string()),
            metadata: serde_json::json!({"node_type": "feature"}),
        };
        let json = serde_json::to_string(&hit).unwrap();
        let deserialized: SearchHit = serde_json::from_str(&json).unwrap();
        assert_eq!(hit.entity_id, deserialized.entity_id);
        assert_eq!(hit.entity_kind, deserialized.entity_kind);
        assert_eq!(hit.title, deserialized.title);
    }

    #[test]
    fn search_query_serde_roundtrip() {
        let q = SearchQuery {
            query: "auth".to_string(),
            entity_kinds: Some(vec![SearchEntityKind::Node]),
            node_types: Some(vec![NodeType::Feature]),
            statuses: None,
            tags: Some(vec!["backend".to_string()]),
            offset: 10,
            limit: 50,
            fuzzy: true,
        };
        let json = serde_json::to_string(&q).unwrap();
        let deserialized: SearchQuery = serde_json::from_str(&json).unwrap();
        assert_eq!(q.query, deserialized.query);
        assert_eq!(q.fuzzy, deserialized.fuzzy);
        assert_eq!(q.offset, deserialized.offset);
    }
}
