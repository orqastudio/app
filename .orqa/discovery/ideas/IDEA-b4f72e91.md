---
id: IDEA-b4f72e91
type: idea
title: Database layer for generated queryable artifacts
description: Replace JSON files with a queryable database (DuckDB, SQLite, or Mongo) for generated artifacts like prompt-registry.json, resolved workflows, and composed schemas
status: captured
created: 2026-03-29
---

## Context

Multiple generated artifacts are currently JSON files that get queried at runtime:

- `prompt-registry.json` — knowledge entries queried by role, tier, stage
- `schema.composed.json` — artifact types and relationships queried by key
- `*.resolved.json` — workflow state machines queried by artifact type

As the plugin ecosystem grows, these files will get larger and querying JSON in memory becomes less efficient.

## Proposal

Replace generated JSON artifacts with a queryable database layer. Candidates:

- **DuckDB** — already in the stack for search indexing. Columnar, fast analytical queries. Good for read-heavy workloads.
- **SQLite** — already used by the Tauri app for state. Widely supported.
- **MongoDB** — document-oriented, natural fit for JSON-shaped data. Adds deployment complexity.

## Considerations

- DuckDB is already a dependency (search.duckdb exists)
- Generated artifacts are write-once-read-many (install/watch writes, runtime reads)
- Daemon endpoints already query these artifacts — moving to DB would be transparent to consumers
- Migration path: generate both JSON + DB entries initially, deprecate JSON reads
