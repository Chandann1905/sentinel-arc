//! SQLite implementation of the `NodeStore` trait.

use chrono::{DateTime, TimeZone, Utc};
use sqlx::SqlitePool;

use project_brain_core::NodeId;
use project_brain_core::NodeType;
use project_brain_core::error::{BrainError, BrainResult};
use project_brain_core::traits::NodeStore;
use project_brain_core::{Confidence, Source};
use project_brain_core::{Node, NodeStatus};

/// SQLite-backed node storage.
#[derive(Debug, Clone)]
pub struct SqliteNodeStore {
    pool: SqlitePool,
}

impl SqliteNodeStore {
    /// Create a new store backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        node: &Node,
    ) -> BrainResult<()> {
        let id = node.id.as_str();
        let node_type = serde_json::to_value(node.node_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let status = serde_json::to_value(node.status)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let source = serde_json::to_value(node.source)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let confidence: i64 = node.confidence.into();
        let metadata = serde_json::to_string(&node.metadata)
            .map_err(|e| BrainError::storage(e.to_string()))?;
        let version = node.version as i64;
        let created_at = datetime_to_timestamp(&node.created_at);
        let updated_at = datetime_to_timestamp(&node.updated_at);

        sqlx::query(
            "INSERT INTO nodes (id, type, title, description, status, source, confidence, metadata, version, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(&node_type)
        .bind(&node.title)
        .bind(&node.description)
        .bind(&status)
        .bind(&source)
        .bind(confidence)
        .bind(&metadata)
        .bind(version)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                BrainError::duplicate("node", id)
            } else {
                BrainError::storage(format!("Failed to create node: {e}"))
            }
        })?;

        Ok(())
    }

    pub async fn update_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        node: &Node,
    ) -> BrainResult<()> {
        let id = node.id.as_str();
        let node_type = serde_json::to_value(node.node_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let status = serde_json::to_value(node.status)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let source = serde_json::to_value(node.source)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let confidence: i64 = node.confidence.into();
        let metadata = serde_json::to_string(&node.metadata)
            .map_err(|e| BrainError::storage(e.to_string()))?;
        let version = node.version as i64;
        let updated_at = datetime_to_timestamp(&node.updated_at);

        let result = sqlx::query(
            "UPDATE nodes SET type = ?, title = ?, description = ?, status = ?, source = ?, \
             confidence = ?, metadata = ?, version = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&node_type)
        .bind(&node.title)
        .bind(&node.description)
        .bind(&status)
        .bind(&source)
        .bind(confidence)
        .bind(&metadata)
        .bind(version)
        .bind(updated_at)
        .bind(id)
        .execute(&mut **tx)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to update node: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(BrainError::not_found("node", id));
        }

        Ok(())
    }

    pub async fn delete_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: &NodeId,
    ) -> BrainResult<()> {
        let result = sqlx::query("DELETE FROM nodes WHERE id = ?")
            .bind(id.as_str())
            .execute(&mut **tx)
            .await
            .map_err(|e| BrainError::storage(format!("Failed to delete node: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(BrainError::not_found("node", id.as_str()));
        }

        Ok(())
    }
}

/// Internal row type for reading nodes from SQLite.
#[derive(Debug, sqlx::FromRow)]
struct NodeRow {
    id: String,
    r#type: String,
    title: String,
    description: String,
    status: String,
    source: String,
    confidence: i64,
    metadata: String,
    version: i64,
    created_at: i64,
    updated_at: i64,
}

fn row_to_node(row: NodeRow) -> BrainResult<Node> {
    let node_type: NodeType = serde_json::from_value(serde_json::Value::String(row.r#type.clone()))
        .map_err(|_| BrainError::storage(format!("Unknown node type: {}", row.r#type)))?;
    let status: NodeStatus = serde_json::from_value(serde_json::Value::String(row.status.clone()))
        .map_err(|_| BrainError::storage(format!("Unknown node status: {}", row.status)))?;
    let source: Source = serde_json::from_value(serde_json::Value::String(row.source.clone()))
        .map_err(|_| BrainError::storage(format!("Unknown source: {}", row.source)))?;
    let metadata: serde_json::Value = serde_json::from_str(&row.metadata)
        .map_err(|e| BrainError::storage(format!("Invalid metadata JSON: {e}")))?;

    Ok(Node {
        id: NodeId::from_string(row.id),
        node_type,
        title: row.title,
        description: row.description,
        status,
        source,
        confidence: Confidence::new(row.confidence as u8),
        metadata,
        version: row.version as u32,
        created_at: timestamp_to_datetime(row.created_at),
        updated_at: timestamp_to_datetime(row.updated_at),
    })
}

fn timestamp_to_datetime(ts: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(ts, 0).single().unwrap_or_else(Utc::now)
}

fn datetime_to_timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}

impl NodeStore for SqliteNodeStore {
    async fn create(&self, node: &Node) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.create_tx(&mut tx, node).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, id: &NodeId) -> BrainResult<Node> {
        let row = sqlx::query_as::<_, NodeRow>(
            "SELECT id, type, title, description, status, source, confidence, metadata, version, created_at, updated_at \
             FROM nodes WHERE id = ?"
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to get node: {e}")))?
        .ok_or_else(|| BrainError::not_found("node", id.as_str()))?;

        row_to_node(row)
    }

    async fn update(&self, node: &Node) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.update_tx(&mut tx, node).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: &NodeId) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.delete_tx(&mut tx, id).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    async fn list_by_type(&self, node_type: NodeType) -> BrainResult<Vec<Node>> {
        let type_str = serde_json::to_value(node_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();

        let rows = sqlx::query_as::<_, NodeRow>(
            "SELECT id, type, title, description, status, source, confidence, metadata, version, created_at, updated_at \
             FROM nodes WHERE type = ? ORDER BY created_at DESC"
        )
        .bind(&type_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list nodes: {e}")))?;

        rows.into_iter().map(row_to_node).collect()
    }

    async fn list_all(&self) -> BrainResult<Vec<Node>> {
        let rows = sqlx::query_as::<_, NodeRow>(
            "SELECT id, type, title, description, status, source, confidence, metadata, version, created_at, updated_at \
             FROM nodes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list all nodes: {e}")))?;

        rows.into_iter().map(row_to_node).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use tempfile::TempDir;

    async fn setup() -> (TempDir, SqliteNodeStore) {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();
        let store = SqliteNodeStore::new(db.pool().clone());
        (tmp, store)
    }

    #[tokio::test]
    async fn create_and_get_node() {
        let (_tmp, store) = setup().await;
        let node = Node::new(NodeType::Feature, "Wallet");

        store.create(&node).await.unwrap();
        let retrieved = store.get(&node.id).await.unwrap();

        assert_eq!(retrieved.id, node.id);
        assert_eq!(retrieved.title, "Wallet");
        assert_eq!(retrieved.node_type, NodeType::Feature);
    }

    #[tokio::test]
    async fn get_nonexistent_returns_not_found() {
        let (_tmp, store) = setup().await;
        let result = store.get(&NodeId::from_string("nonexistent")).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_duplicate_returns_error() {
        let (_tmp, store) = setup().await;
        let node = Node::new(NodeType::Feature, "Wallet");
        store.create(&node).await.unwrap();
        let result = store.create(&node).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update_node() {
        let (_tmp, store) = setup().await;
        let mut node = Node::new(NodeType::Feature, "Wallet");
        store.create(&node).await.unwrap();

        node.title = "Wallet v2".into();
        node.version = 2;
        store.update(&node).await.unwrap();

        let retrieved = store.get(&node.id).await.unwrap();
        assert_eq!(retrieved.title, "Wallet v2");
        assert_eq!(retrieved.version, 2);
    }

    #[tokio::test]
    async fn delete_node() {
        let (_tmp, store) = setup().await;
        let node = Node::new(NodeType::Bug, "Race condition");
        store.create(&node).await.unwrap();

        store.delete(&node.id).await.unwrap();
        let result = store.get(&node.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_by_type() {
        let (_tmp, store) = setup().await;

        let f1 = Node::new(NodeType::Feature, "Wallet");
        let f2 = Node::new(NodeType::Feature, "Orders");
        let b1 = Node::new(NodeType::Bug, "Race condition");

        store.create(&f1).await.unwrap();
        store.create(&f2).await.unwrap();
        store.create(&b1).await.unwrap();

        let features = store.list_by_type(NodeType::Feature).await.unwrap();
        assert_eq!(features.len(), 2);

        let bugs = store.list_by_type(NodeType::Bug).await.unwrap();
        assert_eq!(bugs.len(), 1);
    }

    #[tokio::test]
    async fn list_all() {
        let (_tmp, store) = setup().await;

        store
            .create(&Node::new(NodeType::Feature, "A"))
            .await
            .unwrap();
        store.create(&Node::new(NodeType::Bug, "B")).await.unwrap();

        let all = store.list_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn node_with_tags_roundtrips() {
        let (_tmp, store) = setup().await;
        let mut node = Node::new(NodeType::Feature, "Tagged");
        node.set_tags(vec!["backend".into(), "payment".into()]);

        store.create(&node).await.unwrap();
        let retrieved = store.get(&node.id).await.unwrap();
        assert_eq!(retrieved.tags(), vec!["backend", "payment"]);
    }
}
