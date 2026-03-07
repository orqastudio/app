---
id: EPIC-007
title: "Composability Refactoring"
status: draft
priority: P2
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: composability-gate
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 3
  impact: 3
  dependency: 2
  effort: 4
score: 6.0
roadmap-ref: "D7"
docs-required:
  - .orqa/plans/composability-gate.md
  - docs/architecture/rust-modules.md
  - docs/architecture/streaming-pipeline.md
docs-produced:
  - docs/architecture/rust-modules.md (update with refactored module structure)
  - docs/architecture/streaming-pipeline.md (update with StreamOrchestrator)
tags: [composability, refactoring, code-health]
---

# Composability Refactoring

The codebase has solid boundaries (stores, IPC, components) but monolithic service files that reduce maintainability.

## Why P2

Code health for sustained velocity. Functions individually pass size limits, but file-level cohesion is poor. This is about practicing what we preach — composability is a platform principle.

## Related Plan

See `.orqa/plans/composability-gate.md` for the detailed refactoring plan.

## Tasks

- [ ] Extract `StreamOrchestrator` from `stream_commands.rs` (2,232 lines -> command handlers + orchestrator service)
- [ ] Implement `Tool` trait and `ToolRegistry` from `tool_executor.rs` (966 lines -> pluggable tools)
- [ ] Decompose `ConversationView.svelte` (367 lines) into smaller composable units if complexity grows
- [ ] Service layer for enforcement, governance, and scanning (wrap procedural code in structs)
