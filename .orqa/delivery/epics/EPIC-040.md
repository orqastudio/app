---
id: EPIC-040
title: Provider Abstraction Layer
description: "Refactor sidecar from Claude-specific to provider-agnostic architecture. Provider interface, Claude adapter, sdk_session_id → provider_session_id rename across 13+ files including SQLite migration."
status: completed
priority: P1
created: 2026-03-07
updated: 2026-03-09
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 4
  dependencies: 3
relationships:
  - target: RES-009
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-009
  - target: RES-027
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-027
  - target: MS-001
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-028
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-029
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-330
    type: delivered-by
    rationale: Epic contains this task
  - target: AD-025
    type: driven-by
  - target: IDEA-001
    type: realised-by
  - target: RES-009
    type: guided-by
  - target: RES-027
    type: guided-by
  - target: AD-025
    type: driven-by
---
## Implementation Design

### Sidecar Changes
- `provider-interface.ts` — Provider interface defining `query()`, `resume()`, `cancelStream()`, `healthCheck()`
- `providers/claude-agent.ts` — ClaudeAgentProvider (first concrete implementation)
- `providers/index.ts` — `createProvider()` factory
- `provider.ts` — Thin facade delegating to default provider

### Cross-Stack Rename
- `sdk_session_id` → `provider_session_id` across:
  - Rust types (SidecarRequest, SidecarResponse)
  - TypeScript protocol types
  - SQLite column (migration 005)
  - All callers (13+ files)

### Neutral Protocol
- NDJSON protocol carries `ProviderEvent` types, not Claude-specific types
- Rust backend is provider-agnostic
- Claude-specific concepts encapsulated in ClaudeAgentProvider

## Produced Decision

[AD-025](AD-025) (Provider-Agnostic AI Integration)

## Git Evidence

- `fa8ecc7` — Pluggable sidecar, sdk→provider rename
- `34cc4b6` — Provider interface, Claude adapter

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
