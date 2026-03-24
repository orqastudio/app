---
id: TASK-f6b7c8d9
type: task
title: "Recurring lessons audit and promotion"
description: "Audit all IMPL entries in .orqa/process/lessons/ for recurrence >= 2. Promote eligible lessons to rules or knowledge updates per the learning loop."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "All lesson files in .orqa/process/lessons/ scanned for recurrence count"
  - "Lessons with recurrence >= 2 identified and listed"
  - "Each eligible lesson either promoted to a rule/knowledge update or documented as not yet promotable with rationale"
  - "Promoted lessons have their promoted-to field updated"
  - "New rules created from promotions have enforcement entries defined"
relationships:
  - target: EPIC-b2f0399e
    type: delivers
---

## What

Exercise the learning loop per RULE-551bde31 (lessons-learned) and RULE-c4fe67a2 (governance-debt-priority). Lessons with recurrence >= 2 are overdue for promotion — this task ensures no systemic patterns remain unaddressed.

## How

1. Read all `IMPL-*.md` files in `.orqa/process/lessons/`
2. Check each file's frontmatter for `recurrence` count
3. For each lesson with recurrence >= 2:
   - Determine promotion target: rule, knowledge update, or coding standard addition
   - Create the target artifact with enforcement entries
   - Update the lesson's `promoted-to` field
   - Update the lesson's `status` to `promoted`
4. For lessons with recurrence < 2, verify they are correctly categorised and tracked
5. Check for patterns across lessons — multiple lessons about the same topic may indicate a systemic issue

## Verification

1. Zero lessons remain with recurrence >= 2 and status `active` or `recurring`
2. Every promoted lesson has a valid `promoted-to` reference
3. Every new rule from promotion has at least one enforcement entry
4. `graph_validate()` passes after all changes
