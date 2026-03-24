-- Remove artifacts SQLite table and FTS5 index.
-- Artifacts are file-based (.orqa/) — no DB persistence needed (AD-032).
DROP TABLE IF EXISTS artifacts_fts;
DROP TABLE IF EXISTS artifacts;
