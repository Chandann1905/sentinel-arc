-- Migration 002: Create relationships table
-- Schema matches TRD exactly. Foreign keys reference nodes.

CREATE TABLE IF NOT EXISTS relationships (
    id                TEXT    PRIMARY KEY NOT NULL,
    source_node       TEXT    NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    target_node       TEXT    NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    relationship_type TEXT    NOT NULL,
    confidence        INTEGER NOT NULL DEFAULT 100,
    metadata          TEXT    NOT NULL DEFAULT '{}',
    created_at        INTEGER NOT NULL
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_relationships_source ON relationships(source_node);
CREATE INDEX IF NOT EXISTS idx_relationships_target ON relationships(target_node);
CREATE INDEX IF NOT EXISTS idx_relationships_type ON relationships(relationship_type);
