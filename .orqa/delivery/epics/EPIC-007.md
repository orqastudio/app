---
id: EPIC-007
title: Composability Refactoring
description: "Refactor monolithic service files into composable units while preserving existing store, IPC, and component boundaries."
status: completed
priority: P2
created: 2026-03-07
updated: 2026-03-12
horizon: null
scoring: null
relationships:
  - target: RES-020
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-020
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-313
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
---
**Note:** This epic was superseded by [EPIC-039](EPIC-039), which completed the composability refactoring. The status remains `done` because the work was delivered through [EPIC-039](EPIC-039). The tasks listed below were not completed under this epic directly -- they were subsumed by [EPIC-039](EPIC-039)'s broader scope.

## Why P2

Code health for sustained velocity. Functions individually pass size limits, but file-level cohesion is poor. This is about practicing what we preach — composability is a platform principle.

## Tasks

- [ ] Extract `StreamOrchestrator` from `stream_commands.rs` (2,232 lines -> command handlers + orchestrator service)
- [ ] Implement `Tool` trait and `ToolRegistry` from `tool_executor.rs` (966 lines -> pluggable tools)
- [ ] Decompose `ConversationView.svelte` (367 lines) into smaller composable units if complexity grows
- [ ] Service layer for enforcement, governance, and scanning (wrap procedural code in structs)

## Context

Superseded by [EPIC-039](EPIC-039) which completed the composability refactoring — domain service extraction pattern established, `StreamOrchestrator` decomposed, `Tool` trait implemented.

## Implementation Design

Completed via [EPIC-039](EPIC-039).
