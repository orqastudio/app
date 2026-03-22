---
id: TASK-4eb0c231
title: Backfill skills with category and relationships
description: Use backfill tooling to add category (methodology/domain/tool) and grounded relationships to all 48 skills.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: null
docs: []
acceptance:
  - "All 48 skills have a category field (methodology, domain, or tool)"
  - All 48 skills have a relationships array
  - "Each skill has at least one grounded relationship (to a pillar for methodology, to a decision for domain/tool)"
  - Null targets have rationale and intended field
  - Human reviewed and approved all proposals
rule-overrides:
  - "rule: RULE-a764b2ae"
relationships:
  - target: EPIC-ca7b398b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-fedfd82a
    type: depends-on
  - target: TASK-da2965a2
    type: depended-on-by
  - target: TASK-508cf6cd
    type: depended-on-by
---

## What

Second backfill batch. Skills get a `category` field and `grounded` relationships. Methodology skills connect to pillars. Domain and tool skills connect to decisions.

## How

1. Run backfill tool against all skills
2. Tool proposes category classification and grounded connections
3. Review — category determines what the grounded target should be (PILLAR vs AD)
4. Approve, reject, or edit
5. Commit the batch

## Verification

- All 48 skills have `category` and `relationships` in frontmatter
- Methodology skills point to PILLAR-NNN, domain/tool skills point to AD-NNN
- Spot-check 5 skills for correct classification and connections
