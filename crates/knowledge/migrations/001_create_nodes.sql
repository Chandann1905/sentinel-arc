-- Migration 001: Create nodes table
-- Schema matches TRD exactly.

CREATE TABLE IF NOT EXISTS nodes (
    id          TEXT    PRIMARY KEY NOT NULL,
    type        TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    status      TEXT    NOT NULL DEFAULT 'active',
    source      TEXT    NOT NULL DEFAULT 'manual',
    confidence  INTEGER NOT NULL DEFAULT 50,
    metadata    TEXT    NOT NULL DEFAULT '{}',
    version     INTEGER NOT NULL DEFAULT 1,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_nodes_type ON nodes(type);
CREATE INDEX IF NOT EXISTS idx_nodes_status ON nodes(status);
CREATE INDEX IF NOT EXISTS idx_nodes_created_at ON nodes(created_at);
