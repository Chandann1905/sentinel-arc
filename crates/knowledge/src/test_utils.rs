#[cfg(test)]
pub mod test_helpers {
    use crate::database::Database;
    use crate::engine::event_engine::EventEngine;
    use crate::engine::node_engine::NodeEngine;
    use crate::engine::relationship_engine::RelationshipEngine;
    use crate::engine::rule_engine::RuleEngine;
    use crate::repository::KnowledgeRepository;
    use tempfile::TempDir;

    pub async fn setup_repo() -> (TempDir, KnowledgeRepository) {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();
        let repo = KnowledgeRepository::new(db.pool().clone());
        (tmp, repo)
    }

    pub async fn setup_engines() -> (
        TempDir,
        NodeEngine,
        RelationshipEngine,
        EventEngine,
        RuleEngine,
    ) {
        let (tmp, repo) = setup_repo().await;
        let ne = NodeEngine::new(repo.clone());
        let re = RelationshipEngine::new(repo.clone());
        let ee = EventEngine::new(repo.clone());
        let rle = RuleEngine::new(repo);
        (tmp, ne, re, ee, rle)
    }

    pub async fn setup_knowledge_engine()
    -> (TempDir, crate::engine::knowledge_engine::KnowledgeEngine) {
        let (tmp, repo) = setup_repo().await;
        let ke = crate::engine::knowledge_engine::KnowledgeEngine::new_internal(repo);
        (tmp, ke)
    }
}
