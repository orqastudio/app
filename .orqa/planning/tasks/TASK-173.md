---
id: TASK-173
title: Migrate agent definitions from tools to capabilities
description: Update all 7 agent definitions to declare capabilities instead of concrete tool names.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-051
depends-on:
  - TASK-171
  - TASK-172
assignee: AGENT-003
docs:
  - .orqa/documentation/product/artifact-framework.md
skills:
  - orqa-governance
scope:
  - .orqa/team/agents/
acceptance:
  - All 7 agent definitions have a capabilities field
  - Capabilities map correctly to the vocabulary defined in RULE-040
  - Each agent's capability set matches its current tool access
  - All agent definitions pass schema validation
---

## What

Replace the flat `tools:` arrays (which mix CLI and App tool names) with `capabilities:`
arrays using the vocabulary from [RULE-040](RULE-040).

## How

1. For each agent definition, map its current `tools` list to capabilities using the
   mapping table in [RULE-040](RULE-040)
2. Add `capabilities` field with the abstract names
3. Remove the `tools` field (or leave empty if schema requires it)
4. Verify each agent's capability set is correct for its role

## Verification

- All 7 agent .md files updated
- No concrete tool names remain in `tools` field
- Capabilities match the agent's role boundaries
- Schema validation passes for all agents
