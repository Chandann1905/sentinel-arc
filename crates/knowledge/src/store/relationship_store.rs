//! SQLite implementation of the `RelationshipStore` trait.

use chrono::{DateTime, TimeZone, Utc};
use sqlx::SqlitePool;

use project_brain_core::Confidence;
use project_brain_core::Relationship;
use project_brain_core::RelationshipType;
use project_brain_core::error::{BrainError, BrainResult};
use project_brain_core::traits::RelationshipStore;
use project_brain_core::{NodeId, RelationshipId};

/// SQLite-backed relationship storage.
#[derive(Debug, Clone)]
pub struct SqliteRelationshipStore {
    pool: SqlitePool,
}

impl SqliteRelationshipStore {
    /// Create a new store backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        relationship: &Relationship,
    ) -> BrainResult<()> {
        let id = relationship.id.as_str();
        let source_id = relationship.source_node.as_str();
        let target_id = relationship.target_node.as_str();
        let rel_type = serde_json::to_value(relationship.relationship_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let confidence: i64 = relationship.confidence.into();
        let metadata = serde_json::to_string(&relationship.metadata)
            .map_err(|e| BrainError::storage(e.to_string()))?;
        let created_at = datetime_to_timestamp(&relationship.created_at);

        sqlx::query(
            "INSERT INTO relationships (id, source_node, target_node, relationship_type, confidence, metadata, created_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(source_id)
        .bind(target_id)
        .bind(&rel_type)
        .bind(confidence)
        .bind(&metadata)
        .bind(created_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                BrainError::duplicate("relationship", id)
            } else {
                BrainError::storage(format!("Failed to create relationship: {e}"))
            }
        })?;

        Ok(())
    }

    pub async fn delete_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: &RelationshipId,
    ) -> BrainResult<()> {
        let result = sqlx::query("DELETE FROM relationships WHERE id = ?")
            .bind(id.as_str())
            .execute(&mut **tx)
            .await
            .map_err(|e| BrainError::storage(format!("Failed to delete relationship: {e}")))?;

        if result.rows_affected() == 0 {
            return Err(BrainError::not_found("relationship", id.as_str()));
        }

        Ok(())
    }
}

fn datetime_to_timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}

#[derive(Debug, sqlx::FromRow)]
struct RelRow {
    id: String,
    source_node: String,
    target_node: String,
    relationship_type: String,
    confidence: i64,
    metadata: String,
    created_at: i64,
}

fn row_to_relationship(row: RelRow) -> BrainResult<Relationship> {
    let rel_type: RelationshipType = serde_json::from_value(serde_json::Value::String(
        row.relationship_type.clone(),
    ))
    .map_err(|_| {
        BrainError::storage(format!(
            "Unknown relationship type: {}",
            row.relationship_type
        ))
    })?;
    let metadata: serde_json::Value = serde_json::from_str(&row.metadata)
        .map_err(|e| BrainError::storage(format!("Invalid metadata JSON: {e}")))?;

    Ok(Relationship {
        id: RelationshipId::from_string(row.id),
        source_node: NodeId::from_string(row.source_node),
        target_node: NodeId::from_string(row.target_node),
        relationship_type: rel_type,
        confidence: Confidence::new(row.confidence as u8),
        metadata,
        created_at: Utc
            .timestamp_opt(row.created_at, 0)
            .single()
            .unwrap_or_else(Utc::now),
    })
}

fn rel_type_to_string(rt: &RelationshipType) -> BrainResult<String> {
    Ok(serde_json::to_value(rt)
        .map_err(|e| BrainError::storage(e.to_string()))?
        .as_str()
        .unwrap_or_default()
        .to_string())
}

impl RelationshipStore for SqliteRelationshipStore {
    async fn create(&self, relationship: &Relationship) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.create_tx(&mut tx, relationship).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, id: &RelationshipId) -> BrainResult<Relationship> {
        let row = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships WHERE id = ?"
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to get relationship: {e}")))?
        .ok_or_else(|| BrainError::not_found("relationship", id.as_str()))?;

        row_to_relationship(row)
    }

    async fn delete(&self, id: &RelationshipId) -> BrainResult<()> {
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

    async fn find_by_source(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        let rows = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships WHERE source_node = ? ORDER BY created_at DESC"
        )
        .bind(node_id.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find relationships: {e}")))?;

        rows.into_iter().map(row_to_relationship).collect()
    }

    async fn find_by_target(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        let rows = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships WHERE target_node = ? ORDER BY created_at DESC"
        )
        .bind(node_id.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find relationships: {e}")))?;

        rows.into_iter().map(row_to_relationship).collect()
    }

    async fn find_by_type(&self, rel_type: RelationshipType) -> BrainResult<Vec<Relationship>> {
        let type_str = rel_type_to_string(&rel_type)?;
        let rows = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships WHERE relationship_type = ? ORDER BY created_at DESC"
        )
        .bind(&type_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find relationships: {e}")))?;

        rows.into_iter().map(row_to_relationship).collect()
    }

    async fn find_by_node(&self, node_id: &NodeId) -> BrainResult<Vec<Relationship>> {
        let id_str = node_id.as_str();
        let rows = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships WHERE source_node = ? OR target_node = ? ORDER BY created_at DESC"
        )
        .bind(id_str)
        .bind(id_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find relationships: {e}")))?;

        rows.into_iter().map(row_to_relationship).collect()
    }

    async fn list_all(&self) -> BrainResult<Vec<Relationship>> {
        let rows = sqlx::query_as::<_, RelRow>(
            "SELECT id, source_node, target_node, relationship_type, confidence, metadata, created_at \
             FROM relationships ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list relationships: {e}")))?;

        rows.into_iter().map(row_to_relationship).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use crate::store::node_store::SqliteNodeStore;
    use project_brain_core::Node;
    use project_brain_core::NodeType;
    use project_brain_core::traits::NodeStore;
    use tempfile::TempDir;

    async fn setup() -> (TempDir, SqliteNodeStore, SqliteRelationshipStore) {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();
        let ns = SqliteNodeStore::new(db.pool().clone());
        let rs = SqliteRelationshipStore::new(db.pool().clone());
        (tmp, ns, rs)
    }

    async fn create_two_nodes(ns: &SqliteNodeStore) -> (NodeId, NodeId) {
        let a = Node::new(NodeType::Feature, "A");
        let b = Node::new(NodeType::Feature, "B");
        let a_id = a.id.clone();
        let b_id = b.id.clone();
        ns.create(&a).await.unwrap();
        ns.create(&b).await.unwrap();
        (a_id, b_id)
    }

    #[tokio::test]
    async fn create_and_get_relationship() {
        let (_tmp, ns, rs) = setup().await;
        let (a, b) = create_two_nodes(&ns).await;
        let rel = Relationship::new(a.clone(), b.clone(), RelationshipType::DependsOn);

        rs.create(&rel).await.unwrap();
        let retrieved = rs.get(&rel.id).await.unwrap();

        assert_eq!(retrieved.source_node, a);
        assert_eq!(retrieved.target_node, b);
        assert_eq!(retrieved.relationship_type, RelationshipType::DependsOn);
    }

    #[tokio::test]
    async fn delete_relationship() {
        let (_tmp, ns, rs) = setup().await;
        let (a, b) = create_two_nodes(&ns).await;
        let rel = Relationship::new(a, b, RelationshipType::Calls);

        rs.create(&rel).await.unwrap();
        rs.delete(&rel.id).await.unwrap();

        assert!(rs.get(&rel.id).await.is_err());
    }

    #[tokio::test]
    async fn find_by_source_and_target() {
        let (_tmp, ns, rs) = setup().await;
        let (a, b) = create_two_nodes(&ns).await;

        let rel = Relationship::new(a.clone(), b.clone(), RelationshipType::Uses);
        rs.create(&rel).await.unwrap();

        let from_a = rs.find_by_source(&a).await.unwrap();
        assert_eq!(from_a.len(), 1);

        let to_b = rs.find_by_target(&b).await.unwrap();
        assert_eq!(to_b.len(), 1);

        let from_b = rs.find_by_source(&b).await.unwrap();
        assert!(from_b.is_empty());
    }

    #[tokio::test]
    async fn find_by_node_returns_both_directions() {
        let (_tmp, ns, rs) = setup().await;
        let (a, b) = create_two_nodes(&ns).await;

        let r1 = Relationship::new(a.clone(), b.clone(), RelationshipType::DependsOn);
        let r2 = Relationship::new(b.clone(), a.clone(), RelationshipType::Blocks);
        rs.create(&r1).await.unwrap();
        rs.create(&r2).await.unwrap();

        let rels = rs.find_by_node(&a).await.unwrap();
        assert_eq!(rels.len(), 2);
    }

    #[tokio::test]
    async fn find_by_type() {
        let (_tmp, ns, rs) = setup().await;
        let (a, b) = create_two_nodes(&ns).await;

        rs.create(&Relationship::new(
            a.clone(),
            b.clone(),
            RelationshipType::DependsOn,
        ))
        .await
        .unwrap();
        rs.create(&Relationship::new(a, b, RelationshipType::Uses))
            .await
            .unwrap();

        let deps = rs.find_by_type(RelationshipType::DependsOn).await.unwrap();
        assert_eq!(deps.len(), 1);
    }
}
