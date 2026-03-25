---
id: "TASK-9fed41e8"
type: "task"
title: "Provider-neutral session ID naming"
description: "Renames the provider-specific session ID field to a neutral name across all layers — sidecar, Rust types, commands, domain, repository, and SQLite — with no behavioral changes."
status: "completed"
created: 2026-03-07T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
assignee: "AGENT-e5dd38e4"
acceptance:
  - "All sdk_session_id references renamed to provider_session_id"
  - "SQLite migration 005 renames column"
  - "No behavioral changes"
relationships:
  - target: "EPIC-2f1648f5"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

Rename the provider-specific `sdk_session_id` field to the neutral
`provider_session_id` across all layers: sidecar TypeScript, Rust types,
commands, domain, repository, and SQLite (migration 005).

## Outcome

Atomic rename across 13+ files including database migration. Zero behavioral
changes, all tests pass. Git commit: `fa8ecc7`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.