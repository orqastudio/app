---
id: EPIC-017
title: Enforcement & Continuity
description: Add real-time violation detection during streaming, hook-based rule injection, compliance dashboard, and session handoff continuity.
status: done
priority: P2
created: "2026-03-07"
updated: "2026-03-12"
milestone: MS-002
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on: []
blocks: []
docs-required:
  - DOC-002
  - DOC-014
docs-produced:
  - DOC-002
scoring:
  pillar: 5
  impact: 3
  dependency: 2
  effort: 4
  score: 6.8
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
