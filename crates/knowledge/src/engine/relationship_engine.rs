use crate::repository::KnowledgeRepository;
use project_brain_core::error::{BrainError, BrainResult};
use project_brain_core::traits::{NodeStore, RelationshipStore};
use project_brain_core::{
    Event, EventType, NodeId, Relationship, RelationshipId, RelationshipType,
};

#[derive(Debug, Clone)]
pub(crate) struct RelationshipEngine {
    repo: KnowledgeRepository,
}

#[allow(dead_code)]
impl RelationshipEngine {
    pub(crate) fn new(repo: KnowledgeRepository) -> Self {
        Self { repo }
    }

    pub async fn create_relationship(
        &self,
        rel: Relationship,
    ) -> BrainResult<(Relationship, Event)> {
        if rel.source_node == rel.target_node {
            return Err(BrainError::validation(
                "Self-referential relationships are not allowed",
            ));
        }

        self.repo.node_store().get(&rel.source_node).await?;
        self.repo.node_store().get(&rel.target_node).await?;

        let existing = self
            .repo
            .relationship_store()
            .find_by_source(&rel.source_node)
            .await?;
        if existing.iter().any(|r| {
            r.target_node == rel.target_node && r.relationship_type == rel.relationship_type
        }) {
            return Err(BrainError::validation("Duplicate relationship"));
        }

        let payload = serde_json::json!({ "after": rel });
        let event = Event::new(
            EventType::RelationshipAdded,
            rel.id.as_str(),
            payload,
            "system",
        );

        self.repo
            .create_relationship_with_event(&rel, &event)
            .await?;
        Ok((rel, event))
    }

    pub async fn delete_relationship(&self, id: &RelationshipId) -> BrainResult<Event> {
        let rel = self.repo.relationship_store().get(id).await?;

        let payload = serde_json::json!({ "before": rel });
        let event = Event::new(
            EventType::RelationshipRemoved,
            rel.id.as_str(),
            payload,
            "system",
        );

        self.repo.delete_relationship_with_event(id, &event).await?;
        Ok(event)
    }

    pub async fn get_relationship(&self, id: &RelationshipId) -> BrainResult<Relationship> {
        self.repo.relationship_store().get(id).await
    }

    pub async fn find_related(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_node(node_id).await
    }

    pub async fn find_by_source(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_source(node_id).await
    }

    pub async fn find_by_target(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_target(node_id).await
    }

    pub async fn find_by_type(&self, rel_type: RelationshipType) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_type(rel_type).await
    }

    pub async fn find_incoming(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_target(node_id).await
    }

    pub async fn find_outgoing(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        self.repo.relationship_store().find_by_source(node_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_helpers::setup_engines;
    use project_brain_core::Node;
    use project_brain_core::NodeType;

    #[tokio::test]
    async fn create_valid_relationship() {
        let (_tmp, ne, re, ee, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let (n2, _) = ne.create_node(n2).await.unwrap();

        let rel = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let (created_rel, event) = re.create_relationship(rel).await.unwrap();

        assert_eq!(created_rel.source_node, n1.id);
        assert_eq!(created_rel.target_node, n2.id);
        assert_eq!(
            event.event_type,
            project_brain_core::EventType::RelationshipAdded
        );

        let history = ee
            .get_entity_history(created_rel.id.as_str())
            .await
            .unwrap();
        assert_eq!(history.len(), 1);
    }
}

#[cfg(test)]
mod missing_tests {

    use crate::test_utils::test_helpers::setup_engines;
    use project_brain_core::Node;
    use project_brain_core::NodeId;
    use project_brain_core::NodeType;
    use project_brain_core::Relationship;
    use project_brain_core::RelationshipType;

    #[tokio::test]
    async fn create_missing_source_node() {
        let (_tmp, _, re, _, _) = setup_engines().await;
        let rel = Relationship::new(
            NodeId::from_string("src"),
            NodeId::from_string("tgt"),
            RelationshipType::DependsOn,
        );
        let res = re.create_relationship(rel).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_missing_target_node() {
        let (_tmp, ne, re, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let rel = Relationship::new(
            n1.id,
            NodeId::from_string("tgt"),
            RelationshipType::DependsOn,
        );
        let res = re.create_relationship(rel).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_self_referential_relationship() {
        let (_tmp, ne, re, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let rel = Relationship::new(n1.id.clone(), n1.id.clone(), RelationshipType::DependsOn);
        let res = re.create_relationship(rel).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_duplicate_relationship() {
        let (_tmp, ne, re, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let (n2, _) = ne.create_node(n2).await.unwrap();

        let rel1 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let _ = re.create_relationship(rel1).await.unwrap();

        let rel2 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let res = re.create_relationship(rel2).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_multiple_relationships_different_types() {
        let (_tmp, ne, re, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let (n2, _) = ne.create_node(n2).await.unwrap();

        let rel1 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let _ = re.create_relationship(rel1).await.unwrap();

        let rel2 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::RelatesTo);
        let res = re.create_relationship(rel2).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn delete_relationship() {
        let (_tmp, ne, re, _ee, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let (n2, _) = ne.create_node(n2).await.unwrap();

        let rel1 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let (created, _) = re.create_relationship(rel1).await.unwrap();

        let event = re.delete_relationship(&created.id).await.unwrap();
        assert_eq!(
            event.event_type,
            project_brain_core::EventType::RelationshipRemoved
        );

        let fetched = re.get_relationship(&created.id).await;
        assert!(fetched.is_err());
    }

    #[tokio::test]
    async fn query_methods() {
        let (_tmp, ne, re, _, _) = setup_engines().await;
        let n1 = Node::new(NodeType::Feature, "N1");
        let n2 = Node::new(NodeType::Feature, "N2");
        let n3 = Node::new(NodeType::Feature, "N3");
        let (n1, _) = ne.create_node(n1).await.unwrap();
        let (n2, _) = ne.create_node(n2).await.unwrap();
        let (n3, _) = ne.create_node(n3).await.unwrap();

        let rel1 = Relationship::new(n1.id.clone(), n2.id.clone(), RelationshipType::DependsOn);
        let _ = re.create_relationship(rel1).await.unwrap();

        let rel2 = Relationship::new(n2.id.clone(), n3.id.clone(), RelationshipType::RelatesTo);
        let _ = re.create_relationship(rel2).await.unwrap();

        let out_n1 = re.find_outgoing(&n1.id).await.unwrap();
        assert_eq!(out_n1.len(), 1);

        let in_n2 = re.find_incoming(&n2.id).await.unwrap();
        assert_eq!(in_n2.len(), 1);

        let by_type = re.find_by_type(RelationshipType::RelatesTo).await.unwrap();
        assert_eq!(by_type.len(), 1);
    }
}
