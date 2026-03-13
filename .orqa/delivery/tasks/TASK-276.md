---
id: TASK-276
title: "Backfill decisions with relationships"
description: "Use backfill tooling to add practices and enforces relationships to all 42 decisions, connecting to skills and rules already backfilled."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-058
depends-on: [TASK-275]
assignee: null
docs: []
skills: []
acceptance:
  - "All 42 decisions have a relationships array"
  - "Each decision has practices and enforces relationships (nullable with rationale)"
  - "Connections reference skills and rules already backfilled in TASK-274/TASK-275"
  - "Bidirectional consistency — if AD-029 says practices:SKILL-X, SKILL-X says grounded:AD-029"
  - "Human reviewed and approved all proposals"
rule-overrides:
  - rule: RULE-032
    reason: "Adding new optional relationships field to decision frontmatter during migration"
---

## What

Third backfill batch. Decisions are the Principle stage — they connect downstream to skills (Practice) and rules (Enforcement). Since rules and skills are already backfilled, the tool can cross-reference for bidirectional consistency.

## How

1. Run backfill tool against all decisions
2. Tool proposes practices (skills) and enforces (rules) connections, cross-referencing already-backfilled artifacts
3. Verify bidirectional consistency — if a decision points to a skill, that skill should already point back
4. Approve, reject, or edit
5. Commit the batch

## Verification

- All 42 decisions have `relationships` in frontmatter
- Bidirectional consistency check passes (sample 5 decisions, verify both directions)
- Null targets have rationale and intended field
