-- Migration 003: Create events table
-- Events are immutable: append-only, never updated, never deleted.

CREATE TABLE IF NOT EXISTS events (
    id         TEXT    PRIMARY KEY NOT NULL,
    event_type TEXT    NOT NULL,
    entity_id  TEXT    NOT NULL,
    payload    TEXT    NOT NULL DEFAULT '{}',
    timestamp  INTEGER NOT NULL,
    author     TEXT    NOT NULL DEFAULT 'system'
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_events_entity ON events(entity_id);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
