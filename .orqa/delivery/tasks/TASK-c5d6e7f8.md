---
id: TASK-c5d6e7f8
title: "Update core docs: vision.md, artifact-framework.md, CLAUDE.md"
description: "Update the core documentation files to explain the knowledge vs skills semantic distinction, document the 'knowledge' artifact type, and update all references from 'skill' to 'knowledge'."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - vision.md explains knowledge (domain context injected into agents) vs skills (user-facing slash commands)
  - artifact-framework.md documents the knowledge artifact type with schema, lifecycle, and examples
  - CLAUDE.md (app orchestrator) updated to reference knowledge/ not skills/
  - No doc file uses 'skill' to mean OrqaStudio's domain knowledge concept
  - Documentation passes orqa validate schema checks
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-a1b2c3d4
    type: depends-on
  - target: TASK-3a4b5c6d
    type: depends-on
---

## What

Update the following documentation files:
1. `app/.orqa/documentation/about/vision.md` — add section explaining knowledge vs skills distinction
2. `app/.orqa/documentation/about/artifact-framework.md` — update/add knowledge artifact type documentation
3. `app/.claude/CLAUDE.md` — update all skill references to knowledge references

The semantic distinction to document:
- **Knowledge** = domain context files injected into agents at delegation time (OrqaStudio concept)
- **Skills** = user-invocable slash commands in Claude Code (Claude Code concept)

## How

Per RULE-008 (documentation-first), docs define the target state. Update docs first, then verify they are consistent with the code changes in other tasks.

For `vision.md`: add a "Knowledge vs Skills" section near the agent governance description.

For `artifact-framework.md`: update the artifact type table to show `knowledge` instead of `skill`, with the new ID prefix `KNOW-` and directory path `process/knowledge/`.

For `CLAUDE.md`: search for "skill" references in the Knowledge Injection section and surrounding text — update to use "knowledge" terminology.

## Verification

1. All three docs updated with correct terminology
2. `orqa validate` passes on the documentation artifacts
3. The "Knowledge vs Skills" distinction is clear to a new reader of vision.md
4. CLAUDE.md Knowledge Injection section references `knowledge/` paths correctly
