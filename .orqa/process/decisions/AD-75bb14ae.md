---
id: AD-75bb14ae
type: decision
title: SQLite for All Structured Persistence
description: SQLite is the sole persistence layer for structured data. File-based artifacts are read from disk.
status: surpassed
created: 2026-03-02
updated: 2026-03-02
relationships:
  - target: AD-859ed163
    type: evolves-into
    rationale: "AD-859ed163 scopes SQLite to conversation persistence only, replacing this broader SQLite-for-all decision"
  - target: DOC-bf647454
    type: documented-by
---
## Decision

SQLite is the sole persistence layer for structured data (sessions, messages, metrics, project config). File-based artifacts (docs, rules, agents) are read from disk.

## Rationale

SQLite is embedded, requires no external process, supports full-text search, and handles concurrent reads well. Perfect for a desktop app.

## Consequences

Schema managed via numbered migrations. Repository pattern in Rust for all database access. In-memory SQLite for testing.