---
id: TASK-c3d4e5f6
title: Remove knowledge file copies from connector plugin
description: Delete all copied knowledge files from the connector plugin directory. The connector must not maintain its own copies — it reads from canonical plugin paths at delegation time.
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - No knowledge/ or skills/ directory with copied files exists in connectors/claude-code/
  - Connector plugin manifest no longer declares knowledge file paths pointing to local copies
  - orqa validate passes on the connector plugin
  - The connector still functions correctly (knowledge is read from canonical sources)
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-3a4b5c6d
    type: depends-on
  - target: TASK-7e8f9a0b
    type: depends-on
  - target: TASK-a7b8c9d0
    type: depended-on-by
  - target: TASK-a9b0c1d2
    type: depended-on-by
---

## What

Remove the copied knowledge files from the `connectors/claude-code/` directory. Per AD-060 and the connector bridge model, the connector reads knowledge from the canonical plugin paths declared in `project.json` — it must not maintain its own copies that can diverge or go stale.

## How

1. Identify all knowledge/skill file copies in `connectors/claude-code/`:
   - Look for `knowledge/`, `skills/`, or individual `*.md` files that are copies of plugin knowledge files
2. Delete them: `git rm -r connectors/claude-code/knowledge/` (or equivalent path)
3. Update the connector's `orqa-plugin.json` to remove any `knowledge` or `skills` directory declarations pointing to local copies
4. Verify the connector's `orchestrator.md` and agent definitions do not hardcode local knowledge paths

Do NOT remove the connector-specific files (orchestrator.md, agent definitions, rules) — only the copied knowledge files.

## Verification

1. `git diff --stat` shows only deletions in `connectors/claude-code/knowledge/` (or skills/)
2. `orqa validate` passes on the connector plugin
3. No knowledge file in the connector has identical content to a file in a canonical plugin
4. TASK-a7b8c9d0 (knowledge resolution) can proceed without needing these files
