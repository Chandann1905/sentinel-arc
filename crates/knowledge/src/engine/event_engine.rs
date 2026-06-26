use crate::repository::KnowledgeRepository;
use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::traits::EventStore;
use sentinel_arc_core::{Event, EventId, EventType};

#[derive(Debug, Clone)]
pub(crate) struct EventEngine {
    repo: KnowledgeRepository,
}

#[allow(dead_code)]
impl EventEngine {
    pub(crate) fn new(repo: KnowledgeRepository) -> Self {
        Self { repo }
    }

    pub async fn get_event(&self, id: &EventId) -> BrainResult<Event> {
        self.repo.event_store().get(id).await
    }

    pub async fn get_entity_history(&self, entity_id: &str) -> BrainResult<Vec<Event>> {
        self.repo.event_store().find_by_entity(entity_id).await
    }

    pub async fn get_events_by_type(&self, event_type: EventType) -> BrainResult<Vec<Event>> {
        self.repo.event_store().find_by_type(event_type).await
    }

    pub async fn get_recent_events(&self, limit: usize) -> BrainResult<Vec<Event>> {
        self.repo.event_store().list_recent(limit as u32).await
    }
}

#[cfg(test)]
mod missing_event_tests {

    use crate::test_utils::test_helpers::setup_engines;
    use sentinel_arc_core::{EventType, Node, NodeType};

    #[tokio::test]
    async fn test_get_event() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "T");
        let (_, event) = ne.create_node(n).await.unwrap();

        let fetched = ee.get_event(&event.id).await.unwrap();
        assert_eq!(fetched.id, event.id);
    }

    #[tokio::test]
    async fn test_get_entity_history() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "T");
        let (node, _) = ne.create_node(n).await.unwrap();

        let hist = ee.get_entity_history(node.id.as_str()).await.unwrap();
        assert_eq!(hist.len(), 1);
    }

    #[tokio::test]
    async fn test_get_events_by_type() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "T");
        let _ = ne.create_node(n).await.unwrap();

        let events = ee.get_events_by_type(EventType::NodeCreated).await.unwrap();
        assert!(!events.is_empty());
    }

    #[tokio::test]
    async fn test_get_recent_events() {
        let (_tmp, ne, _, ee, _) = setup_engines().await;
        let n = Node::new(NodeType::Feature, "T");
        let _ = ne.create_node(n).await.unwrap();

        let events = ee.get_recent_events(10).await.unwrap();
        assert!(!events.is_empty());
    }
}
