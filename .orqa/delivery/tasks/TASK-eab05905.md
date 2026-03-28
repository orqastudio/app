---
id: "TASK-eab05905"
type: "task"
title: "Migrate agent definitions from tools to capabilities"
description: "Update all 7 agent definitions to declare capabilities instead of concrete tool names."
status: archived
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-4c94fe14"
docs:
  - "DOC-28344cd7"
acceptance:
  - "All 7 agent definitions have a capabilities field"
  - "Capabilities map correctly to the vocabulary defined in RULE-8abcbfd5"
  - "Each agent's capability set matches its current tool access"
  - "All agent definitions pass schema validation"
relationships:
  - target: "EPIC-709a6f76"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-9726f126"
    type: "depends-on"
  - target: "TASK-34dc7474"
    type: "depends-on"
---

## What

Replace the flat `tools:` arrays (which mix CLI and App tool names) with `capabilities:`
arrays using the vocabulary from [RULE-8abcbfd5](RULE-8abcbfd5).

## How

1. For each agent definition, map its current `tools` list to capabilities using the

   mapping table in [RULE-8abcbfd5](RULE-8abcbfd5)

2. Add `capabilities` field with the abstract names
3. Remove the `tools` field (or leave empty if schema requires it)
4. Verify each agent's capability set is correct for its role

## Verification

- All 7 agent .md files updated
- No concrete tool names remain in `tools` field
- Capabilities match the agent's role boundaries
- Schema validation passes for all agents
