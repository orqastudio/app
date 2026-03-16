---
id: EPIC-039
title: Composability Refactoring
description: "Major refactoring sprint establishing the domain service extraction pattern. Decomposed monolithic command files, decoupled stores, extracted utilities, added semantic design tokens, and established the thin-command → domain service → repository pattern."
status: completed
priority: P1
created: 2026-03-06
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: RES-020
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-020
  - target: RES-021
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-021
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-024
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-025
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-026
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-027
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-329
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: AD-026
    type: informed-by
  - target: DOC-059
    type: informs
---
## Implementation Design

### Backend Extractions
- `stream_commands.rs` (2,425 lines) → 4 domain modules
- `setup_commands.rs` → `domain::setup`
- `governance_commands.rs` → `domain::governance_analysis`
- `artifact_commands.rs` → `domain::artifact_reader` + repo modules
- Project settings I/O → repo layer
- Timestamp utils → `domain::time_utils`
- Path constants → `domain::paths`
- Search commands → OrqaError (not String)

### Frontend Extractions
- SettingsView → focused sub-components
- Toolbar → focused sub-components
- Conversation store decoupled from session store
- Missing error/loading states added

### Cross-Cutting
- 60+ hardcoded colors → semantic design tokens
- Cross-platform make targets
- Security hardening (CSP, capability restrictions)

## Produced Decision

[AD-026](AD-026) (Domain Service Extraction Pattern)

## Git Evidence

- `7fd306e` through `d0fa094` — Full refactoring series (2026-03-06 to 2026-03-07)

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
