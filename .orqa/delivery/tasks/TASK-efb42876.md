---
id: TASK-efb42876
type: task
title: "Update project.json artifact tree config"
description: "Update project.json to replace 'skills' artifact tree entries with 'knowledge' entries, pointing to the renamed knowledge/ directories."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - project.json artifact tree has no 'skills' entries
  - knowledge/ directory path is correctly declared in artifact tree
  - orqa enforce schema passes on project.json
  - The app's artifact browser shows knowledge artifacts under the correct tree node
relationships:
  - target: EPIC-fdcdb958
    type: delivers
  - target: TASK-30f5bdc8
    type: depends-on
---

## What

Update the `project.json` file (and any plugin-level `project.json` overrides) to:

1. Replace `skills` artifact tree node with `knowledge`
2. Update directory paths from `process/skills/` to `process/knowledge/`
3. Update type references from `skill` to `knowledge`
4. Ensure the artifact browser in the app can navigate to knowledge artifacts

## How

Read `project.json` and locate the artifact tree configuration section. Find entries with:

- `"type": "skill"` or `"skills"` path segments
- Replace with `"type": "knowledge"` and `knowledge/` path accordingly

Also check any plugin-level configuration that extends or overrides the main artifact tree.

After updating, run `orqa enforce` to confirm the project config is valid.

## Verification

1. `orqa enforce` passes on the updated project.json
2. `orqa graph` shows knowledge artifacts in the correct tree position
3. The app artifact browser navigation shows "Knowledge" not "Skills"
4. No 'skills' path segments remain in project.json
