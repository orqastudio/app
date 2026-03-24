-- Health snapshots: graph metrics captured at each integrity scan.
-- Used for trend sparklines on the dashboard.
CREATE TABLE IF NOT EXISTS health_snapshots (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    node_count      INTEGER NOT NULL DEFAULT 0,
    edge_count      INTEGER NOT NULL DEFAULT 0,
    orphan_count    INTEGER NOT NULL DEFAULT 0,
    broken_ref_count INTEGER NOT NULL DEFAULT 0,
    error_count     INTEGER NOT NULL DEFAULT 0,
    warning_count   INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_health_snapshots_project
    ON health_snapshots(project_id, id DESC);
