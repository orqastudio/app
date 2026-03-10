---
id: EPIC-007
title: Composability Refactoring
description: Refactor monolithic service files into composable units while preserving existing store, IPC, and component boundaries.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-020
docs-required:
  - .orqa/plans/composability-gate.md
  - docs/architecture/rust-modules.md
  - docs/architecture/streaming-pipeline.md
docs-produced:
  - docs/architecture/rust-modules.md (update with refactored module structure)
  - docs/architecture/streaming-pipeline.md (update with StreamOrchestrator)
scoring:
  pillar: 3
  impact: 3
  dependency: 2
  effort: 4
  score: 6
---
## Why P2

Code health for sustained velocity. Functions individually pass size limits, but file-level cohesion is poor. This is about practicing what we preach — composability is a platform principle.

## Related Plan

See `.orqa/plans/composability-gate.md` for the detailed refactoring plan.

## Tasks

- [ ] Extract `StreamOrchestrator` from `stream_commands.rs` (2,232 lines -> command handlers + orchestrator service)
- [ ] Implement `Tool` trait and `ToolRegistry` from `tool_executor.rs` (966 lines -> pluggable tools)
- [ ] Decompose `ConversationView.svelte` (367 lines) into smaller composable units if complexity grows
- [ ] Service layer for enforcement, governance, and scanning (wrap procedural code in structs)
