//! Search engine — orchestrates query parsing and delegates to SearchRepository.
//!
//! The SearchEngine never accesses SQLite directly. It receives domain objects
//! from KnowledgeEngine and delegates all Tantivy operations to SearchRepository.

use sentinel_arc_core::domain::search::{SearchEntityKind, SearchQuery, SearchResult};
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::{Event, Node, Relationship, Rule};

use crate::search_repository::SearchRepository;

/// Coordinates search queries and index mutations.
///
/// This engine is `pub(crate)` — external consumers access search
/// exclusively through `KnowledgeEngine`.
#[derive(Debug)]
pub(crate) struct SearchEngine {
    repo: SearchRepository,
}

impl SearchEngine {
    /// Create a new SearchEngine backed by the given SearchRepository.
    pub fn new(repo: SearchRepository) -> Self {
        Self { repo }
    }

    // ── Index Mutation Operations ────────────────────────────────────

    /// Index or update a Node in the search index.
    pub fn index_node(&self, node: &Node) -> BrainResult<()> {
        self.repo.index_node(node)
    }

    /// Index or update a Relationship in the search index.
    pub fn index_relationship(&self, rel: &Relationship) -> BrainResult<()> {
        self.repo.index_relationship(rel)
    }

    /// Index or update a Rule in the search index.
    pub fn index_rule(&self, rule: &Rule) -> BrainResult<()> {
        self.repo.index_rule(rule)
    }

    /// Index an Event in the search index.
    pub fn index_event(&self, event: &Event) -> BrainResult<()> {
        self.repo.index_event(event)
    }

    /// Delete a document from the search index by entity ID.
    pub fn delete_by_id(&self, entity_id: &str) -> BrainResult<()> {
        self.repo.delete_by_id(entity_id)
    }

    /// Delete all documents from the search index.
    pub fn clear_index(&self) -> BrainResult<()> {
        self.repo.clear()
    }

    // ── Query Operations ────────────────────────────────────────────

    /// Execute a simple text search.
    pub fn search(&self, query: &str) -> BrainResult<SearchResult> {
        let sq = SearchQuery::new(query);
        self.search_advanced(&sq)
    }

    /// Execute an advanced search with filters and pagination.
    pub fn search_advanced(&self, query: &SearchQuery) -> BrainResult<SearchResult> {
        let limit = query.limit.clamp(1, 100);
        let offset = query.offset;

        let entity_kind_filter: Option<Vec<SearchEntityKind>> = query.entity_kinds.clone();
        let node_type_filter: Option<Vec<String>> = query
            .node_types
            .as_ref()
            .map(|types| types.iter().map(|t| t.to_string().to_lowercase()).collect());
        let status_filter: Option<Vec<String>> = query.statuses.as_ref().map(|statuses| {
            statuses
                .iter()
                .map(|s| s.to_string().to_lowercase())
                .collect()
        });

        let (hits, total_count) = self.repo.search(
            &query.query,
            entity_kind_filter.as_deref(),
            node_type_filter.as_deref(),
            status_filter.as_deref(),
            query.fuzzy,
            offset,
            limit,
        )?;

        Ok(SearchResult {
            hits,
            total_count,
            offset,
            limit,
        })
    }
}
