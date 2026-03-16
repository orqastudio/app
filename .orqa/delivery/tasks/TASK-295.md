---
id: TASK-295
title: Agents content audit (7 agents)
description: "Audit all 7 agent definitions: expand Tier 1 skills where gaps exist, match capabilities to RULE-037 role matrix, update orchestrator prompt for pipeline philosophy."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - All agents have appropriate Tier 1 skills
  - Agent capabilities match RULE-037 role-to-capability matrix
  - Orchestrator prompt reflects pipeline philosophy
  - All path references in agent definitions updated to new structure
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-294
    type: depends-on
  - target: TASK-349
    type: depended-on-by
---

## What

Content audit of all 7 agent definitions for skill coverage and pipeline alignment.

## How

1. Read each agent definition
2. Check Tier 1 skills list against skill audit results (TASK-294)
3. Verify capabilities match [RULE-037](RULE-037) matrix
4. Update orchestrator prompt for pipeline philosophy
5. Update all path references to new directory structure

## Verification

- Every agent has complete Tier 1 skills
- Capabilities are consistent with [RULE-037](RULE-037)
- Orchestrator prompt references pipeline concepts
