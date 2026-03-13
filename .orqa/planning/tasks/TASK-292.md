---
id: TASK-292
title: "Update project.json, READMEs, and symlinks for app layout"
description: "After directory moves, update project.json artifacts array, all directory README frontmatter (icon, label, description, sort), and .claude/ symlinks so the app scanner renders the new three-level structure correctly."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-287, TASK-288, TASK-289]
assignee: null
docs: []
skills: []
acceptance:
  - "project.json artifacts array reflects new process/delivery/documentation structure"
  - "Every artifact directory has README.md with frontmatter (icon, label, description)"
  - ".claude/ symlinks point to correct new paths"
  - "App scanner renders new structure in nav tree"
  - "CLAUDE.md symlink still works"
rule-overrides: []
---

## What

Ensure the app's artifact scanner picks up the new directory structure correctly by updating all configuration and metadata files.

## How

1. Rewrite `project.json` artifacts array for three-level structure
2. Audit and update every README.md in artifact directories for correct frontmatter
3. Recreate `.claude/` symlinks pointing to new paths
4. Verify CLAUDE.md symlink still resolves

## Verification

- `project.json` paths all resolve to existing directories
- Every artifact directory has a README with valid frontmatter
- `.claude/` symlinks are not broken
- App renders new structure in sidebar navigation
