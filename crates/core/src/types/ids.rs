//! Type-safe identifier wrappers for all core entities.
//!
//! Each ID wraps a UUID v4 string to prevent mixing up IDs from different
//! entity types at compile time.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Macro to generate a type-safe ID wrapper around a UUID string.
macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Create a new random ID using UUID v4.
            pub fn new() -> Self {
                Self(Uuid::new_v4().to_string())
            }

            /// Create an ID from an existing string value.
            ///
            /// Use this when loading IDs from storage. The caller is responsible
            /// for ensuring the string is a valid identifier.
            pub fn from_string(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            /// Return the underlying string representation.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }
    };
}

define_id!(
    /// Unique identifier for a [`Node`](crate::Node).
    NodeId
);

define_id!(
    /// Unique identifier for a [`Relationship`](crate::Relationship).
    RelationshipId
);

define_id!(
    /// Unique identifier for an [`Event`](crate::Event).
    EventId
);

define_id!(
    /// Unique identifier for a [`Rule`](crate::Rule).
    RuleId
);

define_id!(
    /// Unique identifier for plugin data entries.
    PluginDataId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_new_generates_unique_ids() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn node_id_from_string_roundtrips() {
        let original = "test-node-123";
        let id = NodeId::from_string(original);
        assert_eq!(id.as_str(), original);
        assert_eq!(id.to_string(), original);
    }

    #[test]
    fn node_id_equality() {
        let id1 = NodeId::from_string("same");
        let id2 = NodeId::from_string("same");
        assert_eq!(id1, id2);
    }

    #[test]
    fn node_id_serialization_roundtrip() {
        let id = NodeId::from_string("serialize-me");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"serialize-me\"");
        let deserialized: NodeId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn relationship_id_serialization_roundtrip() {
        let id = RelationshipId::from_string("rel-001");
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: RelationshipId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn event_id_serialization_roundtrip() {
        let id = EventId::from_string("evt-001");
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: EventId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn rule_id_serialization_roundtrip() {
        let id = RuleId::from_string("rule-001");
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: RuleId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn different_id_types_are_distinct() {
        // This test verifies at a conceptual level that ID types are not interchangeable.
        // The actual compile-time safety is enforced by the type system.
        let node_id = NodeId::from_string("id-1");
        let rel_id = RelationshipId::from_string("id-1");
        // They hold the same string but are different types — cannot be compared directly.
        assert_eq!(node_id.as_str(), rel_id.as_str());
    }
}
