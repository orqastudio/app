---
id: TASK-7c725cf8
type: task
title: "LSP server — hex ID validation, skill doc constraint, and ID generation"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-d1d42012
    type: delivers
  - target: TASK-4dc4cc3f
    type: depends-on
  - target: TASK-37bafa1c
    type: depends-on
  - target: TASK-f45e6ede
    type: depends-on
---

# TASK-7c725cf8: LSP Server — ID and Skill Doc Validation

## Acceptance Criteria

1. LSP diagnoses invalid ID format (not `TYPE-XXXXXXXX` hex) as warning (error after migration)
2. LSP diagnoses skills missing `synchronised-with` relationship as error
3. LSP diagnoses ID type prefix mismatch (e.g. TASK prefix on a skill file) as error
4. LSP offers code action to generate a valid hex ID for new artifacts
5. LSP diagnoses duplicate IDs across the graph as error
6. All diagnostics appear in real-time as the user edits `.orqa/` files
