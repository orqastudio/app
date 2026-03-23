---
id: "TASK-9f1f8b4b"
type: task
title: "Agents content audit (7 agents)"
description: "Audit all 7 agent definitions: expand Tier 1 skills where gaps exist, match capabilities to RULE-f809076f role matrix, update orchestrator prompt for pipeline philosophy."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "All agents have appropriate Tier 1 skills"
  - "Agent capabilities match RULE-f809076f role-to-capability matrix"
  - "Orchestrator prompt reflects pipeline philosophy"
  - "All path references in agent definitions updated to new structure"
rule-overrides: []
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-b6b5c31c"
    type: "depends-on"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Content audit of all 7 agent definitions for skill coverage and pipeline alignment.

## How

1. Read each agent definition
2. Check Tier 1 skills list against skill audit results (TASK-b6b5c31c)
3. Verify capabilities match [RULE-f809076f](RULE-f809076f) matrix
4. Update orchestrator prompt for pipeline philosophy
5. Update all path references to new directory structure

## Verification

- Every agent has complete Tier 1 skills
- Capabilities are consistent with [RULE-f809076f](RULE-f809076f)
- Orchestrator prompt references pipeline concepts