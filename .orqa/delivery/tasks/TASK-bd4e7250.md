---
id: TASK-bd4e7250
type: task
title: "LSP server — hex ID validation, skill doc constraint, and ID generation"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-db0e3e3c
    type: depends-on
  - target: TASK-2eeb847c
    type: depends-on
  - target: TASK-6675ad7c
    type: depends-on
---

# TASK-bd4e7250: LSP Server — ID and Skill Doc Validation

## Acceptance Criteria

1. LSP diagnoses invalid ID format (not `TYPE-XXXXXXXX` hex) as warning (error after migration)
2. LSP diagnoses skills missing `synchronised-with` relationship as error
3. LSP diagnoses ID type prefix mismatch (e.g. TASK prefix on a skill file) as error
4. LSP offers code action to generate a valid hex ID for new artifacts
5. LSP diagnoses duplicate IDs across the graph as error
6. All diagnostics appear in real-time as the user edits `.orqa/` files
