---
id: TASK-176
title: Implement capability resolution in app agent execution pipeline
description: The Rust backend resolves agent capabilities to Tauri tool names when building agent execution context.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-051
depends-on:
  - TASK-173
assignee: AGENT-002
docs:
  - .orqa/documentation/development/coding-standards.md
skills:
  - backend-best-practices
  - orqa-domain-services
  - orqa-ipc-patterns
  - orqa-error-composition
scope:
  - src-tauri/src/domain/
  - src-tauri/src/commands/
acceptance:
  - Rust backend reads agent capabilities from agent definition
  - Capabilities are resolved to Tauri command tool names
  - Resolved tools are passed to the agent execution context
  - Agents without capabilities field fall back to tools field
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
