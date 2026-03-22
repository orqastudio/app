---
id: TASK-8f8b1dba
type: task
title: "Create documentation artifacts for all existing skills"
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-2eeb847c
    type: depends-on
---

# TASK-8f8b1dba: Create Skill Documentation Artifacts

## Acceptance Criteria

1. Core platform skills documented — grouped by domain (planning, governance, search, etc.)
2. Each plugin's skills documented — one doc per plugin with a reference table
3. Project-level skills documented — grouped by domain (architecture, frontend, backend, etc.)
4. Every skill has at least one `synchronised-with` relationship to a doc
5. Every doc has the inverse `synchronised-with` back to its skills
6. `orqa validate` passes with 0 errors for skill documentation constraint
