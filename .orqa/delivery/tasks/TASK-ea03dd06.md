---
id: "TASK-ea03dd06"
type: "task"
title: "Update plugin manifests: all orqa-plugin.json skill → knowledge"
description: "Update all orqa-plugin.json plugin manifest files to replace 'skills' directory references and type declarations with 'knowledge' equivalents."
status: ready
created: 2026-03-20T00:00:00.000Z
updated: 2026-03-20T00:00:00.000Z
acceptance:
  - "All orqa-plugin.json files reference knowledge/ not skills/"
  - "Plugin manifests use 'knowledge' type declarations where 'skill' was used"
  - "orqa enforce schema passes on all plugin manifests"
  - "No plugin manifest references SKILL- prefixed IDs"
relationships:
  - target: "EPIC-fdcdb958"
    type: "delivers"
  - target: "TASK-30f5bdc8"
    type: "depends-on"
  - target: "TASK-126e853f"
    type: "depends-on"
---

## What

Update every `orqa-plugin.json` file across all first-party plugins (software, cli, claude, svelte, tauri, typescript, rust, coding-standards) and the registry to replace skill-related fields with knowledge equivalents.

## How

For each `orqa-plugin.json` in `plugins/*/`:

- Find any field named `skills`, `skillsDir`, or similar → rename to `knowledge`, `knowledgeDir`
- Update any type references: `"type": "skill"` → `"type": "knowledge"`
- Update directory path values: `"skills/"` → `"knowledge/"`
- Update any `provides` or `contributes` arrays that list `skill` artifact types

Also check `registry/official/` for any manifest entries referencing skills.

Run `orqa enforce schema` against each plugin manifest after changes.

## Verification

1. `grep -r '"skill' plugins/` (excluding comments) returns zero matches
2. `orqa enforce schema` passes on every plugin directory
3. Plugin knowledge files are still discovered by `orqa graph` after the rename
