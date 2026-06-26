//! # Project Brain Core
//!
//! Core contracts, types, traits, and error definitions for Project Brain.
//!
//! This crate contains **no business logic**. It defines the shared vocabulary
//! that all other crates in the Project Brain workspace depend on.
//!
//! ## Domain Model
//!
//! The four core entities are:
//! - [`Node`] — Any important entity (Project, Feature, File, etc.)
//! - [`Relationship`] — First-class connection between two nodes
//! - [`Event`] — Immutable record of every meaningful change
//! - [`Rule`] — Data-driven constraint (never hardcoded)

// ── Types ──
pub mod types {
    pub mod event_type;
    pub mod ids;
    pub mod node_type;
    pub mod relationship_type;
    pub mod source;
}

// ── Domain ──
pub mod domain {
    pub mod brain_card;
    pub mod context_package;
    pub mod event;
    pub mod node;
    pub mod relationship;
    pub mod rule;
}

// ── Root ──
pub mod error;
pub mod traits;

// Re-export commonly used items at the crate root for convenience.
pub use domain::brain_card::{BrainCard, HistoryEntry, RoadmapEntry};
pub use domain::context_package::ContextPackage;
pub use domain::event::Event;
pub use domain::node::{Node, NodeStatus};
pub use domain::relationship::Relationship;
pub use domain::rule::{Rule, RuleCategory, RuleSeverity};
pub use error::{BrainError, BrainResult};
pub use types::event_type::EventType;
pub use types::ids::{EventId, NodeId, RelationshipId, RuleId};
pub use types::node_type::NodeType;
pub use types::relationship_type::RelationshipType;
pub use types::source::{Confidence, Source};
