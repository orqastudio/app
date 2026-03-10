---
id: TASK-024
title: Decompose stream commands into domain modules
description: Extracts business logic from the monolithic stream_commands.rs (2,425 lines) into four focused domain modules, leaving the command file as thin orchestration only.
status: done
created: "2026-03-06"
updated: "2026-03-09"
epic: EPIC-039
assignee: AGENT-002
skills:
  - SKILL-009
  - SKILL-008
scope:
  - src-tauri/src/commands/stream_commands.rs
  - src-tauri/src/domain/tool_executor.rs
  - src-tauri/src/domain/system_prompt.rs
  - src-tauri/src/domain/stream_loop.rs
  - src-tauri/src/domain/session_title.rs
acceptance:
  - stream_commands.rs reduced from 2
  - 425 to ~280 lines
  - All business logic in domain modules
  - 385 tests pass
  - zero clippy warnings
---
## What

Extract business logic from the monolithic stream_commands.rs (2,425 lines) into
four focused domain modules.

## Outcome

Extracted to `tool_executor.rs` (~700 lines), `system_prompt.rs` (~150 lines),
`stream_loop.rs` (~350 lines), `session_title.rs` (~85 lines). Command file is
now thin orchestration only. Git commit: `7fd306e`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
