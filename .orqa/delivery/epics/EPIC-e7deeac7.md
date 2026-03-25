---
id: "EPIC-e7deeac7"
type: epic
title: "Codebase Test Coverage"
description: "Achieve comprehensive test coverage across the entire codebase — frontend (Vitest), backend (cargo test + tarpaulin), and IPC contract verification. Close the frontend test gap (zero tests vs 465 Rust tests) and establish coverage measurement and enforcement."
status: "captured"
priority: "P2"
created: "2026-03-07"
updated: "2026-03-13"
horizon: "next"
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Why P2

465 Rust tests exist but zero frontend tests. Changes to stores break components silently. Backend coverage is unmeasured — we have tests but no coverage metrics. This is a learning gap — without tests and coverage tracking, regression patterns can't be detected and the integrity engine can't enforce quality gates.

## Tasks

### Frontend (zero tests → baseline)

- [ ] Vitest setup for Svelte component and store testing
- [ ] Store unit tests (conversation, session, project, settings — state transitions, reactive updates)
- [ ] Component tests for critical UI (ConversationView, ToolApprovalDialog, SessionDropdown)
- [ ] IPC contract tests — verify invoke calls match actual Tauri commands

### Backend (465 tests → measured coverage)

- [ ] Set up cargo tarpaulin for coverage measurement
- [ ] Identify and fill coverage gaps in untested command modules (per EPIC-2bf6887a findings)
- [ ] Add integration tests for cross-module flows (artifact graph, search, streaming)

### Coverage infrastructure

- [ ] Coverage reporting in CI (tarpaulin JSON + Vitest JSON output)
- [ ] Coverage threshold enforcement in pre-commit or make targets
- [ ] Document coverage targets and measurement approach

## Implementation Design

Implementation approach to be defined during planning. See [IDEA-98c50cf5](IDEA-98c50cf5) for the longer-term vision of coverage enforcement as a plugin/integrity check.