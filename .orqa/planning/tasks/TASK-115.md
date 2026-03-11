---
id: TASK-115
title: "Design SQLite schema"
description: "Designed all SQLite tables, columns, indexes, and foreign key constraints for conversation persistence."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-029
depends-on: []
scope:
  - Define tables for sessions, messages, tool calls, metrics
  - Define column types, constraints, and defaults
  - Design indexes for common query patterns
  - Define foreign key relationships and cascade rules
acceptance:
  - Schema covers all persistence needs for conversations
  - Indexes support the IPC command query patterns
  - Schema is documented with migration strategy
---
## What

Designed the SQLite schema for conversation persistence covering sessions, messages, tool calls, and metrics tables with indexes and foreign key constraints.

## How

Defined each table's columns with types, NOT NULL constraints, and defaults, added indexes on foreign keys and frequently queried columns, and documented cascade delete rules and the migration strategy.

## Verification

Schema design covers all conversation persistence needs, indexes align with IPC command access patterns, and the migration approach is documented.
