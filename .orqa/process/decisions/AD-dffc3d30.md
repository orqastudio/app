---
id: AD-dffc3d30
title: SQLite for All Structured Persistence
description: SQLite is the sole persistence layer for structured data. File-based artifacts are read from disk.
status: surpassed
created: 2026-03-02
updated: 2026-03-02
relationships:
  - target: DOC-2a7f1063
    type: documented-by
---
## Decision

SQLite is the sole persistence layer for structured data (sessions, messages, metrics, project config). File-based artifacts (docs, rules, agents) are read from disk.

## Rationale

SQLite is embedded, requires no external process, supports full-text search, and handles concurrent reads well. Perfect for a desktop app.

## Consequences

Schema managed via numbered migrations. Repository pattern in Rust for all database access. In-memory SQLite for testing.
