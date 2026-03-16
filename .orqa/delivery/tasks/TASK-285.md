---
id: TASK-285
title: Create data integrity rule (RULE-045)
description: "Codify data integrity requirements: all cross-references must resolve, pipeline relationships must have bidirectional inverses, pre-commit enforces both, make verify is the manual full-scan."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - RULE-045 exists in .orqa/process/rules/
  - "Rule covers: link resolution, bidirectional inverses, pre-commit enforcement, make verify"
  - Rule has active status and appropriate relationships
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-283
    type: depends-on
  - target: TASK-284
    type: depends-on
  - target: TASK-293
    type: depended-on-by
  - target: TASK-294
    type: depended-on-by
  - target: TASK-296
    type: depended-on-by
  - target: TASK-297
    type: depended-on-by
  - target: TASK-349
    type: depended-on-by
---

## What

Create [RULE-045](RULE-045) codifying the data integrity requirements established in Phase 0.

## How

1. Create `.orqa/process/rules/[RULE-045](RULE-045).md` with frontmatter and body
2. Cover all integrity requirements: link resolution, bidirectional inverses, enforcement mechanisms
3. Reference related rules (RULE-032, [RULE-034](RULE-034), RULE-013)

## Verification

- [RULE-045](RULE-045) exists and passes schema validation
- Rule content accurately reflects the enforcement implemented in [TASK-281](TASK-281)/282
