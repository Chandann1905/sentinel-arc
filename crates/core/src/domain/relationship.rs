//! The Relationship entity — first-class connections between nodes.
//!
//! Relationships are never embedded inside business logic or hidden
//! in metadata. They are always queryable entities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ids::{NodeId, RelationshipId};
use crate::types::relationship_type::RelationshipType;
use crate::types::source::Confidence;

/// A first-class relationship connecting two nodes.
///
/// Schema defined in DOMAIN_MODEL.md.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier.
    pub id: RelationshipId,

    /// The source (from) node.
    pub source_node: NodeId,

    /// The target (to) node.
    pub target_node: NodeId,

    /// The type of relationship.
    pub relationship_type: RelationshipType,

    /// How confident we are in this relationship (0–100).
    pub confidence: Confidence,

    /// Arbitrary key-value metadata.
    pub metadata: serde_json::Value,

    /// When this relationship was created.
    pub created_at: DateTime<Utc>,
}

impl Relationship {
    /// Create a new relationship between two nodes.
    pub fn new(
        source_node: NodeId,
        target_node: NodeId,
        relationship_type: RelationshipType,
    ) -> Self {
        Self {
            id: RelationshipId::new(),
            source_node,
            target_node,
            relationship_type,
            confidence: Confidence::new(100),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_relationship_has_correct_defaults() {
        let src = NodeId::from_string("node-a");
        let tgt = NodeId::from_string("node-b");
        let rel = Relationship::new(src.clone(), tgt.clone(), RelationshipType::DependsOn);

        assert_eq!(rel.source_node, src);
        assert_eq!(rel.target_node, tgt);
        assert_eq!(rel.relationship_type, RelationshipType::DependsOn);
        assert_eq!(rel.confidence.value(), 100);
    }

    #[test]
    fn relationship_serde_roundtrip() {
        let rel = Relationship::new(
            NodeId::from_string("a"),
            NodeId::from_string("b"),
            RelationshipType::Calls,
        );

        let json = serde_json::to_string(&rel).unwrap();
        let deserialized: Relationship = serde_json::from_str(&json).unwrap();

        assert_eq!(rel.id, deserialized.id);
        assert_eq!(rel.source_node, deserialized.source_node);
        assert_eq!(rel.target_node, deserialized.target_node);
        assert_eq!(rel.relationship_type, deserialized.relationship_type);
    }
}
