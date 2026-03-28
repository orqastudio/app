---
id: "IMPL-b8ec72e2"
type: lesson
title: "Plugins and sidecars are paired — no requirement mechanism exists"
description: "The orqastudio-claude-plugin is designed for Claude Code CLI. If a different sidecar is used (Cursor, Copilot), this plugin should not be active. Currently there is no mechanism for a plugin to declare which sidecar it requires, or for the system to enforce that pairing."
status: archived
archived_reason: "Superseded by connector architecture — 'sidecar' renamed to 'connector plugin', pairing mechanism redesigned in plugin taxonomy"
created: "2026-03-13"
updated: "2026-03-28"
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

Design is in progress via [RES-fbe69e04](RES-fbe69e04). Key decisions made:

- Plugins declare `requires.ai-providers` with `any-of`/`all` semantics
- Provider definitions live in `.orqa/providers/<name>.json` (app-native, identity + detection + required plugins)
- Plugin type determines requires shape — only `ai-provider-integration` type has `requires.ai-providers`
- Capability fulfilment is user-configurable per-project (native vs app-MCP), with plugin-provided defaults
- Plugin installation wires capabilities, skills, and agent updates as a complete package
- Provider-side plugin requirements are a pragmatic bridge until [IDEA-d2a429c3](IDEA-d2a429c3) (sidecar-as-plugin)

## Triage

Design completed in [TASK-f51abfea](TASK-f51abfea). Implementation deferred to [IDEA-459f417a](IDEA-459f417a).
