//! SQLite implementation of the `EventStore` trait.
//!
//! Events are append-only: create and query only. No update or delete.

use chrono::{DateTime, TimeZone, Utc};
use sqlx::SqlitePool;

use sentinel_arc_core::Event;
use sentinel_arc_core::EventId;
use sentinel_arc_core::EventType;
use sentinel_arc_core::error::{BrainError, BrainResult};
use sentinel_arc_core::traits::EventStore;

/// SQLite-backed event storage. Append-only by design.
#[derive(Debug, Clone)]
pub struct SqliteEventStore {
    pool: SqlitePool,
}

impl SqliteEventStore {
    /// Create a new store backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        event: &Event,
    ) -> BrainResult<()> {
        let id = event.id.as_str();
        let event_type = serde_json::to_value(event.event_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        let payload = serde_json::to_string(&event.payload)
            .map_err(|e| BrainError::storage(e.to_string()))?;
        let timestamp = datetime_to_timestamp(&event.timestamp);

        sqlx::query(
            "INSERT INTO events (id, event_type, entity_id, payload, timestamp, author) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(&event_type)
        .bind(&event.entity_id)
        .bind(&payload)
        .bind(timestamp)
        .bind(&event.author)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                BrainError::duplicate("event", id)
            } else {
                BrainError::storage(format!("Failed to create event: {e}"))
            }
        })?;

        Ok(())
    }
}

fn datetime_to_timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}

#[derive(Debug, sqlx::FromRow)]
struct EventRow {
    id: String,
    event_type: String,
    entity_id: String,
    payload: String,
    timestamp: i64,
    author: String,
}

fn row_to_event(row: EventRow) -> BrainResult<Event> {
    let event_type: EventType =
        serde_json::from_value(serde_json::Value::String(row.event_type.clone()))
            .map_err(|_| BrainError::storage(format!("Unknown event type: {}", row.event_type)))?;
    let payload: serde_json::Value = serde_json::from_str(&row.payload)
        .map_err(|e| BrainError::storage(format!("Invalid payload JSON: {e}")))?;

    Ok(Event {
        id: EventId::from_string(row.id),
        event_type,
        entity_id: row.entity_id,
        payload,
        timestamp: Utc
            .timestamp_opt(row.timestamp, 0)
            .single()
            .unwrap_or_else(Utc::now),
        author: row.author,
    })
}

impl EventStore for SqliteEventStore {
    async fn create(&self, event: &Event) -> BrainResult<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        self.create_tx(&mut tx, event).await?;
        tx.commit()
            .await
            .map_err(|e| BrainError::storage(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, id: &EventId) -> BrainResult<Event> {
        let row = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, entity_id, payload, timestamp, author \
             FROM events WHERE id = ?",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to get event: {e}")))?
        .ok_or_else(|| BrainError::not_found("event", id.as_str()))?;

        row_to_event(row)
    }

    async fn find_by_entity(&self, entity_id: &str) -> BrainResult<Vec<Event>> {
        let rows = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, entity_id, payload, timestamp, author \
             FROM events WHERE entity_id = ? ORDER BY timestamp ASC",
        )
        .bind(entity_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find events: {e}")))?;

        rows.into_iter().map(row_to_event).collect()
    }

    async fn find_by_type(&self, event_type: EventType) -> BrainResult<Vec<Event>> {
        let type_str = serde_json::to_value(event_type)
            .map_err(|e| BrainError::storage(e.to_string()))?
            .as_str()
            .unwrap_or_default()
            .to_string();

        let rows = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, entity_id, payload, timestamp, author \
             FROM events WHERE event_type = ? ORDER BY timestamp ASC",
        )
        .bind(&type_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find events: {e}")))?;

        rows.into_iter().map(row_to_event).collect()
    }

    async fn find_by_time_range(&self, start: i64, end: i64) -> BrainResult<Vec<Event>> {
        let rows = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, entity_id, payload, timestamp, author \
             FROM events WHERE timestamp >= ? AND timestamp <= ? ORDER BY timestamp ASC",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to find events: {e}")))?;

        rows.into_iter().map(row_to_event).collect()
    }

    async fn list_recent(&self, count: u32) -> BrainResult<Vec<Event>> {
        let count_i64 = count as i64;
        let rows = sqlx::query_as::<_, EventRow>(
            "SELECT id, event_type, entity_id, payload, timestamp, author \
             FROM events ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(count_i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BrainError::storage(format!("Failed to list events: {e}")))?;

        rows.into_iter().map(row_to_event).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use tempfile::TempDir;

    async fn setup() -> (TempDir, SqliteEventStore) {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();
        let store = SqliteEventStore::new(db.pool().clone());
        (tmp, store)
    }

    #[tokio::test]
    async fn create_and_get_event() {
        let (_tmp, store) = setup().await;
        let event = Event::new(
            EventType::NodeCreated,
            "node-1",
            serde_json::json!({"title": "Wallet"}),
            "system",
        );

        store.create(&event).await.unwrap();
        let retrieved = store.get(&event.id).await.unwrap();

        assert_eq!(retrieved.event_type, EventType::NodeCreated);
        assert_eq!(retrieved.entity_id, "node-1");
        assert_eq!(retrieved.author, "system");
    }

    #[tokio::test]
    async fn find_by_entity() {
        let (_tmp, store) = setup().await;

        let e1 = Event::new(
            EventType::NodeCreated,
            "node-1",
            serde_json::json!({}),
            "user",
        );
        let e2 = Event::new(
            EventType::NodeUpdated,
            "node-1",
            serde_json::json!({}),
            "user",
        );
        let e3 = Event::new(
            EventType::NodeCreated,
            "node-2",
            serde_json::json!({}),
            "user",
        );

        store.create(&e1).await.unwrap();
        store.create(&e2).await.unwrap();
        store.create(&e3).await.unwrap();

        let events = store.find_by_entity("node-1").await.unwrap();
        assert_eq!(events.len(), 2);
    }

    #[tokio::test]
    async fn find_by_type() {
        let (_tmp, store) = setup().await;

        store
            .create(&Event::new(
                EventType::NodeCreated,
                "a",
                serde_json::json!({}),
                "u",
            ))
            .await
            .unwrap();
        store
            .create(&Event::new(
                EventType::NodeUpdated,
                "b",
                serde_json::json!({}),
                "u",
            ))
            .await
            .unwrap();
        store
            .create(&Event::new(
                EventType::NodeCreated,
                "c",
                serde_json::json!({}),
                "u",
            ))
            .await
            .unwrap();

        let created = store.find_by_type(EventType::NodeCreated).await.unwrap();
        assert_eq!(created.len(), 2);
    }

    #[tokio::test]
    async fn list_recent() {
        let (_tmp, store) = setup().await;

        for i in 0..5 {
            store
                .create(&Event::new(
                    EventType::NodeCreated,
                    format!("node-{i}").as_str(),
                    serde_json::json!({}),
                    "user",
                ))
                .await
                .unwrap();
        }

        let recent = store.list_recent(3).await.unwrap();
        assert_eq!(recent.len(), 3);
    }

    #[tokio::test]
    async fn events_are_append_only() {
        // The EventStore trait intentionally has no update() or delete() methods.
        // This test verifies that events persist and are queryable.
        let (_tmp, store) = setup().await;
        let event = Event::new(EventType::BugOpened, "bug-1", serde_json::json!({}), "dev");

        store.create(&event).await.unwrap();
        let retrieved = store.get(&event.id).await.unwrap();
        assert_eq!(retrieved.event_type, EventType::BugOpened);
        // No way to delete or update — enforced at trait level.
    }
}
