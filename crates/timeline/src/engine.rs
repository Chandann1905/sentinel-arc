use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::{Event, EventType, NodeId};
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::sync::Arc;

use crate::types::{Timeline, TimelineEvent, TimelineType};

/// The TimelineEngine projects raw events into human-readable chronological timelines.
/// It is strictly read-only and consumes the KnowledgeEngine.
#[derive(Debug, Clone)]
pub struct TimelineEngine {
    knowledge_engine: Arc<KnowledgeEngine>,
}

impl TimelineEngine {
    pub fn new(knowledge_engine: Arc<KnowledgeEngine>) -> Self {
        Self { knowledge_engine }
    }

    /// Generates a chronological timeline for a specific Node (e.g. Feature, Module).
    pub async fn generate_node_timeline(&self, node_id: &NodeId) -> BrainResult<Timeline> {
        let events = self.knowledge_engine.get_history(node_id.as_str()).await?;

        let mut timeline_events = Vec::with_capacity(events.len());
        for event in events {
            timeline_events.push(self.project_event(event));
        }

        // Ensure chronological order (oldest first)
        timeline_events.sort_by_key(|e| e.timestamp);

        Ok(Timeline::new(
            Some(node_id.to_string()),
            TimelineType::Feature,
            timeline_events,
        ))
    }

    /// Generates a timeline of all Architecture Decision Records (ADRs).
    pub async fn generate_decision_history(&self) -> BrainResult<Timeline> {
        let mut accepted = self
            .knowledge_engine
            .get_events_by_type(EventType::DecisionAccepted)
            .await?;
        let mut rejected = self
            .knowledge_engine
            .get_events_by_type(EventType::DecisionRejected)
            .await?;

        let mut all_decisions = Vec::new();
        all_decisions.append(&mut accepted);
        all_decisions.append(&mut rejected);

        let mut timeline_events = Vec::with_capacity(all_decisions.len());
        for event in all_decisions {
            timeline_events.push(self.project_event(event));
        }

        timeline_events.sort_by_key(|e| e.timestamp);

        Ok(Timeline::new(None, TimelineType::Decision, timeline_events))
    }

    /// Generates a timeline for a specific Roadmap item.
    pub async fn generate_roadmap_timeline(&self, roadmap_id: &NodeId) -> BrainResult<Timeline> {
        let events = self
            .knowledge_engine
            .get_history(roadmap_id.as_str())
            .await?;

        let mut timeline_events = Vec::with_capacity(events.len());
        for event in events {
            timeline_events.push(self.project_event(event));
        }

        timeline_events.sort_by_key(|e| e.timestamp);

        Ok(Timeline::new(
            Some(roadmap_id.to_string()),
            TimelineType::Roadmap,
            timeline_events,
        ))
    }

    /// Generates a global project-wide timeline of recent events.
    pub async fn generate_project_timeline(&self, limit: usize) -> BrainResult<Timeline> {
        let events = self.knowledge_engine.get_recent_events(limit).await?;

        let mut timeline_events = Vec::with_capacity(events.len());
        for event in events {
            timeline_events.push(self.project_event(event));
        }

        // Reverse recent events to be chronological (oldest first in the returned limit window)
        timeline_events.sort_by_key(|e| e.timestamp);

        Ok(Timeline::new(None, TimelineType::Project, timeline_events))
    }

    /// Core projection logic: turns a raw Event into a human-readable TimelineEvent.
    fn project_event(&self, event: Event) -> TimelineEvent {
        let description = match event.event_type {
            EventType::NodeCreated => {
                let title = event
                    .payload
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Node");
                format!("Node '{}' was created.", title)
            }
            EventType::NodeUpdated => {
                format!("Node '{}' was updated.", event.entity_id)
            }
            EventType::NodeArchived => {
                format!("Node '{}' was archived.", event.entity_id)
            }
            EventType::RelationshipAdded => {
                let target = event
                    .payload
                    .get("target_node")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown");
                let rel_type = event
                    .payload
                    .get("relationship_type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("RELATED_TO");
                format!(
                    "Node '{}' now {} node '{}'.",
                    event.entity_id, rel_type, target
                )
            }
            EventType::RelationshipRemoved => {
                format!(
                    "A relationship was removed from node '{}'.",
                    event.entity_id
                )
            }
            EventType::RuleAdded => {
                let rule_name = event
                    .payload
                    .get("name")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Rule");
                format!("Rule '{}' was added.", rule_name)
            }
            EventType::RuleUpdated => {
                format!("Rule '{}' was updated.", event.entity_id)
            }
            EventType::DecisionAccepted => {
                let title = event
                    .payload
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Decision");
                format!("Decision '{}' was accepted.", title)
            }
            EventType::DecisionRejected => {
                let title = event
                    .payload
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Decision");
                format!("Decision '{}' was rejected.", title)
            }
            EventType::BugOpened => {
                let title = event
                    .payload
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Bug");
                format!("Bug '{}' was opened.", title)
            }
            EventType::BugClosed => {
                format!("Bug '{}' was closed.", event.entity_id)
            }
            EventType::RoadmapAdded => {
                let title = event
                    .payload
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Unknown Item");
                format!("Roadmap item '{}' was added.", title)
            }
            EventType::RoadmapCompleted => {
                format!("Roadmap item '{}' was completed.", event.entity_id)
            }
            EventType::FeatureReleased => {
                format!("Feature '{}' was released.", event.entity_id)
            }
        };

        TimelineEvent {
            event_id: event.id,
            timestamp: event.timestamp,
            author: event.author,
            event_type: event.event_type,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinel_arc_core::{Event, EventType};
    use sentinel_arc_knowledge::database::Database;
    use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;

    async fn setup_knowledge_engine() -> (tempfile::TempDir, Arc<KnowledgeEngine>) {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::init_with_path(&db_path).await.unwrap();
        let ke = KnowledgeEngine::new(&db);
        (dir, Arc::new(ke))
    }

    #[tokio::test]
    async fn test_project_event_descriptions() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let te = TimelineEngine::new(ke);

        let evt = Event::new(
            EventType::NodeCreated,
            "node-1",
            serde_json::json!({"title": "Test Feature"}),
            "system",
        );
        let proj = te.project_event(evt);
        assert_eq!(proj.description, "Node 'Test Feature' was created.");

        let evt = Event::new(
            EventType::DecisionAccepted,
            "dec-1",
            serde_json::json!({"title": "Use Rust"}),
            "system",
        );
        let proj = te.project_event(evt);
        assert_eq!(proj.description, "Decision 'Use Rust' was accepted.");

        let evt = Event::new(
            EventType::RelationshipAdded,
            "node-1",
            serde_json::json!({"target_node": "node-2", "relationship_type": "DEPENDS_ON"}),
            "system",
        );
        let proj = te.project_event(evt);
        assert_eq!(
            proj.description,
            "Node 'node-1' now DEPENDS_ON node 'node-2'."
        );
    }

    #[tokio::test]
    async fn test_generate_project_timeline() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let te = TimelineEngine::new(ke.clone());

        // Create some events by invoking knowledge engine methods
        let n1 = sentinel_arc_core::Node::new(sentinel_arc_core::NodeType::Feature, "Feature 1");
        let (_n1, _evt1) = ke.create_node(n1).await.unwrap();

        let n2 = sentinel_arc_core::Node::new(sentinel_arc_core::NodeType::Feature, "Feature 2");
        let (_n2, _evt2) = ke.create_node(n2).await.unwrap();

        let timeline = te.generate_project_timeline(10).await.unwrap();

        assert_eq!(timeline.timeline_type, TimelineType::Project);
        assert!(timeline.events.len() >= 2);

        // Ensure chronological order
        for i in 0..(timeline.events.len() - 1) {
            assert!(timeline.events[i].timestamp <= timeline.events[i + 1].timestamp);
        }
    }
}
