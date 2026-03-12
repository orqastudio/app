---
id: TASK-277
title: "Backfill lessons with maturity and relationships"
description: "Use backfill tooling to add maturity (observation/understanding) and grounded relationships to all 16 lessons."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-058
depends-on: [TASK-276]
assignee: null
docs: []
skills: []
acceptance:
  - "All 16 lessons have a maturity field (observation or understanding)"
  - "All 16 lessons have a relationships array"
  - "Lessons with maturity=understanding have at least one grounded relationship"
  - "Lessons with maturity=observation may have empty relationships (or informs)"
  - "Human reviewed and approved all proposals"
rule-overrides:
  - rule: RULE-032
    reason: "Adding new optional fields to lesson frontmatter during migration"
---

## What

Final backfill batch. Lessons are the smallest set (16). Each gets a `maturity` classification and relationships. Understanding-stage lessons must connect to a principle (decision).

## How

1. Run backfill tool against all lessons
2. Tool proposes maturity classification based on content (does it identify root cause = understanding, or just describe what happened = observation?)
3. For understanding-stage lessons, propose grounded connections to decisions
4. Approve, reject, or edit
5. Commit the batch

## Verification

- All 16 lessons have `maturity` and `relationships` in frontmatter
- No understanding-stage lesson lacks a grounded relationship
- Spot-check all 16 (small enough set to verify completely)
