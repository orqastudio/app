-- Migration 010: Placeholder for extended health snapshot columns.
--
-- The actual ALTER TABLE statements are executed by run_migration_010() in db.rs
-- using the idempotent pragma_table_info pattern to avoid duplicate-column errors
-- on databases that already have these columns.
SELECT 1;
