---
id: "TASK-f8ea54b9"
type: task
title: "Implement capability resolution in companion plugin SubagentStart hook"
description: "The companion plugin resolves agent capabilities to Claude Code tool names when subagents spawn."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-e5dd38e4"
docs: []
acceptance:
  - "SubagentStart hook reads agent definition capabilities"
  - "Hook resolves capabilities to Claude Code CLI tool names"
  - "Resolved tools are injected as additionalContext for the subagent"
  - "Agents without capabilities field fall back to tools field"
relationships:
  - target: "EPIC-709a6f76"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-eab05905"
    type: "depends-on"
  - target: "TASK-3123558b"
    type: "depended-on-by"
---
## What

When the companion plugin (EPIC-9a1eba3f) spawns a subagent, the `SubagentStart` hook
reads the agent's `capabilities` field and resolves it to Claude Code tool names
using the mapping from [RULE-8abcbfd5](RULE-8abcbfd5).

## How

1. In the plugin's SubagentStart hook, read agent definition from `.orqa/process/agents/`
2. Extract `capabilities` array
3. Resolve each capability to the CLI tool name using the mapping table
4. Return resolved tool names as `additionalContext`
5. Fall back to raw `tools` field if `capabilities` is missing (backwards compat)

## Verification

- Subagent receives correct CLI tool names via additionalContext
- No app-only tool names leak into CLI subagents
- Backwards compatibility with tools field works