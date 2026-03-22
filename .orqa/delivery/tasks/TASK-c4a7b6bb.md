---
id: TASK-c4a7b6bb
title: Backfill missing bidirectional inverses
description: "For every relationship A --type--> B, ensure B --inverse--> A exists. Add relationships arrays to artifact types that need them."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - Every relationship has a bidirectional inverse
  - verify-pipeline-integrity.mjs reports zero missing inverses
  - Pillar schemas support optional relationships for receiving grounded-by edges
rule-overrides: []
relationships:
  - target: EPIC-942c7678
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-e7324438
    type: depends-on
  - target: TASK-d5d3e417
    type: depends-on
  - target: TASK-5da55ccb
    type: depended-on-by
  - target: TASK-cea1bc37
    type: depended-on-by
---

## What

Ensure bidirectional consistency across all relationship edges in the artifact graph.

## How

1. Run `node tools/verify-pipeline-integrity.mjs` to identify missing inverses
2. For each missing inverse, add the corresponding relationship to the target artifact
3. If target artifact type lacks a relationships field (e.g., pillars), update schema to support optional relationships
4. Commit in batches

## Verification

- `node tools/verify-pipeline-integrity.mjs` reports zero missing inverses
- `make verify-integrity` passes clean
