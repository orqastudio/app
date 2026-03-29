---
id: KNOW-5c2f9d7a
type: knowledge
status: active
title: "Generated Plugin Structure and Anti-Patterns"
description: "What the connector-generated Claude Code plugin must contain, what it must NOT contain, and the thin-hook constraint"
tier: on-demand
created: 2026-03-29
roles: [implementer, reviewer]
paths: [connectors/, targets/claude-code-plugin/]
tags: [connector, plugin, generation, claude-code, hooks]
relationships:
  - type: synchronised-with
    target: DOC-4d531f5e
---

# Generated Plugin Structure and Anti-Patterns

## What the Generated Plugin Contains

The connector generates a complete tool-native plugin in the target tool's native format. For Claude Code this means the `.claude/` directory.

| Component | Purpose | Notes |
| ----------- | --------- | ------- |
| Permission configuration | Role-scoped file access | Works WITHOUT bypass permissions |
| Agent definitions | Generated from base roles + workflow context | YAML frontmatter + system prompt body |
| Slash commands / skills | Thin wrappers exposing OrqaStudio actions | Uses `skills/`, NOT legacy `commands/` |
| Hook scripts | Marshal events to engine | THIN only — see constraints below |
| hooks.json | Hook declarations | Generated from plugin hook declarations, NOT static |
| Validation rules | Artifact validation constraints | Generated from engine's enforcement crate |

## Agent File Format (Claude Code)

```yaml
---
name: implementer
description: "Implements code changes. Reads task, reads knowledge, writes code, runs checks."
model: sonnet
tools: "Read,Write,Edit,Bash,Grep,Glob,Agent,TaskCreate,TaskUpdate,TaskGet,TaskList,SendMessage"
disallowedTools: ""
maxTurns: 50
skills:
  - orqa-validate
---

[Generated system prompt from engine prompt pipeline]
```

**Critical constraints:**

- NO file-level path permissions in agent frontmatter — Claude Code silently ignores these. Use PreToolUse hooks instead.
- Plugin agents CANNOT set `permissionMode`, `hooks`, or `mcpServers` — silently ignored
- `.claude-plugin/` contains ONLY `plugin.json` — nothing else at that directory level

## The Thin-Hook Constraint

All hooks must follow this pattern:

```text
receive event → call daemon/CLI → apply response
```

**No business logic in hooks.** If a hook has `if/else` trees, scoring algorithms, or domain heuristics, it has exceeded its role.

| What Belongs in Hooks | What Does NOT Belong |
| ---------------------- | --------------------- |
| Calling daemon/CLI with event data | Rule evaluation |
| Applying response from daemon | Knowledge injection algorithms |
| Timeout configuration (seconds, not ms) | Prompt generation |
| Error handling for daemon unreachable | Impact analysis |
| | Artifact validation beyond format check |

## What Does NOT Come from the Connector

Git hooks and linting configs are NOT part of the generated tool-native plugin. Those come from their respective OrqaStudio plugins:

| Content | Source |
| --------- | -------- |
| Git hooks | `core` plugin |
| eslint.config.js | `coding-standards` plugin |
| clippy.toml | `rust` plugin |
| tsconfig.base.json | `typescript` plugin |
| .prettierrc | `coding-standards` plugin |

These plugins install enforcement infrastructure directly to `.orqa/configs/` — not through the connector.

## Development Strategy: Target First

1. Hand-write the target plugin — the ideal output (test fixture in `targets/claude-code-plugin/`)
2. Build the connector and engine infrastructure to generate that exact output
3. Validate: enable generated version, disable hand-written version, verify no functionality lost
4. Once validated: generated version replaces the target

The target is only replaced by generated output — never modified to match imperfect generation.
