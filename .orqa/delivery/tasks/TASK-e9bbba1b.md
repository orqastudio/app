---
id: TASK-e9bbba1b
title: Provider-neutral session ID naming
description: "Renames the provider-specific session ID field to a neutral name across all layers — sidecar, Rust types, commands, domain, repository, and SQLite — with no behavioral changes."
status: completed
created: 2026-03-07
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - All sdk_session_id references renamed to provider_session_id
  - SQLite migration 005 renames column
  - No behavioral changes
relationships:
  - target: EPIC-ee688e85
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-300e666c
    type: depended-on-by
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
