---
id: EPIC-017
title: "Enforcement & Continuity"
description: "Add real-time violation detection during streaming, hook-based rule injection, compliance dashboard, and session handoff continuity."
status: completed
priority: P2
created: 2026-03-07
updated: 2026-03-12
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 4
  dependencies: 3
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-314
    type: delivered-by
    rationale: Epic contains this task
  - target: MS-001
    type: delivers
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-001
    type: grounded-by
---
**Note:** Two deliverables from this epic were deferred:
- **Visual compliance dashboard** -- not delivered; should be tracked in a future epic when dashboard UI work is prioritised.
- **Session handoff and continuity** -- not delivered; depends on SDK session resume capabilities. Should be tracked in a separate epic.

The enforcement portions (hooks, real-time violation detection) were completed via [EPIC-050](EPIC-050) and [EPIC-052](EPIC-052).

## Tasks

- [x] Hooks that inject relevant rules into conversations based on file context — completed via [EPIC-050](EPIC-050) (companion plugin)
- [x] Real-time violation detection during streaming — completed via enforcement engine in `stream_commands.rs`
- [ ] Visual compliance dashboard — deferred to future epic
- [ ] Session handoff and continuity — deferred to future epic (SDK session resume)

## Context

Superseded by [EPIC-050](EPIC-050) (Rule Enforcement Engine) and [EPIC-052](EPIC-052) (Structured Thinking Enforcement) for the enforcement portions. The session handoff/continuity features remain valid future work but should be tracked in a separate epic.

## Implementation Design

Enforcement: completed via [EPIC-050](EPIC-050) and [EPIC-052](EPIC-052).
Continuity: requires separate epic for SDK session resume and cross-session search.
