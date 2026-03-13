---
id: TASK-285
title: "Create data integrity rule (RULE-045)"
description: "Codify data integrity requirements: all cross-references must resolve, pipeline relationships must have bidirectional inverses, pre-commit enforces both, make verify is the manual full-scan."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-283, TASK-284]
assignee: null
docs: []
skills: []
acceptance:
  - "RULE-045 exists in .orqa/governance/rules/"
  - "Rule covers: link resolution, bidirectional inverses, pre-commit enforcement, make verify"
  - "Rule has active status and appropriate relationships"
rule-overrides: []
---

## What

Create RULE-045 codifying the data integrity requirements established in Phase 0.

## How

1. Create `.orqa/governance/rules/RULE-045.md` with frontmatter and body
2. Cover all integrity requirements: link resolution, bidirectional inverses, enforcement mechanisms
3. Reference related rules (RULE-032, RULE-034, RULE-013)

## Verification

- RULE-045 exists and passes schema validation
- Rule content accurately reflects the enforcement implemented in TASK-281/282
