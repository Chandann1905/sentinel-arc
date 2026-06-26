//! Repository abstraction layer providing atomic Unit of Work across stores.

use sqlx::SqlitePool;

use sentinel_arc_core::error::{BrainError, BrainResult};
use sentinel_arc_core::{Event, Node, NodeId, Relationship, RelationshipId};

use crate::store::event_store::SqliteEventStore;
use crate::store::node_store::SqliteNodeStore;
use crate::store::relationship_store::SqliteRelationshipStore;
use crate::store::rule_store::SqliteRuleStore;

/// Provides transactional unit of work management for cross-store operations.
///
/// Ensures that mutations to entities and their corresponding events are committed atomically.
#[derive(Debug, Clone)]
pub struct KnowledgeRepository {
    pool: SqlitePool,
    node_store: SqliteNodeStore,
    event_store: SqliteEventStore,
    relationship_store: SqliteRelationshipStore,
    rule_store: SqliteRuleStore,
}

impl KnowledgeRepository {
    /// Create a new KnowledgeRepository wrapping the provided connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            node_store: SqliteNodeStore::new(pool.clone()),
            event_store: SqliteEventStore::new(pool.clone()),
            relationship_store: SqliteRelationshipStore::new(pool.clone()),
            rule_store: SqliteRuleStore::new(pool.clone()),
            pool,
        }
    }

    /// Access the underlying node store.
    pub fn node_store(&self) -> &SqliteNodeStore {
        &self.node_store
    }

    /// Access the underlying event store.
    pub fn event_store(&self) -> &SqliteEventStore {
        &self.event_store
    }

    /// Access the underlying relationship store.
    pub fn relationship_store(&self) -> &SqliteRelationshipStore {
        &self.relationship_store
    }

    /// Access the underlying rule store.
    pub fn rule_store(&self) -> &SqliteRuleStore {
        &self.rule_store
    }

    /// Atomically create a node and its corresponding event.
    pub async fn create_node_with_event(&self, node: &Node, event: &Event) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.node_store.create_tx(&mut tx, node).await?;
        self.event_store.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    /// Atomically update a node and append its corresponding event.
    pub async fn update_node_with_event(&self, node: &Node, event: &Event) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.node_store.update_tx(&mut tx, node).await?;
        self.event_store.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    /// Atomically delete a node and append its corresponding event.
    #[allow(dead_code)]
    pub async fn delete_node_with_event(&self, id: &NodeId, event: &Event) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.node_store.delete_tx(&mut tx, id).await?;
        self.event_store.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    /// Atomically create a relationship and append its corresponding event.
    pub async fn create_relationship_with_event(
        &self,
        rel: &Relationship,
        event: &Event,
    ) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.relationship_store.create_tx(&mut tx, rel).await?;
        self.event_store.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    /// Atomically delete a relationship and append its corresponding event.
    pub async fn delete_relationship_with_event(
        &self,
        id: &RelationshipId,
        event: &Event,
    ) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.relationship_store.delete_tx(&mut tx, id).await?;
        self.event_store.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }
}
