---
id: "IMPL-e22b63b4"
type: lesson
title: "Plugins and sidecars are paired — no requirement mechanism exists"
description: "The orqastudio-claude-plugin is designed for Claude Code CLI. If a different sidecar is used (Cursor, Copilot), this plugin should not be active. Currently there is no mechanism for a plugin to declare which sidecar it requires, or for the system to enforce that pairing."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

The companion plugin (`orqastudio-claude-plugin`) contains hooks, rules, skills, and agents designed specifically for the Claude Code CLI sidecar. These artifacts assume:

- Claude Code tool names (PascalCase: `Read`, `Edit`, `Bash`)
- Claude Code MCP server availability (orqastudio MCP server)
- Claude Code slash command and skill loading mechanisms

If a user switches to a different sidecar (e.g., Cursor with its own plugin ecosystem), the Claude-specific plugin's hooks would fire in the wrong context, its skills would reference unavailable tools, and its agents would try to use capabilities that don't exist.

The pairing is implicit — nothing in `plugin.json` declares "I require the Claude Code sidecar" and nothing in the system checks that constraint.

## Fix

Design is in progress via [RES-cd3d33bf](RES-cd3d33bf). Key decisions made:
- Plugins declare `requires.ai-providers` with `any-of`/`all` semantics
- Provider definitions live in `.orqa/providers/<name>.json` (app-native, identity + detection + required plugins)
- Plugin type determines requires shape — only `ai-provider-integration` type has `requires.ai-providers`
- Capability fulfilment is user-configurable per-project (native vs app-MCP), with plugin-provided defaults
- Plugin installation wires capabilities, skills, and agent updates as a complete package
- Provider-side plugin requirements are a pragmatic bridge until [IDEA-1c7ebe74](IDEA-1c7ebe74) (sidecar-as-plugin)

## Triage

Design completed in [TASK-53493d31](TASK-53493d31). Implementation deferred to [IDEA-1287dd52](IDEA-1287dd52).