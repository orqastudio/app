---
id: TASK-207
title: Integration testing for all enforcement layers
description: |
  End-to-end verification that all four enforcement layers work together in
  both CLI (plugin) and app (Rust) contexts.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - Process gates fire at correct moments in both CLI and app contexts
  - Skill injection works for path-based triggers with deduplication
  - Linter delegation documented and functional via make check
  - All new rules validate against schema
  - No regression in existing enforcement
relationships:
  - target: EPIC-052
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-196
    type: depends-on
  - target: TASK-197
    type: depends-on
  - target: TASK-198
    type: depends-on
  - target: TASK-199
    type: depends-on
  - target: TASK-200
    type: depends-on
  - target: TASK-201
    type: depends-on
  - target: TASK-203
    type: depends-on
  - target: TASK-206
    type: depends-on
  - target: TASK-342
    type: depended-on-by
---


## What

Verify all enforcement layers work end-to-end:
1. Process gates fire at correct moments
2. Skill injection works for path-based triggers
3. Linter delegation is documented and functional
4. All new rules validate against schema

## How

1. Gate test: Write to `backend/src-tauri/` with no prior reads → understand-first warning
2. Gate test: Read docs first, then write → no warning
3. Injection test: Edit domain file → skills injected as systemMessage
4. Injection test: Second edit same area → skills NOT re-injected
5. Linter test: `make check` catches all documented standards
6. Schema test: All rules (including [RULE-041](RULE-041)/042/043) validate

## Verification

- All test scenarios pass
- Every rule has clear enforcement status (gate, inject, lint, or "agent discipline")
- No regression in existing enforcement
