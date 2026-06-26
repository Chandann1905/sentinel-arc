//! All event types supported by Sentinel Arc.
//!
//! Derived from DOMAIN_MODEL.md. Events are immutable records of every
//! meaningful change — they are never edited or deleted.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of event recorded in the event log.
///
/// Events form the complete audit trail of the knowledge base.
/// They are append-only and immutable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// A new node was created.
    NodeCreated,
    /// An existing node was updated.
    NodeUpdated,
    /// A node was archived (soft-deleted).
    NodeArchived,
    /// A new relationship was added between two nodes.
    RelationshipAdded,
    /// A relationship was removed.
    RelationshipRemoved,
    /// A new rule was added.
    RuleAdded,
    /// An existing rule was updated.
    RuleUpdated,
    /// An architecture decision was accepted.
    DecisionAccepted,
    /// An architecture decision was rejected.
    DecisionRejected,
    /// A new bug was opened.
    BugOpened,
    /// A bug was closed.
    BugClosed,
    /// A roadmap item was added.
    RoadmapAdded,
    /// A roadmap item was completed.
    RoadmapCompleted,
    /// A feature was released.
    FeatureReleased,
}

impl EventType {
    /// Return all known event types.
    pub fn all() -> &'static [EventType] {
        &[
            Self::NodeCreated,
            Self::NodeUpdated,
            Self::NodeArchived,
            Self::RelationshipAdded,
            Self::RelationshipRemoved,
            Self::RuleAdded,
            Self::RuleUpdated,
            Self::DecisionAccepted,
            Self::DecisionRejected,
            Self::BugOpened,
            Self::BugClosed,
            Self::RoadmapAdded,
            Self::RoadmapCompleted,
            Self::FeatureReleased,
        ]
    }

    /// Whether this event type relates to node mutations.
    pub fn is_node_event(&self) -> bool {
        matches!(
            self,
            Self::NodeCreated | Self::NodeUpdated | Self::NodeArchived
        )
    }

    /// Whether this event type relates to relationship mutations.
    pub fn is_relationship_event(&self) -> bool {
        matches!(self, Self::RelationshipAdded | Self::RelationshipRemoved)
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeCreated => write!(f, "Node Created"),
            Self::NodeUpdated => write!(f, "Node Updated"),
            Self::NodeArchived => write!(f, "Node Archived"),
            Self::RelationshipAdded => write!(f, "Relationship Added"),
            Self::RelationshipRemoved => write!(f, "Relationship Removed"),
            Self::RuleAdded => write!(f, "Rule Added"),
            Self::RuleUpdated => write!(f, "Rule Updated"),
            Self::DecisionAccepted => write!(f, "Decision Accepted"),
            Self::DecisionRejected => write!(f, "Decision Rejected"),
            Self::BugOpened => write!(f, "Bug Opened"),
            Self::BugClosed => write!(f, "Bug Closed"),
            Self::RoadmapAdded => write!(f, "Roadmap Added"),
            Self::RoadmapCompleted => write!(f, "Roadmap Completed"),
            Self::FeatureReleased => write!(f, "Feature Released"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_all_variants() {
        for et in EventType::all() {
            let json = serde_json::to_string(et).unwrap();
            let deserialized: EventType = serde_json::from_str(&json).unwrap();
            assert_eq!(*et, deserialized, "Failed roundtrip for {et:?}");
        }
    }

    #[test]
    fn serialization_uses_snake_case() {
        assert_eq!(
            serde_json::to_string(&EventType::NodeCreated).unwrap(),
            "\"node_created\""
        );
        assert_eq!(
            serde_json::to_string(&EventType::RelationshipAdded).unwrap(),
            "\"relationship_added\""
        );
        assert_eq!(
            serde_json::to_string(&EventType::FeatureReleased).unwrap(),
            "\"feature_released\""
        );
    }

    #[test]
    fn event_classification() {
        assert!(EventType::NodeCreated.is_node_event());
        assert!(EventType::NodeUpdated.is_node_event());
        assert!(EventType::NodeArchived.is_node_event());
        assert!(!EventType::RelationshipAdded.is_node_event());

        assert!(EventType::RelationshipAdded.is_relationship_event());
        assert!(EventType::RelationshipRemoved.is_relationship_event());
        assert!(!EventType::NodeCreated.is_relationship_event());
    }

    #[test]
    fn all_returns_14_types() {
        assert_eq!(EventType::all().len(), 14);
    }
}
