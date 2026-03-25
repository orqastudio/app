---
id: "TASK-1ee2031e"
type: "task"
title: "Plugin reads skill scope fields for path-based injection"
description: "Update the companion plugin to read skill scope/layer fields and auto-inject skills when agents touch matching file paths, replacing the hardcoded injection table."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
docs: []
acceptance:
  - "Skills with matching scope are auto-injected on file operations"
  - "No hardcoded path→skill table in plugin or orchestrator"
  - "Skill discovery is dynamic — adding a new skill with scope auto-enables injection"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Instead of the orchestrator prompt containing a hardcoded table mapping file paths to skills, the plugin reads skill metadata directly. Each skill's SKILL.md frontmatter already has scope information — the plugin just needs to use it.

This makes skill injection self-building: create a new skill with the right scope field, and it's automatically injected when agents touch matching paths. No orchestrator prompt update needed.

## How

1. On plugin load, scan `.orqa/process/skills/*/SKILL.md` for scope fields
2. Build a map of path patterns → skill names from skill metadata
3. On PreToolUse (Write/Edit), match the file path against the map
4. Inject matching skills that haven't been loaded yet this session

## Verification

- Writing to `backend/src-tauri/src/domain/` triggers backend skill injection
- Writing to `ui/src/lib/components/` triggers frontend skill injection
- Adding a new skill with scope auto-enables injection without config changes
- No hardcoded injection table exists in the plugin or orchestrator