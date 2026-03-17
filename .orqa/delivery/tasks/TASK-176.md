---
id: TASK-176
title: Implement capability resolution in app agent execution pipeline
description: The Rust backend resolves agent capabilities to Tauri tool names when building agent execution context.
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-002
docs:
  - DOC-021
acceptance:
  - Rust backend reads agent capabilities from agent definition
  - Capabilities are resolved to Tauri command tool names
  - Resolved tools are passed to the agent execution context
  - Agents without capabilities field fall back to tools field
relationships:
  - target: EPIC-051
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-173
    type: depends-on
  - target: SKILL-043
    type: informed-by
  - target: SKILL-009
    type: informed-by
  - target: SKILL-012
    type: informed-by
  - target: SKILL-010
    type: informed-by
  - target: TASK-341
    type: depended-on-by
  - target: SKILL-009
    type: informed-by
  - target: SKILL-010
    type: informed-by
  - target: SKILL-012
    type: informed-by
  - target: SKILL-043
    type: informed-by
---

## What

When the app builds an agent execution context (for dogfooding / app-native agents),
the Rust backend reads the agent's `capabilities` and resolves them to the app's
native tool names.

## How

1. In the agent execution pipeline, read agent definition frontmatter
2. Extract `capabilities` array
3. Resolve using the App mapping table (file_read → read, code_search_regex → search_regex, etc.)
4. Pass resolved tool names to the Agent SDK or tool approval pipeline
5. Fall back to `tools` field if `capabilities` is missing

## Verification

- Agent execution receives correct app tool names
- No CLI-only tool names leak into app agents
- Backwards compatibility with tools field works
