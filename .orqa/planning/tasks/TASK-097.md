---
id: TASK-097
title: "SQLite persistence design"
description: "Evaluated SQLite via rusqlite for structured local storage of conversation data, sessions, and messages."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-025
depends-on: []
scope:
  - Evaluate rusqlite vs sqlx for SQLite access from Rust
  - Design persistence scope (conversations only, not governance)
  - Assess migration strategy for schema evolution
acceptance:
  - SQLite selected for conversation persistence with documented rationale
  - Scope boundary established
  - rusqlite selected as the access library
---
## What

Evaluated SQLite access libraries and established the persistence scope boundary: SQLite via rusqlite for conversation data (sessions, messages, metrics) only, with governance data remaining file-based.

## How

Compared rusqlite and sqlx for ergonomics and async compatibility in a Tauri context, then defined the data ownership boundary that became AD-032.

## Verification

The persistence scope decision was recorded and the rusqlite-based persistence layer was implemented in accordance with this design.
