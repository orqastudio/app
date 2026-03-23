---
id: "TASK-5da55ccb"
type: task
title: "Create data integrity rule (RULE-130f1f63)"
description: "Codify data integrity requirements: all cross-references must resolve, pipeline relationships must have bidirectional inverses, pre-commit enforces both, make verify is the manual full-scan."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "RULE-130f1f63 exists in .orqa/process/rules/"
  - "Rule covers: link resolution, bidirectional inverses, pre-commit enforcement, make verify"
  - "Rule has active status and appropriate relationships"
rule-overrides: []
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8954343c"
    type: "depends-on"
  - target: "TASK-c4a7b6bb"
    type: "depends-on"
  - target: "TASK-a86c3565"
    type: "depended-on-by"
  - target: "TASK-b6b5c31c"
    type: "depended-on-by"
  - target: "TASK-f6e9b767"
    type: "depended-on-by"
  - target: "TASK-6e4fd8b9"
    type: "depended-on-by"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Create RULE-130f1f63 codifying the data integrity requirements established in Phase 0.

## How

1. Create `.orqa/process/rules/RULE-130f1f63.md` with frontmatter and body
2. Cover all integrity requirements: link resolution, bidirectional inverses, enforcement mechanisms
3. Reference related rules (RULE-a764b2ae, RULE-2f7b6a31, RULE-633e636d)

## Verification

- RULE-130f1f63 exists and passes schema validation
- Rule content accurately reflects the enforcement implemented in [TASK-e7324438](TASK-e7324438)/282