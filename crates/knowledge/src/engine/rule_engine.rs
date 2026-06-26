use sentinel_arc_core::error::BrainResult;
use sentinel_arc_core::traits::RuleStore;
use sentinel_arc_core::{Rule, RuleCategory, RuleId};

use crate::repository::KnowledgeRepository;

/// The Rule Engine — manages the lifecycle and query of data-driven rules.
///
/// Encapsulates operations around rules to provide a symmetrical engine boundary.
/// In the future, this engine will be responsible for rule validation, caching,
/// and emitting events on rule changes.
#[derive(Debug, Clone)]
pub(crate) struct RuleEngine {
    repo: KnowledgeRepository,
}

#[allow(dead_code)]
impl RuleEngine {
    /// Initialize a new RuleEngine with the given repository.
    pub(crate) fn new(repo: KnowledgeRepository) -> Self {
        Self { repo }
    }

    /// Retrieve a specific rule by ID.
    pub async fn get_rule(&self, id: &RuleId) -> BrainResult<Rule> {
        self.repo.rule_store().get(id).await
    }

    /// Retrieve all rules.
    pub async fn list_rules(&self) -> BrainResult<Vec<Rule>> {
        self.repo.rule_store().list_all().await
    }

    /// Retrieve only enabled rules.
    pub async fn list_enabled_rules(&self) -> BrainResult<Vec<Rule>> {
        self.repo.rule_store().list_enabled().await
    }

    /// Retrieve rules by category.
    pub async fn list_rules_by_category(&self, category: RuleCategory) -> BrainResult<Vec<Rule>> {
        self.repo.rule_store().list_by_category(category).await
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_helpers::setup_knowledge_engine;

    #[tokio::test]
    async fn get_rule_success() {
        let (_tmp, ke) = setup_knowledge_engine().await;
        let rules = ke.list_rules().await.unwrap();
        assert!(rules.is_empty());
    }
}

#[cfg(test)]
mod missing_rule_tests {
    use crate::test_utils::test_helpers::setup_engines;
    use sentinel_arc_core::RuleCategory;

    #[tokio::test]
    async fn test_list_rules() {
        let (_tmp, _, _, _, rule_engine) = setup_engines().await;
        let rules = rule_engine.list_rules().await.unwrap();
        assert!(rules.is_empty());
    }

    #[tokio::test]
    async fn test_list_rules_by_category() {
        let (_tmp, _, _, _, rule_engine) = setup_engines().await;
        let rules = rule_engine
            .list_rules_by_category(RuleCategory::Architecture)
            .await
            .unwrap();
        assert!(rules.is_empty());
    }

    #[tokio::test]
    async fn test_list_enabled_rules() {
        let (_tmp, _, _, _, rule_engine) = setup_engines().await;
        let rules = rule_engine.list_enabled_rules().await.unwrap();
        assert!(rules.is_empty());
    }
}
