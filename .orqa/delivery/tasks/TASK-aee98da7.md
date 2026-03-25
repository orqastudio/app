---
id: TASK-aee98da7
type: task
title: "Token efficiency — lazy rule loading"
status: captured
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "Rules are not all loaded into every prompt"
  - "Only relevant rules loaded based on task context"
  - "Token usage per prompt measurably reduced"
  - "RES-2f602d54 recommendations implemented (at minimum: lazy rule loading)"
relationships:
  - target: EPIC-4304bdcc
    type: delivers
  - target: TASK-8870f959
    type: depends-on
    rationale: "Team design informs how rules are loaded into agent prompts"
  - target: TASK-272b3d07
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-272b3d07"
---
## What

Implement the top recommendation from RES-2f602d54: lazy rule loading. Currently 58 rule files (~8,800 tokens) are loaded into every prompt. Load only rules relevant to the current task.

Depends on team design research (TASK-8870f959) because generated system prompts change how rules reach agents.
