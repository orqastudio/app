---
id: "IMPL-005"
title: "Config paths must match disk paths exactly"
description: "Every path in project.json artifacts must resolve to an actual directory on disk. Mismatches cause silent empty results with no error.\n"
status: "promoted"
created: "2026-03-07"
updated: "2026-03-07"
maturity: "understanding"
recurrence: 1
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Config-disk alignment is fundamental structural integrity"
  - target: "RULE-003"
    type: "grounded-by"
    rationale: "Lesson promoted to RULE-003"
---
## What Happened

All artifact categories showed empty content in the app. The scanner was working correctly, but `project.json` had wrong paths:
- Config said `.orqa/milestones/` — disk had `.orqa/planning/milestones/`
- Config said `.orqa/lessons/` — disk had `.orqa/governance/lessons/`
- Six paths total were wrong

## Why It Was Unexpected

The scanner silently returned empty results when a path didn't exist. No error, no warning — just an empty artifact list. The app showed "no items" which looked like a rendering bug rather than a config bug.

## The Correct Approach

1. Every `path` in `project.json` `artifacts` array must resolve to an actual directory
2. Moving files on disk requires updating config paths in the same commit
3. The scanner should log warnings (not errors) when configured paths don't exist
4. A verification checklist should be run before committing artifact path changes

## Prevention

This lesson was promoted to [RULE-003](RULE-003) (`artifact-config-integrity`), which enforces config-disk alignment and is part of the pre-commit verification checklist.

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
