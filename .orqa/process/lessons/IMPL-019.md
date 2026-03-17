---
id: IMPL-019
title: Plugins and sidecars are paired — no requirement mechanism exists
description: "The orqastudio-claude-plugin is designed for Claude Code CLI. If a different sidecar is used (Cursor, Copilot), this plugin should not be active. Currently there is no mechanism for a plugin to declare which sidecar it requires, or for the system to enforce that pairing."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: RULE-003
    type: informs
    rationale: "Plugin config integrity — a plugin that requires a specific sidecar has an implicit dependency that is not expressed in any config"
  - target: IDEA-071
    type: informs
    rationale: "Enforcement mechanism designed in TASK-307, implementation tracked by IDEA-071"
  - target: TASK-307
    type: informs
    rationale: "TASK-307 designed the plugin-sidecar pairing mechanism that addresses this lesson"
  - target: IDEA-071
    type: informs
    rationale: "Lesson informs IDEA-071 — implementation of the plugin-sidecar pairing mechanism"
  - target: IDEA-071
    type: informed-by
    rationale: "Promoted to this idea for implementation of the plugin-sidecar pairing mechanism"
  - target: IMPL-027
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-027"
  - target: IMPL-043
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-043"
  - target: IMPL-028
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-028"
  - target: IMPL-020
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-020"
---
## Pattern

The companion plugin (`orqastudio-claude-plugin`) contains hooks, rules, skills, and agents designed specifically for the Claude Code CLI sidecar. These artifacts assume:

- Claude Code tool names (PascalCase: `Read`, `Edit`, `Bash`)
- Claude Code MCP server availability (ChunkHound)
- Claude Code slash command and skill loading mechanisms

If a user switches to a different sidecar (e.g., Cursor with its own plugin ecosystem), the Claude-specific plugin's hooks would fire in the wrong context, its skills would reference unavailable tools, and its agents would try to use capabilities that don't exist.

The pairing is implicit — nothing in `plugin.json` declares "I require the Claude Code sidecar" and nothing in the system checks that constraint.

## Fix

Design is in progress via [RES-052](RES-052). Key decisions made:
- Plugins declare `requires.ai-providers` with `any-of`/`all` semantics
- Provider definitions live in `.orqa/providers/<name>.json` (app-native, identity + detection + required plugins)
- Plugin type determines requires shape — only `ai-provider-integration` type has `requires.ai-providers`
- Capability fulfilment is user-configurable per-project (native vs app-MCP), with plugin-provided defaults
- Plugin installation wires capabilities, skills, and agent updates as a complete package
- Provider-side plugin requirements are a pragmatic bridge until [IDEA-069](IDEA-069) (sidecar-as-plugin)

## Triage

Design completed in [TASK-307](TASK-307). Implementation deferred to [IDEA-071](IDEA-071).
