---
id: EPIC-007
title: Composability Refactoring
description: Refactor monolithic service files into composable units while preserving existing store, IPC, and component boundaries.
status: draft
priority: P2
created: "2026-03-07"
updated: "2026-03-07"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-020
docs-required:
  - DOC-010
  - DOC-014
docs-produced:
  - DOC-010
  - DOC-014
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

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
