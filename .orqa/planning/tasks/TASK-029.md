---
id: TASK-029
title: Provider-neutral session ID naming
description: Renames the provider-specific session ID field to a neutral name across all layers — sidecar, Rust types, commands, domain, repository, and SQLite — with no behavioral changes.
status: done
created: 2026-03-07
updated: 2026-03-09
epic: EPIC-040
assignee: backend-engineer
skills:
  - orqa-ipc-patterns
  - orqa-repository-pattern
scope:
  - sidecar/src/protocol.ts
  - sidecar/src/index.ts
  - src-tauri/src/sidecar/types.rs
  - src-tauri/src/commands/stream_commands.rs
  - src-tauri/src/domain/session.rs
  - src-tauri/src/repo/session_repo.rs
  - src-tauri/src/db.rs
acceptance:
  - All sdk_session_id references renamed to provider_session_id
  - SQLite migration 005 renames column
  - No behavioral changes
---
## What

Rename the provider-specific `sdk_session_id` field to the neutral
`provider_session_id` across all layers: sidecar TypeScript, Rust types,
commands, domain, repository, and SQLite (migration 005).

## Outcome

Atomic rename across 13+ files including database migration. Zero behavioral
changes, all tests pass. Git commit: `fa8ecc7`.
