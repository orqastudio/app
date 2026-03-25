---
id: TASK-492dc3a0
type: task
title: Decompose stream commands into domain modules
description: "Extracts business logic from the monolithic stream_commands.rs (2,425 lines) into four focused domain modules, leaving the command file as thin orchestration only."
status: completed
created: 2026-03-06
updated: 2026-03-09
assignee: AGENT-e5dd38e4
acceptance:
  - stream_commands.rs reduced from 2
  - 425 to ~280 lines
  - All business logic in domain modules
  - 385 tests pass
  - zero clippy warnings
relationships:
  - target: EPIC-c1833545
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-8058601a
    type: depended-on-by
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