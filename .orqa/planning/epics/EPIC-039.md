---
id: EPIC-039
title: Composability Refactoring
description: Major refactoring sprint establishing the domain service extraction pattern. Decomposed monolithic command files, decoupled stores, extracted utilities, added semantic design tokens, and established the thin-command → domain service → repository pattern.
status: done
priority: P1
created: 2026-03-06
updated: 2026-03-09
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-020
  - RES-021
docs-required: []
docs-produced:
  - .orqa/governance/decisions/AD-026.md
scoring:
  user-value: 4
  pillar-alignment: 5
  dependency-weight: 5
  effort: 5
  risk: 3
  score: 22
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

AD-026 (Domain Service Extraction Pattern)

## Git Evidence

- `7fd306e` through `d0fa094` — Full refactoring series (2026-03-06 to 2026-03-07)
