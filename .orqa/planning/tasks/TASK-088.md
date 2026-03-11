---
id: TASK-088
title: Cross-layer consistency verification
description: Verify consistency across the orchestrator, agents, skills, and rules layers — skill injection tables, agent-to-subagent mappings, orphaned artifacts, and cross-references.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on:
  - TASK-084
  - TASK-085
  - TASK-086
  - TASK-087
scope:
  - Verify orchestrator skill injection table matches available skills
  - Verify agent-to-subagent mapping is current
  - Find orphaned skills (not referenced by any agent)
  - Find orphaned rules (not referenced by any other artifact)
  - Verify orchestrator (CLAUDE.md) is consistent with all active rules
acceptance:
  - Orchestrator skill injection table lists only skills that exist
  - Agent-to-subagent mapping matches available Claude Code subagent types
  - No orphaned skills or rules without justification
  - Orchestrator instructions are consistent with active rule content
---
## What

Final cross-layer verification ensuring all team and enforcement artifacts are internally consistent with each other and with the orchestrator definition.

## How

1. Read orchestrator agent definition (source of CLAUDE.md)
2. Cross-reference skill injection table with `.orqa/team/skills/` contents
3. Cross-reference agent-to-subagent mapping with available subagent types
4. Search for skills and rules not referenced by any other artifact
5. Verify orchestrator instructions don't contradict active rules

## Verification

- Every skill in the injection table exists in `.orqa/team/skills/`
- Every subagent type in the mapping is available in Claude Code
- Orphan report is clean or justified
