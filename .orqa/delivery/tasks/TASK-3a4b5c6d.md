---
id: TASK-3a4b5c6d
title: "Mass artifact rename: skills/ → knowledge/ directories, SKILL-* → KNOW-* IDs"
description: Rename all .orqa/process/skills/ directories to .orqa/process/knowledge/ and migrate all SKILL-XXXXXXXX IDs to KNOW-XXXXXXXX across the project, app, and all consuming plugins.
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - No directory named skills/ exists under .orqa/process/ in any project or plugin
  - No SKILL-* ID remains in any .md artifact file
  - All relationship targets updated to the new KNOW-* IDs
  - orqa enforce passes on all projects after the rename
  - project.json artifact trees updated to reference knowledge/ paths
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-a1b2c3d4
    type: depends-on
  - target: TASK-c3d4e5f6
    type: depended-on-by
  - target: TASK-7e8f9a0b
    type: depended-on-by
  - target: TASK-c5d6e7f8
    type: depended-on-by
---

## What

Perform the mass rename across all repositories:
1. Rename `.orqa/process/skills/` → `.orqa/process/knowledge/` in every location (main project, app submodule, all plugins)
2. Rename all `SKILL-XXXXXXXX.md` files to `KNOW-XXXXXXXX.md` (preserving the 8-char hex suffix)
3. Update all `id:` fields in renamed files from `SKILL-*` to `KNOW-*`
4. Update all relationship `target:` fields that reference `SKILL-*` IDs

## How

This is a coordinated multi-step operation:

1. For each repo containing `.orqa/process/skills/`:
   - `git mv .orqa/process/skills .orqa/process/knowledge`
   - Rename each file: `SKILL-XXXXXXXX.md` → `KNOW-XXXXXXXX.md`

2. Mass-update frontmatter IDs:
   - Use `sed` or a script to replace `id: SKILL-` → `id: KNOW-` in all renamed files
   - Replace `target: KNOW-` → `target: KNOW-` in all `.md` files (relationship references)

3. Update `project.json` artifact tree entries (see TASK-5a6b7c8d for project.json specifics)

4. Run `orqa enforce` on each project to catch any missed references

Scope: `app/.orqa/`, `.orqa/`, all `plugins/*/` directories, `connectors/claude-code/`

## Verification

1. `find . -type d -name "skills"` returns zero results under `.orqa/`
2. `grep -r "SKILL-" .orqa/` returns zero matches
3. `orqa enforce` passes on main project
4. `orqa enforce` passes on app submodule
5. `orqa enforce` passes on all plugin directories
