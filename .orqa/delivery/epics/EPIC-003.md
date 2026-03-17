---
id: EPIC-003
title: Context Injection on Failed Resume
description: Add fallback context injection when SDK session resume fails due to app restart or cleared storage.
status: captured
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 2
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
---
## Why P1

Can't restart the app during development without losing conversation context. Every Rust change requires a restart, so this directly blocks dogfooding workflow.

## Tasks

- [ ] Detect SDK resume failure in sidecar (returned session_id !== passed session_id)
- [ ] Sidecar emits `context_needed` event to Rust
- [ ] Rust loads last ~20 text messages from SQLite for the session
- [ ] Rust sends `context_history` to sidecar for injection
- [ ] Rust emits `ContextInjected` event for transparency [EPIC-001](EPIC-001)

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
