//! The Event entity — immutable audit trail.
//!
//! Events are never edited, never deleted. They record every meaningful
//! change in the knowledge base for full history reconstruction.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::event_type::EventType;
use crate::types::ids::EventId;

/// An immutable event recording a meaningful change.
///
/// Schema defined in DOMAIN_MODEL.md.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier.
    pub id: EventId,

    /// What kind of change occurred.
    pub event_type: EventType,

    /// The entity (node, relationship, rule) this event relates to.
    pub entity_id: String,

    /// Detailed payload with before/after state or relevant data.
    pub payload: serde_json::Value,

    /// When this event occurred.
    pub timestamp: DateTime<Utc>,

    /// Who or what caused this event (user, system, scanner, AI).
    pub author: String,
}

impl Event {
    /// Create a new event.
    pub fn new(
        event_type: EventType,
        entity_id: impl Into<String>,
        payload: serde_json::Value,
        author: impl Into<String>,
    ) -> Self {
        Self {
            id: EventId::new(),
            event_type,
            entity_id: entity_id.into(),
            payload,
            timestamp: Utc::now(),
            author: author.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_event_records_timestamp() {
        let before = Utc::now();
        let event = Event::new(
            EventType::NodeCreated,
            "node-123",
            serde_json::json!({"title": "Test"}),
            "system",
        );
        let after = Utc::now();

        assert!(event.timestamp >= before);
        assert!(event.timestamp <= after);
        assert_eq!(event.event_type, EventType::NodeCreated);
        assert_eq!(event.entity_id, "node-123");
        assert_eq!(event.author, "system");
    }

    #[test]
    fn event_serde_roundtrip() {
        let event = Event::new(
            EventType::FeatureReleased,
            "feature-wallet",
            serde_json::json!({"version": "1.0"}),
            "admin",
        );

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&json).unwrap();

        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.entity_id, deserialized.entity_id);
        assert_eq!(event.author, deserialized.author);
    }
}
