use project_brain_core::domain::brain_card::HistoryEntry;
use project_brain_core::error::BrainResult;
use project_brain_core::{
    BrainCard, Event, EventId, Node, NodeId, Relationship, RelationshipId, RelationshipType, Rule,
    RuleId,
};

use crate::engine::event_engine::EventEngine;
use crate::engine::node_engine::NodeEngine;
use crate::engine::relationship_engine::RelationshipEngine;
use crate::engine::rule_engine::RuleEngine;
use crate::repository::KnowledgeRepository;

/// The central facade for all Project Brain Knowledge operations.
///
/// Wraps individual engines to provide a single, unified API for upstream consumers
/// (e.g., UI, MCP servers, and Context Engines).
#[derive(Debug, Clone)]
pub struct KnowledgeEngine {
    #[allow(dead_code)]
    repo: KnowledgeRepository,
    node_engine: NodeEngine,
    rel_engine: RelationshipEngine,
    event_engine: EventEngine,
    rule_engine: RuleEngine,
}

impl KnowledgeEngine {
    /// Initialize a new KnowledgeEngine with the given database handle.
    pub fn new(db: &crate::database::Database) -> Self {
        let repo = KnowledgeRepository::new(db.pool().clone());
        Self {
            node_engine: NodeEngine::new(repo.clone()),
            rel_engine: RelationshipEngine::new(repo.clone()),
            event_engine: EventEngine::new(repo.clone()),
            rule_engine: RuleEngine::new(repo.clone()),
            repo,
        }
    }

    /// Initialize internally with a given repository.
    pub(crate) fn new_internal(repo: KnowledgeRepository) -> Self {
        Self {
            node_engine: NodeEngine::new(repo.clone()),
            rel_engine: RelationshipEngine::new(repo.clone()),
            event_engine: EventEngine::new(repo.clone()),
            rule_engine: RuleEngine::new(repo.clone()),
            repo,
        }
    }

    // ── Node Operations ─────────────────────────────────────────────

    pub async fn create_node(&self, node: Node) -> BrainResult<(Node, Event)> {
        self.node_engine.create_node(node).await
    }

    pub async fn update_node(&self, node: Node) -> BrainResult<(Node, Event)> {
        self.node_engine.update_node(node).await
    }

    pub async fn archive_node(&self, id: &NodeId) -> BrainResult<(Node, Event)> {
        self.node_engine.archive_node(id).await
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
        self.rel_engine.create_relationship(rel).await
    }

    pub async fn delete_relationship(&self, id: &RelationshipId) -> BrainResult<Event> {
        self.rel_engine.delete_relationship(id).await
    }

    pub async fn get_relationship(&self, id: &RelationshipId) -> BrainResult<Relationship> {
        self.rel_engine.get_relationship(id).await
    }

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

    // ── Brain Card Assembly ─────────────────────────────────────────

    /// Dynamically assemble a Brain Card for a given node.
    pub async fn generate_brain_card(&self, id: &NodeId) -> BrainResult<BrainCard> {
        let node = self.get_node(id).await?;
        let mut card = BrainCard::empty(node.title, node.node_type);

        card.purpose = node.description;
        card.status = node.status.to_string();

        // Populate basic lists from tags if they map directly (optional logic)
        // For MVP, we adhere strictly to "only existing data, do not invent fields"
        // Since database_tables and files are not explicit fields on Node, we leave them as defaults
        // (Vec::new()) unless they are stored in `tags`, but the prompt implies we don't invent anything.

        // Fetch relationships
        let rels = self.find_related(id).await?;
        for rel in rels {
            // If this node is the SOURCE and it depends on the TARGET -> Dependency
            if rel.source_node == *id && rel.relationship_type == RelationshipType::DependsOn {
                card.dependencies.push(rel.target_node);
            } else {
                // Otherwise it is related
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
                description: event.event_type.to_string(), // MVP fallback description
                author: event.author,
            });
        }

        Ok(card)
    }
}

#[cfg(test)]
mod missing_knowledge_tests {
    use crate::test_utils::test_helpers::setup_knowledge_engine;
    use project_brain_core::{Node, NodeType};

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
        assert_eq!(archived.status, project_brain_core::NodeStatus::Archived);
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
        let rel = project_brain_core::Relationship::new(
            n1.id,
            n2.id,
            project_brain_core::RelationshipType::DependsOn,
        );
        let res = ke.create_relationship(rel).await;
        assert!(res.is_ok());
    }
}
