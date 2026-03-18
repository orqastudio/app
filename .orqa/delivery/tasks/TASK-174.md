---
id: TASK-174
title: Update delegation rules for capability resolution
description: Update RULE-001 and RULE-026 to reference capability-based delegation and skill loading.
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-003
docs:
  - DOC-036
acceptance:
  - RULE-001 delegation protocol includes capability resolution step
  - RULE-026 skill loading references capability-based tool access
  - Both rules reference RULE-040 for the mapping table
relationships:
  - target: EPIC-051
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-171
    type: depends-on
  - target: TASK-341
    type: depended-on-by
---

## What

Update the two rules most affected by the tool abstraction:
- [RULE-001](RULE-001) (agent-delegation) — add capability resolution to the delegation protocol
- [RULE-026](RULE-026) (skill-enforcement) — update loading mechanism references

## How

1. In [RULE-001](RULE-001), add a step to the delegation protocol: "Resolve agent capabilities
   to current-context tool names using [RULE-040](RULE-040) mapping"
2. In [RULE-026](RULE-026), update references from "agent YAML tools list" to "agent capabilities
   resolved per context"
3. Add [RULE-040](RULE-040) to Related Rules in both

## Verification

- Both rules updated and pass schema validation
- Delegation protocol explicitly mentions capability resolution
- No references to concrete tool lists in agent YAML remain in either rule
