use crate::repository::KnowledgeRepository;
use chrono::Utc;
use sentinel_arc_core::error::{BrainError, BrainResult};
use sentinel_arc_core::traits::NodeStore;
use sentinel_arc_core::{Event, EventType, Node, NodeId, NodeStatus, NodeType};

#[derive(Debug, Clone)]
pub(crate) struct NodeEngine {
    repo: KnowledgeRepository,
}

#[allow(dead_code)]
impl NodeEngine {
    pub(crate) fn new(repo: KnowledgeRepository) -> Self {
        Self { repo }
    }

    pub async fn create_node(&self, mut node: Node) -> BrainResult<(Node, Event)> {
        self.validate_title(&node.title)?;
        node.version = 1;
        node.status = NodeStatus::Active;
        let now = Utc::now();
        node.created_at = now;
        node.updated_at = now;

        let payload = serde_json::json!({ "after": node });
        let event = Event::new(EventType::NodeCreated, node.id.as_str(), payload, "system");

        self.repo.create_node_with_event(&node, &event).await?;
        Ok((node, event))
    }

    pub async fn update_node(&self, mut node: Node) -> BrainResult<(Node, Event)> {
        let before_state = self.repo.node_store().get(&node.id).await?;
        self.validate_title(&node.title)?;

        node.version = before_state.version + 1;
        node.updated_at = Utc::now();

        let payload = serde_json::json!({
            "before": before_state,
            "after": node,
        });

        let event = Event::new(EventType::NodeUpdated, node.id.as_str(), payload, "system");

        self.repo.update_node_with_event(&node, &event).await?;
        Ok((node, event))
    }

    pub async fn archive_node(&self, id: &NodeId) -> BrainResult<(Node, Event)> {
        let mut node = self.repo.node_store().get(id).await?;

        if node.status == NodeStatus::Archived {
            return Err(BrainError::validation(format!(
                "Node {} is already archived",
                id.as_str()
            )));
        }

        let before_state = node.clone();
        node.status = NodeStatus::Archived;
        node.version += 1;
        node.updated_at = Utc::now();

        let payload = serde_json::json!({
            "before": before_state,
            "after": node,
        });

        let event = Event::new(EventType::NodeArchived, node.id.as_str(), payload, "system");
        self.repo.update_node_with_event(&node, &event).await?;

        Ok((node, event))
    }

    pub async fn get_node(&self, id: &NodeId) -> BrainResult<Node> {
        self.repo.node_store().get(id).await
    }

    pub async fn list_nodes(&self) -> BrainResult<Vec<Node>> {
        self.repo.node_store().list_all().await
    }

    pub async fn list_by_type(&self, node_type: NodeType) -> BrainResult<Vec<Node>> {
        self.repo.node_store().list_by_type(node_type).await
    }

    fn validate_title(&self, title: &str) -> BrainResult<()> {
        if title.trim().is_empty() {
            return Err(BrainError::validation("Node title cannot be empty"));
        }
        if title.len() > 255 {
            return Err(BrainError::validation("Node title exceeds 255 characters"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_helpers::setup_engines;
    use sentinel_arc_core::Node;
    use sentinel_arc_core::NodeType;

    #[tokio::test]
    async fn create_valid_node() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test Feature");
        let (node, event) = ne.create_node(n).await.unwrap();

        assert_eq!(node.version, 1);
        assert_eq!(event.event_type, sentinel_arc_core::EventType::NodeCreated);

        let history = ee.get_entity_history(node.id.as_str()).await.unwrap();
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn create_node_empty_title() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "   ");
        let res = ne.create_node(n).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_node_long_title() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "a".repeat(256).as_str());
        let res = ne.create_node(n).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn update_node_increments_version() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test");
        let (mut node, _) = ne.create_node(n).await.unwrap();

        node.title = "Updated".to_string();
        let (updated, event) = ne.update_node(node).await.unwrap();

        assert_eq!(updated.version, 2);
        assert_eq!(updated.title, "Updated");
        assert_eq!(event.event_type, sentinel_arc_core::EventType::NodeUpdated);
    }

    #[tokio::test]
    async fn update_node_not_found() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test");
        let res = ne.update_node(n).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn archive_node_success() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test");
        let (node, _) = ne.create_node(n).await.unwrap();

        let (archived, event) = ne.archive_node(&node.id).await.unwrap();
        assert_eq!(archived.status, NodeStatus::Archived);
        assert_eq!(event.event_type, sentinel_arc_core::EventType::NodeArchived);

        let history = ee.get_entity_history(node.id.as_str()).await.unwrap();
        assert_eq!(history.len(), 2); // Created + Archived
    }

    #[tokio::test]
    async fn list_nodes_success() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test");
        let _ = ne.create_node(n).await.unwrap();

        let nodes = ne.list_nodes().await.unwrap();
        assert_eq!(nodes.len(), 1);
    }

    #[tokio::test]
    async fn get_node_success() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "Test");
        let (node, _) = ne.create_node(n).await.unwrap();

        let fetched = ne.get_node(&node.id).await.unwrap();
        assert_eq!(fetched.id, node.id);
    }

    #[tokio::test]
    async fn list_by_type_success() {
        let (_tmp, ne, _, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "Test");
        let n2 = Node::new(NodeType::Bug, "Test2");
        let _ = ne.create_node(n1).await.unwrap();
        let _ = ne.create_node(n2).await.unwrap();

        let nodes = ne.list_by_type(NodeType::Feature).await.unwrap();
        assert_eq!(nodes.len(), 1);
    }
}
