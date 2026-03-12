---
id: TASK-274
title: "Backfill rules with relationships"
description: "Use backfill tooling to add grounded relationships to all 44 rules, connecting each to the decision or pillar it enforces."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-058
depends-on: [TASK-273]
assignee: null
docs: []
skills: []
acceptance:
  - "All 44 rules have a relationships array"
  - "Each rule has at least one grounded relationship (to a decision or pillar)"
  - "Null targets have rationale and intended field"
  - "All relationships have rationale explaining why the connection exists"
  - "Human reviewed and approved all proposals"
rule-overrides:
  - rule: RULE-032
    reason: "Adding new optional relationships field to rule frontmatter during migration"
  - rule: RULE-004
    reason: "Rule frontmatter is being extended with new fields"
---

## What

First backfill batch — rules are the enforcement layer and the most impactful to connect. Each rule gets a `grounded` relationship pointing to the decision or pillar it serves.

## How

1. Run backfill tool against all rules
2. Review proposals — for each rule, the tool proposes which decision/pillar it's grounded in
3. Approve, reject, or edit each proposal
4. Commit the batch

## Verification

- All 44 rules have `relationships` array in frontmatter
- No rule has an empty relationships array (at minimum grounded, even if null with rationale)
- Spot-check 5 rules for correct connections
