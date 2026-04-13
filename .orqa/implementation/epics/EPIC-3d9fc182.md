---
id: EPIC-3d9fc182
type: epic
title: SurrealDB Daemon Integration
description: Embed SurrealDB into the OrqaStudio daemon process, replacing the HashMap-based artifact store with a live SurrealDB instance serving as source of truth for all artifact queries and writes. Completes the SurrealDB PoC (engine/graph-db) and integrates it with the daemon HTTP API.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: now
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 3
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Critical path Stream 1 — SurrealDB is the artifact graph foundation
---

## Context

The SurrealDB PoC lives in `engine/graph-db/` as a standalone Rust crate. It proves the graph model works but is not wired into the daemon. The daemon currently uses a HashMap-based artifact store seeded from markdown file scans. This epic replaces that store with an embedded SurrealDB instance.

## Acceptance Criteria

- [ ] SurrealDB embedded instance starts with the daemon (no separate process required)
- [ ] All artifact reads go through SurrealDB queries, not file system scans
- [ ] All artifact writes are committed to SurrealDB (daemon-side, before any UI notification)
- [ ] The daemon HTTP API returns SurrealDB-backed responses
- [ ] The graph-db PoC crate is integrated (not duplicated) into the daemon crate
- [ ] Daemon starts cleanly with an empty SurrealDB store (first-run case)
