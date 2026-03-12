---
id: TASK-172
title: Update agent schema to support capabilities field
description: Replace the tools field in the agent schema with capabilities. The rule owns the tool mapping, not the agent.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-051
depends-on:
  - TASK-171
assignee: AGENT-003
docs:
  - .orqa/documentation/product/artifact-framework.md
skills:
  - orqa-governance
  - orqa-schema-compliance
scope:
  - .orqa/team/agents/schema.json
acceptance:
  - Agent schema includes a capabilities field (array of strings)
  - Schema validates against existing agent definitions (backwards compatible)
  - Schema passes JSON Schema validation
---

## What

Replace `tools` with `capabilities` in the agent schema. The rule (RULE-034) owns
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
