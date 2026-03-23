---
id: "TASK-73f7e0fa"
type: task
title: "Implement PreToolUse hook (file + bash event enforcement)"
description: "Plugin PreToolUse hook evaluates active rules against tool calls and blocks/warns on violations."
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
assignee: "AGENT-cc255bc8"
docs: []
acceptance:
  - "PreToolUse hook fires before Edit, Write, and Bash tool calls"
  - "Hook loads active rules with enforcement entries"
  - "File events match against file_path and new_text fields"
  - "Bash events match against command field"
  - "Violations return block with message or warn with additionalContext"
  - "Non-violations allow the tool call to proceed"
relationships:
  - target: "EPIC-3a8ad459"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-0b584382"
    type: "depends-on"
  - target: "TASK-11cf4c1d"
    type: "depended-on-by"
  - target: "TASK-fa39671d"
    type: "depended-on-by"
---
## What

The PreToolUse hook is the primary enforcement mechanism. It intercepts tool calls
before execution and evaluates them against active rules that have `enforcement`
entries in their frontmatter.

## How

1. Create `hooks/pre-tool-use.md` hook definition
2. On each tool call, build enforcement context from tool name and input
3. Load active rules with enforcement entries (via rule engine from TASK-0b584382)
4. For each rule: match event type (file/bash) and evaluate pattern
5. If match: return block or warn action per the enforcement entry
6. If no match: return continue (allow)

## Verification

- Test: Edit with `unwrap()` in Rust file → blocked by [RULE-b49142be](RULE-b49142be)
- Test: Bash with `--no-verify` → blocked by [RULE-633e636d](RULE-633e636d)
- Test: Normal Edit → allowed through
- Test: Rule with status=inactive → not enforced