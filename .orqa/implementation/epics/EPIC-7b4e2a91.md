---
id: EPIC-7b4e2a91
type: epic
title: Artifact Storage Migration Pipeline
description: Build the one-way migration pipeline that reads existing .orqa/ markdown files on first launch after SurrealDB integration, parses their frontmatter and content, and inserts them as typed SurrealDB records with full relationship graphs intact. Includes integrity validation and rollback on failure.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: next
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Stream 1 — artifact storage migration enables SurrealDB to become source of truth
  - target: EPIC-3d9fc182
    type: depends-on
    rationale: Requires SurrealDB daemon integration to be complete before migration can run
---

## Context

Once SurrealDB is embedded in the daemon, existing .orqa/ markdown artifacts need migrating into the graph database. This is a one-way, idempotent migration that runs on daemon start when SurrealDB is empty and .orqa/ files exist.

## Acceptance Criteria

- [ ] Migration detects first-run condition (empty SurrealDB, non-empty .orqa/) automatically
- [ ] All non-task artifact types are parsed and inserted with correct type, status, and frontmatter fields
- [ ] All relationships in artifact frontmatter are inserted as typed SurrealDB edges
- [ ] Migration is idempotent — running it twice produces the same result
- [ ] Failed migration rolls back and leaves .orqa/ files untouched
- [ ] Migration report written to .state/ with counts and any skipped files
