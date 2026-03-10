---
id: IMPL-013
title: Process Skills at Orchestration Level
description: |
  Orchestration-level process knowledge (like UAT methodology) must be loadable by the orchestrator. Embedding it in specialist agent definitions makes it available only during that agent's execution.
status: active
created: 2026-03-07
updated: 2026-03-07
recurrence: 1
---
## What Happened

During UAT Round 1, the UAT process was initially encoded only in the qa-tester agent definition. But UAT is orchestrator-led — the orchestrator collects findings, systematizes them, and creates tasks. The qa-tester only does technical verification later.

Embedding orchestration-level process knowledge in a specialist agent means it's only available when that agent is active, not when the orchestrator is coordinating.

## Why It Matters

Task ownership must be clear between the orchestrator and specialist agents:

- **Orchestrator** owns process: coordination, collection, triage, task creation, governance encoding
- **Specialist agents** own execution: technical verification, code implementation, review

Process skills (like UAT methodology) should be loadable by the orchestrator, not buried in agent definitions.

## The Correct Approach

1. Process knowledge → skill (loadable by orchestrator and relevant agents)
2. Technical knowledge → agent definition or Tier 2 skill (loaded during execution)
3. If in doubt, ask: "Who leads this activity?" — that's where the knowledge belongs

## Broader Implication

A general agent review is needed to ensure task ownership is properly defined across all agent definitions. Some process knowledge may be incorrectly embedded in specialist agents rather than available at orchestration level.
