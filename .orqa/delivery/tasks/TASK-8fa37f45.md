---
id: TASK-8fa37f45
type: task
title: Audit integrity checking system for coverage gaps
description: "Systematic audit of the integrity engine (artifact_graph.rs check_integrity) to identify what integrity checks are missing. Cross-reference the planning placement rule, idea lifecycle, epic reconciliation, and other RULE-b10fe6d1 integrity requirements against what the engine actually checks."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Full inventory of RULE-b10fe6d1 integrity checks vs what check_integrity() implements
  - "Gap list: checks defined in rules but not implemented in the engine"
  - Priority assessment for each gap
  - Tasks created for high-priority gaps
relationships:
  - target: EPIC-82dd0bd2
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-22c9442a
    type: depends-on
  - target: TASK-80c05aa7
    type: depended-on-by
---

## What

The integrity engine currently checks broken links, missing inverses, null targets, and research gaps. RULE-b10fe6d1 defines many more checks (planning placement, status consistency, promotion chain integrity, etc.) that aren't implemented yet. Audit the gap.

## How

1. Read RULE-b10fe6d1 "Artifact Integrity Checks" section — list every check
2. Read `check_integrity()` in `artifact_graph.rs` — list every implemented check
3. Cross-reference: what's defined but not implemented?
4. Prioritise gaps by impact on data integrity
5. Create tasks for high-priority gaps

## Verification

Gap inventory documented. High-priority tasks created.

## Lessons

(To be filled during audit)