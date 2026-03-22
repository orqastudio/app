---
id: TASK-a9b0c1d2
title: "Update plugin dev docs and connector README"
description: "Update the plugin development documentation to say plugins provide knowledge/ not skills/, and update the connector README to explain the bridge model — no local copies, reads from canonical plugin paths."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - Plugin development docs describe knowledge/ directory and KNOW- artifact type
  - Connector README explains the bridge model: reads from project.json plugin paths
  - No doc refers to the old skills/ directory pattern for plugin authors
  - Connector README explains that orchestrator.md is connector-specific (not copied)
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-c3d4e5f6
    type: depends-on
  - target: TASK-a7b8c9d0
    type: depends-on
---

## What

Update the plugin development documentation and connector README:

1. **Plugin dev docs** (`app/.orqa/documentation/development/plugin-development.md` or equivalent):
   - Replace `skills/` with `knowledge/` in directory structure examples
   - Update artifact type examples from `SKILL-` to `KNOW-` IDs
   - Clarify that knowledge files are domain context for agents, not user commands

2. **Connector README** (`connectors/claude-code/README.md`):
   - Document the bridge model: connector reads knowledge from canonical plugin paths
   - Explain that the connector does NOT copy knowledge files
   - Show how knowledge resolution works (via project.json plugin paths)
   - Distinguish connector-specific files (orchestrator.md, agents) from plugin knowledge

## How

Read the existing plugin development docs to identify all skill/knowledge references. Update each occurrence. Ensure the examples in the docs match the actual directory structure after the rename tasks complete.

For the connector README: if it doesn't exist, create it. Document:
- What the connector is (Claude Code bridge for OrqaStudio)
- File structure (what's connector-specific vs resolved at runtime)
- How knowledge injection works in the delegation model

## Verification

1. Plugin dev docs show only `knowledge/` and `KNOW-` examples
2. Connector README exists and explains the bridge model accurately
3. A new plugin author following the docs would create `knowledge/` not `skills/`
4. `orqa validate` passes on all updated documentation artifacts
