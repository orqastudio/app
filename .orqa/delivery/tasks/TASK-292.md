---
id: TASK-292
title: "Update project.json, READMEs, and symlinks for app layout"
description: "After directory moves, update project.json artifacts array, all directory README frontmatter (icon, label, description, sort), and .claude/ symlinks so the app scanner renders the new three-level structure correctly."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - project.json artifacts array reflects new process/delivery/documentation structure
  - "Every artifact directory has README.md with frontmatter (icon, label, description)"
  - .claude/ symlinks point to correct new paths
  - App scanner renders new structure in nav tree
  - CLAUDE.md symlink still works
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-287
    type: depends-on
  - target: TASK-288
    type: depends-on
  - target: TASK-289
    type: depends-on
  - target: TASK-299
    type: depended-on-by
  - target: TASK-300
    type: depended-on-by
  - target: TASK-301
    type: depended-on-by
  - target: TASK-349
    type: depended-on-by
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
