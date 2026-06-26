//! Core traits defining the storage interfaces for Project Brain.
//!
//! These traits define the contracts that storage implementations must
//! fulfill. The `knowledge` crate provides the SQLite implementation.
//!
//! All traits are async to support both SQLite and potential future
//! storage backends.

use std::future::Future;

use crate::domain::event::Event;
use crate::domain::node::Node;
use crate::domain::relationship::Relationship;
use crate::domain::rule::{Rule, RuleCategory};
use crate::error::BrainResult;
use crate::types::event_type::EventType;
use crate::types::ids::{EventId, NodeId, RelationshipId, RuleId};
use crate::types::node_type::NodeType;
use crate::types::relationship_type::RelationshipType;

/// Storage interface for Node entities.
pub trait NodeStore: Send + Sync {
    /// Insert a new node. Returns error if ID already exists.
    fn create(&self, node: &Node) -> impl Future<Output = BrainResult<()>> + Send;

    /// Retrieve a node by its ID. Returns NotFound if absent.
    fn get(&self, id: &NodeId) -> impl Future<Output = BrainResult<Node>> + Send;

    /// Update an existing node. Returns NotFound if absent.
    fn update(&self, node: &Node) -> impl Future<Output = BrainResult<()>> + Send;

    /// Delete a node by its ID. Returns NotFound if absent.
    fn delete(&self, id: &NodeId) -> impl Future<Output = BrainResult<()>> + Send;

    /// List all nodes of a given type.
    fn list_by_type(
        &self,
        node_type: NodeType,
    ) -> impl Future<Output = BrainResult<Vec<Node>>> + Send;

    /// List all nodes (no filter).
    fn list_all(&self) -> impl Future<Output = BrainResult<Vec<Node>>> + Send;
}

/// Storage interface for Relationship entities.
pub trait RelationshipStore: Send + Sync {
    /// Insert a new relationship.
    fn create(&self, relationship: &Relationship) -> impl Future<Output = BrainResult<()>> + Send;

    /// Retrieve a relationship by ID.
    fn get(&self, id: &RelationshipId) -> impl Future<Output = BrainResult<Relationship>> + Send;

    /// Delete a relationship by ID.
    fn delete(&self, id: &RelationshipId) -> impl Future<Output = BrainResult<()>> + Send;

    /// Find all relationships where the given node is the source.
    fn find_by_source(
        &self,
        node_id: &NodeId,
    ) -> impl Future<Output = BrainResult<Vec<Relationship>>> + Send;

    /// Find all relationships where the given node is the target.
    fn find_by_target(
        &self,
        node_id: &NodeId,
    ) -> impl Future<Output = BrainResult<Vec<Relationship>>> + Send;

    /// Find all relationships of a given type.
    fn find_by_type(
        &self,
        rel_type: RelationshipType,
    ) -> impl Future<Output = BrainResult<Vec<Relationship>>> + Send;

    /// Find all relationships involving a given node (as source or target).
    fn find_by_node(
        &self,
        node_id: &NodeId,
    ) -> impl Future<Output = BrainResult<Vec<Relationship>>> + Send;

    /// List all relationships.
    fn list_all(&self) -> impl Future<Output = BrainResult<Vec<Relationship>>> + Send;
}

/// Storage interface for Event entities.
///
/// Events are **append-only**: they can be created and queried, but
/// never updated or deleted. This is enforced by the trait interface.
pub trait EventStore: Send + Sync {
    /// Append a new event. Events are immutable once created.
    fn create(&self, event: &Event) -> impl Future<Output = BrainResult<()>> + Send;

    /// Retrieve an event by ID.
    fn get(&self, id: &EventId) -> impl Future<Output = BrainResult<Event>> + Send;

    /// Find all events for a given entity ID, ordered by timestamp.
    fn find_by_entity(
        &self,
        entity_id: &str,
    ) -> impl Future<Output = BrainResult<Vec<Event>>> + Send;

    /// Find all events of a given type, ordered by timestamp.
    fn find_by_type(
        &self,
        event_type: EventType,
    ) -> impl Future<Output = BrainResult<Vec<Event>>> + Send;

    /// Find events within a time range (Unix timestamps), ordered by timestamp.
    fn find_by_time_range(
        &self,
        start: i64,
        end: i64,
    ) -> impl Future<Output = BrainResult<Vec<Event>>> + Send;

    /// List recent events, limited to `count`, ordered by timestamp descending.
    fn list_recent(&self, count: u32) -> impl Future<Output = BrainResult<Vec<Event>>> + Send;
}

/// Storage interface for Rule entities.
pub trait RuleStore: Send + Sync {
    /// Insert a new rule.
    fn create(&self, rule: &Rule) -> impl Future<Output = BrainResult<()>> + Send;

    /// Retrieve a rule by ID.
    fn get(&self, id: &RuleId) -> impl Future<Output = BrainResult<Rule>> + Send;

    /// Update an existing rule.
    fn update(&self, rule: &Rule) -> impl Future<Output = BrainResult<()>> + Send;

    /// Delete a rule by ID.
    fn delete(&self, id: &RuleId) -> impl Future<Output = BrainResult<()>> + Send;

    /// List all rules in a given category.
    fn list_by_category(
        &self,
        category: RuleCategory,
    ) -> impl Future<Output = BrainResult<Vec<Rule>>> + Send;

    /// List all enabled rules.
    fn list_enabled(&self) -> impl Future<Output = BrainResult<Vec<Rule>>> + Send;

    /// List all rules.
    fn list_all(&self) -> impl Future<Output = BrainResult<Vec<Rule>>> + Send;
}

#[cfg(test)]
mod tests {
    // Trait definition compilation test — if this module compiles,
    // the traits are well-formed.

    #[test]
    fn traits_are_object_safe_for_send_sync() {
        // Verify the traits require Send + Sync.
        // These would fail at compile time if traits weren't Send + Sync.
        fn _assert_node_store<T: super::NodeStore>() {}
        fn _assert_relationship_store<T: super::RelationshipStore>() {}
        fn _assert_event_store<T: super::EventStore>() {}
        fn _assert_rule_store<T: super::RuleStore>() {}
    }
}
