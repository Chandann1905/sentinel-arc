-- Migration 004: Create rules table
-- Rules are data, not code.

CREATE TABLE IF NOT EXISTS rules (
    id       TEXT    PRIMARY KEY NOT NULL,
    name     TEXT    NOT NULL,
    category TEXT    NOT NULL,
    value    TEXT    NOT NULL,
    severity TEXT    NOT NULL DEFAULT 'warning',
    enabled  INTEGER NOT NULL DEFAULT 1
);

-- Index for common query patterns
CREATE INDEX IF NOT EXISTS idx_rules_category ON rules(category);
CREATE INDEX IF NOT EXISTS idx_rules_enabled ON rules(enabled);
