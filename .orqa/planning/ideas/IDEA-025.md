---
id: IDEA-025
title: Agent task ownership review
description: |
  Audit agent definitions for process vs execution knowledge placement, define clear task ownership boundaries between orchestrator and specialist agents.
status: captured
created: 2026-03-07
updated: 2026-03-07
pillars:
  - PILLAR-001
research-needed:
  - Audit all 16 agent definitions for process vs execution knowledge placement
  - Identify process skills incorrectly embedded in specialist agents
  - Define clear task ownership boundaries between orchestrator and agents
  - Review whether any orchestrator responsibilities are missing from its definition
promoted-to: null
---## Problem

During UAT Round 1 (EPIC-043), we discovered that process knowledge (UAT methodology) was embedded in a specialist agent (qa-tester) rather than at the orchestrator level where it belongs. This suggests a broader pattern: task ownership between the orchestrator and specialist agents may not be clearly defined across all agent definitions.

## Proposed Investigation

1. Audit each agent definition for process-level knowledge that should be orchestrator-owned
2. Audit the orchestrator for technical execution knowledge that should be agent-owned
3. Define a clear "orchestrator owns process, agents own execution" boundary
4. Extract misplaced knowledge into skills loadable by the correct owner
5. Document the ownership model in the agent-delegation rule

## Origin

- IMPL-013: Process skills belong at orchestration level
- UAT Round 1 (EPIC-043): Finding during governance encoding phase
