//! SQLite implementation of the `RuleStore` trait.

use sqlx::SqlitePool;

use project_brain_core::RuleId;
use project_brain_core::error::{BrainError, BrainResult};
use project_brain_core::traits::RuleStore;
use project_brain_core::{Rule, RuleCategory, RuleSeverity};

/// SQLite-backed rule storage.
#[derive(Debug, Clone)]
pub struct SqliteRuleStore {
    pool: SqlitePool,
}

impl SqliteRuleStore {
    /// Create a new store backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct RuleRow {
    id: String,
    name: String,
    category: String,
    value: String,
    severity: String,
    enabled: bool,
}

fn row_to_rule(row: RuleRow) -> BrainResult<Rule> {
    let category: RuleCategory =
        serde_json::from_value(serde_json::Value::String(row.category.clone()))
            .map_err(|_| BrainError::storage(format!("Unknown rule category: {}", row.category)))?;
    let severity: RuleSeverity =
        serde_json::from_value(serde_json::Value::String(row.severity.clone()))
            .map_err(|_| BrainError::storage(format!("Unknown rule severity: {}", row.severity)))?;

    Ok(Rule {
        id: RuleId::from_string(row.id),
        name: row.name,
        category,
        value: row.value,
        severity,
        enabled: row.enabled,
    })
}

fn category_to_string(cat: &RuleCategory) -> BrainResult<String> {
    Ok(serde_json::to_value(cat)
        .map_err(|e| BrainError::storage(e.to_string()))?
        .as_str()
        .unwrap_or_default()
        .to_string())
}

impl RuleStore for SqliteRuleStore {
    async fn create(&self, rule: &Rule) -> BrainResult<()> {
        let id = rule.id.as_str();
        let category = category_to_string(&rule.category)?;
        let severity = serde_json::to_value(rule.severity)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();

        sqlx::query(
            "INSERT INTO rules (id, name, category, value, severity, enabled) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(&rule.name)
        .bind(&category)
        .bind(&rule.value)
        .bind(&severity)
        .bind(rule.enabled)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                BrainError::duplicate("rule", id)
            } else {
                BrainError::storage(format!("Failed to create rule: {e}"))
            }
        })?;

        Ok(())
    }

    async fn get(&self, id: &RuleId) -> BrainResult<Rule> {
        let row = sqlx::query_as::<_, RuleRow>(
            "SELECT id, name, category, value, severity, enabled FROM rules WHERE id = ?",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to get rule: {e}")))?
        .ok_or_else(|| BrainError::not_found("rule", id.as_str()))?;

        row_to_rule(row)
    }

    async fn update(&self, rule: &Rule) -> BrainResult<()> {
        let id = rule.id.as_str();
        let category = category_to_string(&rule.category)?;
        let severity = serde_json::to_value(rule.severity)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();

        let result = sqlx::query(
            "UPDATE rules SET name = ?, category = ?, value = ?, severity = ?, enabled = ? WHERE id = ?",
        )
        .bind(&rule.name)
        .bind(&category)
        .bind(&rule.value)
        .bind(&severity)
        .bind(rule.enabled)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to update rule: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(BrainError::not_found("rule", id));
        }
        Ok(())
    }

    async fn delete(&self, id: &RuleId) -> BrainResult<()> {
        let result = sqlx::query("DELETE FROM rules WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| BrainError::storage(format!("Failed to delete rule: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(BrainError::not_found("rule", id.as_str()));
        }
        Ok(())
    }

    async fn list_by_category(&self, category: RuleCategory) -> BrainResult<Vec<Rule>> {
        let cat_str = category_to_string(&category)?;
        let rows = sqlx::query_as::<_, RuleRow>(
            "SELECT id, name, category, value, severity, enabled FROM rules WHERE category = ?",
        )
        .bind(cat_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list rules: {e}")))?;

        rows.into_iter().map(row_to_rule).collect()
    }

    async fn list_enabled(&self) -> BrainResult<Vec<Rule>> {
        let rows = sqlx::query_as::<_, RuleRow>(
            "SELECT id, name, category, value, severity, enabled FROM rules WHERE enabled = 1",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list rules: {e}")))?;

        rows.into_iter().map(row_to_rule).collect()
    }

    async fn list_all(&self) -> BrainResult<Vec<Rule>> {
        let rows = sqlx::query_as::<_, RuleRow>(
            "SELECT id, name, category, value, severity, enabled FROM rules",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list all rules: {e}")))?;

        rows.into_iter().map(row_to_rule).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use tempfile::TempDir;

    async fn setup() -> (TempDir, SqliteRuleStore) {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();
        let store = SqliteRuleStore::new(db.pool().clone());
        (tmp, store)
    }

    #[tokio::test]
    async fn create_and_get_rule() {
        let (_tmp, store) = setup().await;
        let rule = Rule::new("backend", RuleCategory::Technology, "supabase");

        store.create(&rule).await.unwrap();
        let retrieved = store.get(&rule.id).await.unwrap();

        assert_eq!(retrieved.name, "backend");
        assert_eq!(retrieved.value, "supabase");
        assert_eq!(retrieved.category, RuleCategory::Technology);
        assert!(retrieved.enabled);
    }

    #[tokio::test]
    async fn update_rule() {
        let (_tmp, store) = setup().await;
        let mut rule = Rule::new("backend", RuleCategory::Technology, "firebase");
        store.create(&rule).await.unwrap();

        rule.value = "supabase".into();
        rule.enabled = false;
        store.update(&rule).await.unwrap();

        let retrieved = store.get(&rule.id).await.unwrap();
        assert_eq!(retrieved.value, "supabase");
        assert!(!retrieved.enabled);
    }

    #[tokio::test]
    async fn delete_rule() {
        let (_tmp, store) = setup().await;
        let rule = Rule::new("test", RuleCategory::Validation, "yes");
        store.create(&rule).await.unwrap();

        store.delete(&rule.id).await.unwrap();
        assert!(store.get(&rule.id).await.is_err());
    }

    #[tokio::test]
    async fn list_by_category() {
        let (_tmp, store) = setup().await;

        store
            .create(&Rule::new("backend", RuleCategory::Technology, "supabase"))
            .await
            .unwrap();
        store
            .create(&Rule::new("arch", RuleCategory::Architecture, "clean"))
            .await
            .unwrap();
        store
            .create(&Rule::new("state", RuleCategory::Technology, "riverpod"))
            .await
            .unwrap();

        let tech_rules = store
            .list_by_category(RuleCategory::Technology)
            .await
            .unwrap();
        assert_eq!(tech_rules.len(), 2);

        let arch_rules = store
            .list_by_category(RuleCategory::Architecture)
            .await
            .unwrap();
        assert_eq!(arch_rules.len(), 1);
    }

    #[tokio::test]
    async fn list_enabled() {
        let (_tmp, store) = setup().await;

        let r1 = Rule::new("enabled_rule", RuleCategory::Technology, "val");
        let mut r2 = Rule::new("disabled_rule", RuleCategory::Technology, "val");
        r2.enabled = false;

        store.create(&r1).await.unwrap();
        store.create(&r2).await.unwrap();

        let enabled = store.list_enabled().await.unwrap();
        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].name, "enabled_rule");
    }
}
