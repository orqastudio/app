-- Migration 004: Add sdk_session_id column to sessions table.
--
-- Stores the Claude Agent SDK session UUID so the sidecar can resume
-- SDK conversations after an app restart.

-- Idempotent: SQLite ALTER TABLE ADD COLUMN fails if column exists,
-- so we guard with a pragma check.

-- NOTE: SQLite does not support IF NOT EXISTS on ALTER TABLE ADD COLUMN,
-- so we use a separate check. The Rust migration code handles idempotency.
ALTER TABLE sessions ADD COLUMN sdk_session_id TEXT;
