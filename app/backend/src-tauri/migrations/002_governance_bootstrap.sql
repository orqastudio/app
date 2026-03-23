-- Phase 2b: Governance Bootstrap Schema

CREATE TABLE IF NOT EXISTS governance_analyses (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    scan_data       TEXT NOT NULL,
    summary         TEXT NOT NULL,
    strengths       TEXT NOT NULL,
    gaps            TEXT NOT NULL,
    session_id      INTEGER REFERENCES sessions(id),
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_governance_analyses_project
    ON governance_analyses(project_id, created_at);

CREATE TABLE IF NOT EXISTS governance_recommendations (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    analysis_id     INTEGER NOT NULL REFERENCES governance_analyses(id) ON DELETE CASCADE,
    category        TEXT NOT NULL,
    priority        TEXT NOT NULL
                    CHECK (priority IN ('critical', 'recommended', 'optional')),
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    artifact_type   TEXT NOT NULL,
    target_path     TEXT NOT NULL,
    content         TEXT NOT NULL,
    rationale       TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending'
                    CHECK (status IN ('pending', 'approved', 'rejected', 'applied')),
    applied_at      TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_governance_recommendations_project
    ON governance_recommendations(project_id, status);
CREATE INDEX IF NOT EXISTS idx_governance_recommendations_analysis
    ON governance_recommendations(analysis_id);
