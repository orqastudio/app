---
id: IDEA-025
title: Agent task ownership review
description: |
  Audit agent definitions for process vs execution knowledge placement, define clear task ownership boundaries between orchestrator and specialist agents.
status: surpassed
created: "2026-03-07"
updated: "2026-03-12"
pillars:
  - PILLAR-001
research-needed:
  - Audit all 16 agent definitions for process vs execution knowledge placement
  - Identify process skills incorrectly embedded in specialist agents
  - Define clear task ownership boundaries between orchestrator and agents
  - Review whether any orchestrator responsibilities are missing from its definition
promoted-to: null
relationships:
  - target: TASK-406
    type: informed-by
    rationale: "Auto-generated inverse of informed-by relationship from TASK-406"
---
## Motivation

Agent task ownership boundaries between orchestrator and specialist agents were unclear, causing process knowledge to leak into execution agents.

## Archived

This concern was resolved by [EPIC-045](EPIC-045) (Portable Governance Framework), which replaced the 15 specialist agent definitions with 7 universal roles and established clear orchestrator-vs-agent ownership boundaries via [RULE-001](RULE-001). Process knowledge now lives in skills loaded by the orchestrator, and execution knowledge in skills loaded by implementers.

## Problem

During UAT Round 1 [EPIC-043](EPIC-043), we discovered that process knowledge (UAT methodology) was embedded in a specialist agent (qa-tester) rather than at the orchestrator level where it belongs. This suggests a broader pattern: task ownership between the orchestrator and specialist agents may not be clearly defined across all agent definitions.

## Proposed Investigation

1. Audit each agent definition for process-level knowledge that should be orchestrator-owned
2. Audit the orchestrator for technical execution knowledge that should be agent-owned
3. Define a clear "orchestrator owns process, agents own execution" boundary
4. Extract misplaced knowledge into skills loadable by the correct owner
5. Document the ownership model in the agent-delegation rule

## Origin

- [IMPL-013](IMPL-013): Process skills belong at orchestration level
- UAT Round 1 [EPIC-043](EPIC-043): Finding during governance encoding phase
