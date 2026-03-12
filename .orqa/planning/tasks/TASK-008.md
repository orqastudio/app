---
id: TASK-008
title: Update scanner to use config-driven paths
description: Replace the scanner's folder-guessing logic with config-driven scanning. The scanner reads the artifacts array from project.json and scans exactly those paths. No more inferring groups from README frontmatter.
status: done
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-033
depends-on:
  - TASK-007
assignee: AGENT-002
skills:
  - SKILL-003
  - SKILL-012
  - SKILL-009
acceptance:
  - artifact_scan_tree reads artifacts config from project.json
  - For each Type entry
  - scans path for .md files → DocNode list
  - For each Group entry
  - scans children paths → NavType list → NavGroup
  - Direct types (no children) wrapped in synthetic NavGroup
  - Empty/missing artifacts config returns empty NavTree (no crash)
  - Old folder-guessing logic removed (scan_group_dir
  - scan_type_dirs)
  - README.md files filtered from node lists (they're landing pages)
  - Hidden files (. or _) skipped
  - cargo build and clippy pass
---
## What

The current scanner walks `.orqa/` and guesses what's a group vs type based
on README frontmatter (`role: group`, `role: artifacts`). Replace with:

1. Load `artifacts` config from project settings
2. For each entry: scan its configured `path` for `.md` files
3. Build NavTree from config structure, not discovered structure

## What Gets Deleted

- `scan_group_dir()` guessing logic
- `scan_type_dirs()` guessing logic
- README frontmatter role parsing
- All folder-structure inference code

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
