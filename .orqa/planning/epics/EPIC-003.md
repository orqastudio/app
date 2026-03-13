---
id: EPIC-003
title: Context Injection on Failed Resume
description: Add fallback context injection when SDK session resume fails due to app restart or cleared storage.
status: draft
priority: P1
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required: []
docs-produced: []
scoring:
  pillar: 4
  impact: 5
  dependency: 3
  effort: 3
  score: 11
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
