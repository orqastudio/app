---
id: "TASK-ac3c54b4"
type: task
title: "Update delegation rules for capability resolution"
description: "Update RULE-87ba1b81 and RULE-dd5b69e6 to reference capability-based delegation and skill loading."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-4c94fe14"
docs:
  - "DOC-28344cd7"
acceptance:
  - "RULE-87ba1b81 delegation protocol includes capability resolution step"
  - "RULE-dd5b69e6 skill loading references capability-based tool access"
  - "Both rules reference RULE-8abcbfd5 for the mapping table"
relationships:
  - target: "EPIC-709a6f76"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-9726f126"
    type: "depends-on"
  - target: "TASK-3123558b"
    type: "depended-on-by"
---
## What

Update the two rules most affected by the tool abstraction:
- [RULE-87ba1b81](RULE-87ba1b81) (agent-delegation) — add capability resolution to the delegation protocol
- [RULE-dd5b69e6](RULE-dd5b69e6) (skill-enforcement) — update loading mechanism references

## How

1. In [RULE-87ba1b81](RULE-87ba1b81), add a step to the delegation protocol: "Resolve agent capabilities
   to current-context tool names using [RULE-8abcbfd5](RULE-8abcbfd5) mapping"
2. In [RULE-dd5b69e6](RULE-dd5b69e6), update references from "agent YAML tools list" to "agent capabilities
   resolved per context"
3. Add [RULE-8abcbfd5](RULE-8abcbfd5) to Related Rules in both

## Verification

- Both rules updated and pass schema validation
- Delegation protocol explicitly mentions capability resolution
- No references to concrete tool lists in agent YAML remain in either rule