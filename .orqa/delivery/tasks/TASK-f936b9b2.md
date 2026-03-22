---
id: "TASK-f936b9b2"
title: "Migrate agent definitions from tools to capabilities"
description: "Update all 7 agent definitions to declare capabilities instead of concrete tool names."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-1dab5ebe"
docs:
  - "DOC-01ddd8aa"
acceptance:
  - "All 7 agent definitions have a capabilities field"
  - "Capabilities map correctly to the vocabulary defined in RULE-92dba0cb"
  - "Each agent's capability set matches its current tool access"
  - "All agent definitions pass schema validation"
relationships:
  - target: "EPIC-0a7b21cf"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ad922861"
    type: "depends-on"
  - target: "TASK-561205e2"
    type: "depends-on"
  - target: "TASK-027139e7"
    type: "depended-on-by"
  - target: "TASK-65c86121"
    type: "depended-on-by"
  - target: "TASK-2df410be"
    type: "depended-on-by"
  - target: "TASK-413692fe"
    type: "depended-on-by"
---
## What

Replace the flat `tools:` arrays (which mix CLI and App tool names) with `capabilities:`
arrays using the vocabulary from [RULE-92dba0cb](RULE-92dba0cb).

## How

1. For each agent definition, map its current `tools` list to capabilities using the
   mapping table in [RULE-92dba0cb](RULE-92dba0cb)
2. Add `capabilities` field with the abstract names
3. Remove the `tools` field (or leave empty if schema requires it)
4. Verify each agent's capability set is correct for its role

## Verification

- All 7 agent .md files updated
- No concrete tool names remain in `tools` field
- Capabilities match the agent's role boundaries
- Schema validation passes for all agents
