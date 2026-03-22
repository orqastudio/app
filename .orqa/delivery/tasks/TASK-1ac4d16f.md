---
id: TASK-1ac4d16f
title: Cross-layer consistency verification
description: "Verify consistency across the orchestrator, agents, skills, and rules layers — skill injection tables, agent-to-subagent mappings, orphaned artifacts, and cross-references."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Orchestrator skill injection table lists only skills that exist
  - Agent-to-subagent mapping matches available Claude Code subagent types
  - No orphaned skills or rules without justification
  - Orchestrator instructions are consistent with active rule content
relationships:
  - target: EPIC-4a7aeacb
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-b6e9df91
    type: depends-on
  - target: TASK-69b7753b
    type: depends-on
  - target: TASK-f32f3eba
    type: depends-on
  - target: TASK-b743b819
    type: depends-on
  - target: TASK-77b6e5bd
    type: depended-on-by
  - target: TASK-ec136ce9
    type: depended-on-by
---
## What

Final cross-layer verification ensuring all team and enforcement artifacts are internally consistent with each other and with the orchestrator definition.

## How

1. Read orchestrator agent definition (source of CLAUDE.md)
2. Cross-reference skill injection table with `.orqa/process/skills/` contents
3. Cross-reference agent-to-subagent mapping with available subagent types
4. Search for skills and rules not referenced by any other artifact
5. Verify orchestrator instructions don't contradict active rules

## Verification

- Every skill in the injection table exists in `.orqa/process/skills/`
- Every subagent type in the mapping is available in Claude Code
- Orphan report is clean or justified
