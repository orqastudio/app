---
id: TASK-284
title: "Backfill missing bidirectional inverses"
description: "For every relationship A --type--> B, ensure B --inverse--> A exists. Add relationships arrays to artifact types that need them."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-281, TASK-282]
assignee: null
docs: []
skills: []
acceptance:
  - "Every relationship has a bidirectional inverse"
  - "verify-pipeline-integrity.mjs reports zero missing inverses"
  - "Pillar schemas support optional relationships for receiving grounded-by edges"
rule-overrides: []
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
