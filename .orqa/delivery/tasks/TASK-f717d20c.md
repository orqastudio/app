---
id: "TASK-f717d20c"
type: task
title: "Agents content audit (7 agents)"
description: "Audit all 7 agent definitions: expand Tier 1 skills where gaps exist, match capabilities to RULE-b723ea53 role matrix, update orchestrator prompt for pipeline philosophy."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "All agents have appropriate Tier 1 skills"
  - "Agent capabilities match RULE-b723ea53 role-to-capability matrix"
  - "Orchestrator prompt reflects pipeline philosophy"
  - "All path references in agent definitions updated to new structure"
rule-overrides: []
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8e9ca15d"
    type: "depends-on"
  - target: "TASK-d2b54d2f"
    type: "depended-on-by"
---
## What

Content audit of all 7 agent definitions for skill coverage and pipeline alignment.

## How

1. Read each agent definition
2. Check Tier 1 skills list against skill audit results (TASK-8e9ca15d)
3. Verify capabilities match [RULE-b723ea53](RULE-b723ea53) matrix
4. Update orchestrator prompt for pipeline philosophy
5. Update all path references to new directory structure

## Verification

- Every agent has complete Tier 1 skills
- Capabilities are consistent with [RULE-b723ea53](RULE-b723ea53)
- Orchestrator prompt references pipeline concepts