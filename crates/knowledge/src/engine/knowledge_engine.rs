use sentinel_arc_core::domain::brain_card::HistoryEntry;
use sentinel_arc_core::domain::search::{SearchQuery, SearchResult};
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::{
    BrainCard, Event, EventId, Node, NodeId, Relationship, RelationshipId, RelationshipType, Rule,
    RuleId,
};

use crate::engine::event_engine::EventEngine;
use crate::engine::node_engine::NodeEngine;
use crate::engine::relationship_engine::RelationshipEngine;
use crate::engine::rule_engine::RuleEngine;
use crate::engine::search_engine::SearchEngine;
use crate::repository::KnowledgeRepository;
use crate::search_repository::SearchRepository;

/// The central facade for all Sentinel Arc Knowledge operations.
///
/// Wraps individual engines to provide a single, unified API for upstream consumers
/// (e.g., UI, MCP servers, and Context Engines).
#[derive(Debug)]
pub struct KnowledgeEngine {
    #[allow(dead_code)]
    repo: KnowledgeRepository,
    node_engine: NodeEngine,
    rel_engine: RelationshipEngine,
    event_engine: EventEngine,
    rule_engine: RuleEngine,
    search_engine: SearchEngine,
}

impl KnowledgeEngine {
    /// Initialize a new KnowledgeEngine with the given database handle.
    ///
    /// The search index is stored at `.brain/search_index/` alongside the database.
    pub fn new(db: &crate::database::Database) -> Self {
        let repo = KnowledgeRepository::new(db.pool().clone());

        // Place the Tantivy index alongside the database file.
        let index_path = db.db_path().parent().unwrap().join("search_index");
        let search_repo =
            SearchRepository::open(&index_path).expect("Failed to initialize search index");

        Self {
            node_engine: NodeEngine::new(repo.clone()),
            rel_engine: RelationshipEngine::new(repo.clone()),
            event_engine: EventEngine::new(repo.clone()),
            rule_engine: RuleEngine::new(repo.clone()),
            search_engine: SearchEngine::new(search_repo),
            repo,
        }
    }

    /// Initialize internally with a given repository and in-memory search index.
    #[allow(dead_code)]
    pub(crate) fn new_internal(repo: KnowledgeRepository) -> Self {
        let search_repo = SearchRepository::open_in_memory()
            .expect("Failed to initialize in-memory search index");

        Self {
            node_engine: NodeEngine::new(repo.clone()),
            rel_engine: RelationshipEngine::new(repo.clone()),
            event_engine: EventEngine::new(repo.clone()),
            rule_engine: RuleEngine::new(repo.clone()),
            search_engine: SearchEngine::new(search_repo),
            repo,
        }
    }

    // ── Node Operations ─────────────────────────────────────────────

    pub async fn create_node(&self, node: Node) -> BrainResult<(Node, Event)> {
        let (node, event) = self.node_engine.create_node(node).await?;
        // Best-effort indexing — log and continue on failure.
        if let Err(e) = self.search_engine.index_node(&node) {
            eprintln!(
                "Warning: search index update failed for node {}: {e}",
                node.id
            );
        }
        if let Err(e) = self.search_engine.index_event(&event) {
            eprintln!(
                "Warning: search index update failed for event {}: {e}",
                event.id
            );
        }
        Ok((node, event))
    }

    pub async fn update_node(&self, node: Node) -> BrainResult<(Node, Event)> {
        let (node, event) = self.node_engine.update_node(node).await?;
        if let Err(e) = self.search_engine.index_node(&node) {
            eprintln!(
                "Warning: search index update failed for node {}: {e}",
                node.id
            );
        }
        if let Err(e) = self.search_engine.index_event(&event) {
            eprintln!(
                "Warning: search index update failed for event {}: {e}",
                event.id
            );
        }
        Ok((node, event))
    }

    pub async fn archive_node(&self, id: &NodeId) -> BrainResult<(Node, Event)> {
        let (node, event) = self.node_engine.archive_node(id).await?;
        if let Err(e) = self.search_engine.index_node(&node) {
            eprintln!(
                "Warning: search index update failed for node {}: {e}",
                node.id
            );
        }
        if let Err(e) = self.search_engine.index_event(&event) {
            eprintln!(
                "Warning: search index update failed for event {}: {e}",
                event.id
            );
        }
        Ok((node, event))
    }

    pub async fn get_node(&self, id: &NodeId) -> BrainResult<Node> {
        self.node_engine.get_node(id).await
    }

    pub async fn list_nodes(&self) -> BrainResult<Vec<Node>> {
        self.node_engine.list_nodes().await
    }

    // ── Relationship Operations ─────────────────────────────────────

    pub async fn create_relationship(
        &self,
        rel: Relationship,
    ) -> BrainResult<(Relationship, Event)> {
        let (rel, event) = self.rel_engine.create_relationship(rel).await?;
        if let Err(e) = self.search_engine.index_relationship(&rel) {
            eprintln!(
                "Warning: search index update failed for relationship {}: {e}",
                rel.id
            );
        }
        if let Err(e) = self.search_engine.index_event(&event) {
            eprintln!(
                "Warning: search index update failed for event {}: {e}",
                event.id
            );
        }
        Ok((rel, event))
    }

    pub async fn delete_relationship(&self, id: &RelationshipId) -> BrainResult<Event> {
        let event = self.rel_engine.delete_relationship(id).await?;
        if let Err(e) = self.search_engine.delete_by_id(id.as_str()) {
            eprintln!("Warning: search index delete failed for relationship {id}: {e}");
        }
        if let Err(e) = self.search_engine.index_event(&event) {
            eprintln!(
                "Warning: search index update failed for event {}: {e}",
                event.id
            );
        }
        Ok(event)
    }

    pub async fn get_relationship(&self, id: &RelationshipId) -> BrainResult<Relationship> {
        self.rel_engine.get_relationship(id).await
    }

    /// Retrieve all relationships in the repository.
    pub async fn list_all_relationships(&self) -> BrainResult<Vec<Relationship>> {
        self.rel_engine.list_all_relationships().await
    }

    /// Find relationships involving the given node.
    pub async fn find_related(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.rel_engine.find_related(node_id).await
    }

    // ── Event Operations ────────────────────────────────────────────

    pub async fn get_event(&self, id: &EventId) -> BrainResult<Event> {
        self.event_engine.get_event(id).await
    }

    pub async fn get_history(&self, entity_id: &str) -> BrainResult<Vec<Event>> {
        self.event_engine.get_entity_history(entity_id).await
    }

    // ── Rule Operations ─────────────────────────────────────────────

    pub async fn get_rule(&self, id: &RuleId) -> BrainResult<Rule> {
        self.rule_engine.get_rule(id).await
    }

    pub async fn list_rules(&self) -> BrainResult<Vec<Rule>> {
        self.rule_engine.list_enabled_rules().await
    }

    // ── Search Operations ───────────────────────────────────────────

    /// Full-text search across all entities.
    pub fn search(&self, query: &str) -> BrainResult<SearchResult> {
        self.search_engine.search(query)
    }

    /// Full-text search with advanced filters and pagination.
    pub fn search_advanced(&self, query: &SearchQuery) -> BrainResult<SearchResult> {
        self.search_engine.search_advanced(query)
    }

    /// Rebuild the entire search index from current SQLite state.
    ///
    /// Reads all entities via existing engine methods and re-indexes them.
    /// Use this to recover from index corruption or after a bulk import.
    pub async fn rebuild_search_index(&self) -> BrainResult<()> {
        self.search_engine.clear_index()?;

        let nodes = self.list_nodes().await?;
        for node in &nodes {
            self.search_engine.index_node(node)?;
        }

        let rules = self.rule_engine.list_enabled_rules().await?;
        for rule in &rules {
            self.search_engine.index_rule(rule)?;
        }

        // Index relationships for all nodes.
        for node in &nodes {
            let rels = self.find_related(&node.id).await?;
            for rel in &rels {
                self.search_engine.index_relationship(rel)?;
            }
        }

        Ok(())
    }

    // ── Brain Card Assembly ─────────────────────────────────────────

    /// Dynamically assemble a Brain Card for a given node.
    pub async fn generate_brain_card(&self, id: &NodeId) -> BrainResult<BrainCard> {
        let node = self.get_node(id).await?;
        let mut card = BrainCard::empty(node.title, node.node_type);

        card.purpose = node.description;
        card.status = node.status.to_string();

        // Fetch relationships
        let rels = self.find_related(id).await?;
        for rel in rels {
            if rel.source_node == *id && rel.relationship_type == RelationshipType::DependsOn {
                card.dependencies.push(rel.target_node);
            } else {
                let other_id = if rel.source_node == *id {
                    rel.target_node
                } else {
                    rel.source_node
                };
                if !card.related_nodes.contains(&other_id) {
                    card.related_nodes.push(other_id);
                }
            }
        }

        // Fetch history
        let events = self.get_history(id.as_str()).await?;
        for event in events {
            card.history.push(HistoryEntry {
                timestamp: event.timestamp,
                description: event.event_type.to_string(),
                author: event.author,
            });
        }

        Ok(card)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_helpers::setup_knowledge_engine;
    use sentinel_arc_core::{Node, NodeType, SearchEntityKind, SearchQuery};

    #[tokio::test]
    async fn test_create_node() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (node, _) = ke.create_node(n).await.unwrap();
        assert_eq!(node.title, "T");
    }

    #[tokio::test]
    async fn test_update_node() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (mut node, _) = ke.create_node(n).await.unwrap();
        node.title = "U".to_string();
        let (updated, _) = ke.update_node(node).await.unwrap();
        assert_eq!(updated.title, "U");
    }

    #[tokio::test]
    async fn test_archive_node() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (node, _) = ke.create_node(n).await.unwrap();
        let (archived, _) = ke.archive_node(&node.id).await.unwrap();
        assert_eq!(archived.status, sentinel_arc_core::NodeStatus::Archived);
    }

    #[tokio::test]
    async fn test_get_node() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (node, _) = ke.create_node(n).await.unwrap();
        let fetched = ke.get_node(&node.id).await.unwrap();
        assert_eq!(fetched.id, node.id);
    }

    #[tokio::test]
    async fn test_list_nodes() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let _ = ke.create_node(n).await.unwrap();
        let nodes = ke.list_nodes().await.unwrap();
        assert!(!nodes.is_empty());
    }

    #[tokio::test]
    async fn test_get_history() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (node, _) = ke.create_node(n).await.unwrap();
        let hist = ke.get_history(node.id.as_str()).await.unwrap();
        assert!(!hist.is_empty());
    }

    #[tokio::test]
    async fn test_get_event() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "T");
        let (_, event) = ke.create_node(n).await.unwrap();
        let fetched = ke.get_event(&event.id).await.unwrap();
        assert_eq!(fetched.id, event.id);
    }

    #[tokio::test]
    async fn test_list_rules() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let rules = ke.list_rules().await.unwrap();
        assert!(rules.is_empty());
    }

    #[tokio::test]
    async fn test_create_relationship() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let (n1, _) = ke.create_node(n1).await.unwrap();
        let (n2, _) = ke.create_node(n2).await.unwrap();
        let rel = sentinel_arc_core::Relationship::new(
            n1.id,
            n2.id,
            sentinel_arc_core::RelationshipType::DependsOn,
        );
        let res = ke.create_relationship(rel).await;
        assert!(res.is_ok());
    }

    // ── Search Integration Tests ────────────────────────────────────

    #[tokio::test]
    async fn test_create_node_indexes_automatically() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "WalletModule");
        let (node, _) = ke.create_node(n).await.unwrap();

        let result = ke.search("WalletModule").unwrap();
        assert_eq!(result.total_count, 1);
        assert_eq!(result.hits[0].entity_id, node.id.as_str());
    }

    #[tokio::test]
    async fn test_update_node_updates_index() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "OldName");
        let (mut node, _) = ke.create_node(n).await.unwrap();

        node.title = "NewName".to_string();
        ke.update_node(node).await.unwrap();

        let old_result = ke.search("OldName").unwrap();
        assert_eq!(old_result.total_count, 0);

        let new_result = ke.search("NewName").unwrap();
        assert_eq!(new_result.total_count, 1);
    }

    #[tokio::test]
    async fn test_search_with_entity_kind_filter() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n = Node::new(NodeType::Feature, "SearchTest");
        ke.create_node(n).await.unwrap();

        let q = SearchQuery {
            query: "SearchTest".to_string(),
            entity_kinds: Some(vec![SearchEntityKind::Node]),
            node_types: None,
            statuses: None,
            tags: None,
            offset: 0,
            limit: 20,
            fuzzy: false,
        };
        let result = ke.search_advanced(&q).unwrap();
        assert_eq!(result.total_count, 1);
    }

    #[tokio::test]
    async fn test_rebuild_search_index() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let n1 = Node::new(NodeType::Feature, "RebuildA");
        let n2 = Node::new(NodeType::Bug, "RebuildB");
        ke.create_node(n1).await.unwrap();
        ke.create_node(n2).await.unwrap();

        // Nuke the index
        ke.search_engine.clear_index().unwrap();
        assert_eq!(ke.search("RebuildA").unwrap().total_count, 0);

        // Rebuild from SQLite
        ke.rebuild_search_index().await.unwrap();
        assert_eq!(ke.search("RebuildA").unwrap().total_count, 1);
        assert_eq!(ke.search("RebuildB").unwrap().total_count, 1);
    }
}
