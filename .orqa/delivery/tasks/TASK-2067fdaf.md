---
id: "TASK-2067fdaf"
title: "Update delegation rules for capability resolution"
description: "Update RULE-532100d9 and RULE-deab6ea7 to reference capability-based delegation and skill loading."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-1dab5ebe"
docs:
  - "DOC-01ddd8aa"
acceptance:
  - "RULE-532100d9 delegation protocol includes capability resolution step"
  - "RULE-deab6ea7 skill loading references capability-based tool access"
  - "Both rules reference RULE-92dba0cb for the mapping table"
relationships:
  - target: "EPIC-0a7b21cf"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ad922861"
    type: "depends-on"
  - target: "TASK-413692fe"
    type: "depended-on-by"
---
## What

Update the two rules most affected by the tool abstraction:
- [RULE-532100d9](RULE-532100d9) (agent-delegation) — add capability resolution to the delegation protocol
- [RULE-deab6ea7](RULE-deab6ea7) (skill-enforcement) — update loading mechanism references

## How

1. In [RULE-532100d9](RULE-532100d9), add a step to the delegation protocol: "Resolve agent capabilities
   to current-context tool names using [RULE-92dba0cb](RULE-92dba0cb) mapping"
2. In [RULE-deab6ea7](RULE-deab6ea7), update references from "agent YAML tools list" to "agent capabilities
   resolved per context"
3. Add [RULE-92dba0cb](RULE-92dba0cb) to Related Rules in both

## Verification

- Both rules updated and pass schema validation
- Delegation protocol explicitly mentions capability resolution
- No references to concrete tool lists in agent YAML remain in either rule
