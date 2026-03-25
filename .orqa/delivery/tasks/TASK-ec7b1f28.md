---
id: "TASK-ec7b1f28"
type: "task"
title: "Fix all broken links and frontmatter refs"
description: "Run verify-links.mjs across all .orqa/ artifacts, fix every broken reference. Commit in batches by artifact type."
status: "completed"
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
assignee: null
docs: []
acceptance:
  - "verify-links.mjs runs clean with zero broken references"
  - "All artifact cross-references resolve to existing files"
  - "All frontmatter refs (depends-on, epic, milestone, etc.) point to valid artifacts"
rule-overrides: []
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-c12c22b1"
    type: "depends-on"
  - target: "TASK-0415e966"
    type: "depends-on"
---

## What

Fix every broken link and frontmatter reference across all .orqa/ artifacts.

## How

1. Run `node tools/verify-links.mjs` to get full report
2. Fix each broken reference — update to correct target or remove if target no longer exists
3. Commit in batches by artifact type (rules, skills, decisions, etc.)

## Verification

- `node tools/verify-links.mjs` reports zero issues
- `make verify-links` passes clean