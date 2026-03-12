---
id: TASK-207
title: Integration testing for all enforcement layers
description: |
  End-to-end verification that all four enforcement layers work together in
  both CLI (plugin) and app (Rust) contexts.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-196
  - TASK-197
  - TASK-198
  - TASK-199
  - TASK-200
  - TASK-201
  - TASK-203
  - TASK-206
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
