---
id: "EPIC-897bbe8f"
type: epic
title: "Composability Refactoring"
description: "Major refactoring sprint establishing the domain service extraction pattern. Decomposed monolithic command files, decoupled stores, extracted utilities, added semantic design tokens, and established the thin-command → domain service → repository pattern."
status: "completed"
priority: "P1"
created: "2026-03-06"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: "RES-45cffcbe"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-45cffcbe"
  - target: "RES-b5d6ae6e"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-b5d6ae6e"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-320eb399"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8b482a0a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c09db10b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-570f27e0"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-fca01488"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "AD-6cd1ff6f"
    type: "driven-by"
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

[AD-6cd1ff6f](AD-6cd1ff6f) (Domain Service Extraction Pattern)

## Git Evidence

- `7fd306e` through `d0fa094` — Full refactoring series (2026-03-06 to 2026-03-07)

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.