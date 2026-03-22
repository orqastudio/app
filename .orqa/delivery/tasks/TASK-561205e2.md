---
id: TASK-561205e2
title: Update agent schema to support capabilities field
description: "Replace the tools field in the agent schema with capabilities. The rule owns the tool mapping, not the agent."
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-1dab5ebe
docs:
  - DOC-01ddd8aa
acceptance:
  - Agent schema includes a capabilities field (array of strings)
  - Schema validates against existing agent definitions (backwards compatible)
  - Schema passes JSON Schema validation
relationships:
  - target: EPIC-0a7b21cf
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-ad922861
    type: depends-on
  - target: TASK-f936b9b2
    type: depended-on-by
  - target: TASK-413692fe
    type: depended-on-by
---

## What

Replace `tools` with `capabilities` in the agent schema. The rule (RULE-2f7b6a31) owns
the mapping from capabilities to provider-specific tools — agent definitions only
declare what they need, never concrete tool names.

## How

1. Read current agent schema
2. Remove `tools` property
3. Add `capabilities` property as an array of strings
4. Validate schema is well-formed

## Verification

- Schema file validates as JSON Schema
- Existing agent definitions still pass validation
- New `capabilities` field is accepted
