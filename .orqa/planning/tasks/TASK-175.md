---
id: TASK-175
title: Implement capability resolution in companion plugin SubagentStart hook
description: The companion plugin resolves agent capabilities to Claude Code tool names when subagents spawn.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-051
depends-on:
  - TASK-173
assignee: AGENT-002
docs: []
skills:
  - orqa-plugin-development
  - rule-enforcement
scope:
  - orqa-plugin/hooks/
acceptance:
  - SubagentStart hook reads agent definition capabilities
  - Hook resolves capabilities to Claude Code CLI tool names
  - Resolved tools are injected as additionalContext for the subagent
  - Agents without capabilities field fall back to tools field
---

## What

When the companion plugin (EPIC-050) spawns a subagent, the `SubagentStart` hook
reads the agent's `capabilities` field and resolves it to Claude Code tool names
using the mapping from [RULE-040](RULE-040).

## How

1. In the plugin's SubagentStart hook, read agent definition from `.orqa/team/agents/`
2. Extract `capabilities` array
3. Resolve each capability to the CLI tool name using the mapping table
4. Return resolved tool names as `additionalContext`
5. Fall back to raw `tools` field if `capabilities` is missing (backwards compat)

## Verification

- Subagent receives correct CLI tool names via additionalContext
- No app-only tool names leak into CLI subagents
- Backwards compatibility with tools field works
