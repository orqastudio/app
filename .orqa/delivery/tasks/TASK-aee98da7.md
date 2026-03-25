---
id: TASK-aee98da7
type: task
title: "Token efficiency — lazy rule loading"
status: surpassed
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "Rules are not all loaded into every prompt"
  - "Only relevant rules loaded based on task context"
  - "Token usage per prompt measurably reduced"
  - "RES-2f602d54 recommendations implemented (at minimum: lazy rule loading)"
relationships: []
---
## What

Implement the top recommendation from RES-2f602d54: lazy rule loading. Currently 58 rule files (~8,800 tokens) are loaded into every prompt. Load only rules relevant to the current task.

Depends on team design research (TASK-8870f959) because generated system prompts change how rules reach agents.
