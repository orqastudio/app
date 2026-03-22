---
id: TASK-78abb39a
title: "Verify pipeline integrity — all artifacts connected, no orphans"
description: "Final verification that the migration is complete: every governance artifact has relationships, no orphan nodes, bidirectional consistency holds, pipeline flow is forward."
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: null
docs: []
acceptance:
  - "Every governance artifact (lesson, decision, rule, skill) has a non-empty relationships array"
  - No orphan nodes — every artifact is reachable from at least one other artifact
  - "Bidirectional consistency — if A says grounded:B, B says grounded-by:A"
  - "No unintended null targets without intended:true"
  - Pipeline flow analysis — no backwards flow (enforcement without upstream observation)
  - Deprecated fields completely removed
rule-overrides: []
relationships:
  - target: EPIC-ca7b398b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-7b2f5ee7
    type: depends-on
  - target: TASK-508cf6cd
    type: depended-on-by
---

## What

Final integrity check. Walk the entire graph and verify it's consistent, complete, and forward-flowing.

## How

1. Script or tool that reads all governance artifacts
2. Check: every artifact has relationships array with at least one entry
3. Check: bidirectional consistency across all relationship pairs
4. Check: null targets all have intended:true (no unresolved tensions left from migration)
5. Check: no deprecated fields remain anywhere
6. Report any issues found

## Verification

- Integrity check script runs clean with zero issues
- Pipeline flow report shows forward flow (observations → principles → practices → enforcement)
- Summary report committed as evidence
