//! # Sentinel Arc Knowledge Layer
//!
//! This crate implements the Knowledge Layer — the source of truth for
//! all project knowledge. It provides SQLite-backed storage for the
//! four core entities: Nodes, Relationships, Events, and Rules,
//! plus Tantivy-backed full-text search.
//!
//! ## Architecture
//!
//! ### Storage Layer (Milestone 1)
//! - `Database` — connection pool, initialization, migrations
//! - `SqliteNodeStore` — implements `NodeStore` trait
//! - `SqliteRelationshipStore` — implements `RelationshipStore` trait
//! - `SqliteEventStore` — implements `EventStore` trait (append-only)
//! - `SqliteRuleStore` — implements `RuleStore` trait
//!
//! ### Engine Layer (Milestone 2–5)
//! - `EventEngine` — orchestrates event emission for entity mutations
//! - `NodeEngine` — high-level node lifecycle management with auto-versioning
//! - `SearchEngine` — full-text search via Tantivy (Milestone 5)

// ── Storage modules ──
pub mod database;

// ── Store modules ──
pub(crate) mod store {
    pub mod event_store;
    pub mod node_store;
    pub mod relationship_store;
    pub mod rule_store;
}

// ── Engine modules ──
pub mod engine {
    pub(crate) mod event_engine;
    pub mod knowledge_engine;
    pub(crate) mod node_engine;
    pub(crate) mod relationship_engine;
    pub(crate) mod rule_engine;
    pub(crate) mod search_engine;
}

pub(crate) mod repository;
pub(crate) mod search_repository;

#[cfg(test)]
pub mod test_utils;

// ── Storage re-exports ──
pub use database::Database;

// ── Engine re-exports ──
pub use engine::knowledge_engine::KnowledgeEngine;
