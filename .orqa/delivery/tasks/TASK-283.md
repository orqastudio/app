---
id: TASK-283
title: "Fix all broken links and frontmatter refs"
description: "Run verify-links.mjs across all .orqa/ artifacts, fix every broken reference. Commit in batches by artifact type."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-281, TASK-282]
assignee: null
docs: []
skills: []
acceptance:
  - "verify-links.mjs runs clean with zero broken references"
  - "All artifact cross-references resolve to existing files"
  - "All frontmatter refs (depends-on, epic, milestone, etc.) point to valid artifacts"
rule-overrides: []
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
