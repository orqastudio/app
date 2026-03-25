---
id: "TASK-30ca6f82"
type: "task"
title: "Backfill decisions with relationships"
description: "Use backfill tooling to add practices and enforces relationships to all 42 decisions, connecting to skills and rules already backfilled."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
assignee: null
docs: []
acceptance:
  - "All 42 decisions have a relationships array"
  - "Each decision has practices and enforces relationships (nullable with rationale)"
  - "Connections reference skills and rules already backfilled in TASK-eb558448/TASK-4b57032b"
  - "Bidirectional consistency — if AD-48b310f9 says practices:KNOW-X, KNOW-X says grounded:AD-48b310f9"
  - "Human reviewed and approved all proposals"
rule-overrides:
  - "rule: RULE-23699df2"
relationships:
  - target: "EPIC-3e6cad90"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-4b57032b"
    type: "depends-on"
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