-- Migration 003: Enforcement violations table
--
-- Tracks every rule enforcement event (block or warn) that occurs during tool execution.
-- Useful for auditing which rules triggered, trending violations over time, and feeding
-- the self-learning loop.

CREATE TABLE IF NOT EXISTS enforcement_violations (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    rule_name  TEXT    NOT NULL,
    action     TEXT    NOT NULL,   -- 'block' or 'warn'
    tool_name  TEXT    NOT NULL,   -- e.g. 'write_file', 'bash'
    detail     TEXT,               -- file path, command snippet, etc.
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);
