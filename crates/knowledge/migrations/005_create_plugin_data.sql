-- Migration 005: Create plugin_data table
-- Generic key-value store for plugin-specific data.

CREATE TABLE IF NOT EXISTS plugin_data (
    id     TEXT PRIMARY KEY NOT NULL,
    plugin TEXT NOT NULL,
    key    TEXT NOT NULL,
    value  TEXT NOT NULL DEFAULT '{}'
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_plugin_data_plugin ON plugin_data(plugin);
CREATE UNIQUE INDEX IF NOT EXISTS idx_plugin_data_plugin_key ON plugin_data(plugin, key);
