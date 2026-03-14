---
id: TASK-416
title: Unify process gates and enforcement engine evaluation
description: "The Rust backend has two separate enforcement systems: process gates (workflow state tracking) and the enforcement engine (pattern matching). Unify them so enforcement entries can reference workflow state and process gates can evaluate enforcement entries."
status: done
priority: P3
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-064
depends-on:
  - TASK-415
assignee: null
skills:
  - SKILL-032
  - SKILL-009
  - SKILL-026
acceptance:
  - Enforcement entries can declare workflow state conditions (e.g., 'fire only if code was written this session')
  - Process gates are expressed as enforcement entries on rules, not hardcoded in process_gates.rs
  - "Single evaluation pipeline: context + workflow state → enforcement entries → verdicts"
  - Existing behavior preserved — all 5 current process gates still fire at the same conditions
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 2 — unifies the two enforcement systems in the Rust backend
  - target: EPIC-064
    type: belongs-to
    rationale: Task belongs to this epic
---

## Scope

### Current State

Two systems:
- `process_gates.rs`: 5 hardcoded gates with WorkflowTracker state conditions
- `enforcement_engine.rs`: Pattern matching against file/bash content

### Target State

One system:
- Enforcement entries on rules can declare `conditions` that reference workflow state
- Process gates become enforcement entries on the rules they enforce
- WorkflowTracker state is available to the enforcement engine as evaluation context
- `process_gates.rs` becomes a thin wrapper that translates workflow state to enforcement context

### Migration

1. Express each current process gate as an enforcement entry on the appropriate rule
2. Add `conditions` field to enforcement entry schema (e.g., `has_written_code: true`)
3. Enforcement engine evaluates conditions against WorkflowTracker state
4. Remove hardcoded gate logic from process_gates.rs
5. Verify all 5 gates still fire at the correct conditions
