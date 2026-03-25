---
id: "EPIC-9b58fdcb"
type: "epic"
title: "Claude Code connector rewrite — dual-manifest plugin with LSP, MCP, agents, and hooks"
description: "Rewrites the Claude Code connector as a properly packaged dual-manifest plugin (orqa-plugin.json + .claude-plugin/plugin.json). Fixes path bugs and outdated intent mappings, maps all 9 OrqaStudio agents to Claude Code subagent definitions, adds new skills (artifact-creation, delegation-patterns, governance-context), improves hooks (validate-artifact, save-context, subagent-review), adds MCP server for artifact graph API, adds LSP server for real-time frontmatter validation, and adds new slash commands (/orqa-validate, /orqa-create)."
status: "active"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# EPIC-9b58fdcb: Claude Code Connector Rewrite

## Goal

The claude-code connector bridges OrqaStudio's governance system with Claude Code's plugin framework. The current version has path bugs, outdated intent mappings, manual installation, and doesn't leverage Claude Code's full capabilities (MCP, LSP, agent teams, advanced hooks).

The rewrite creates a properly packaged connector that serves as BOTH an OrqaStudio plugin (`orqa-plugin.json`) AND a Claude Code plugin (`.claude-plugin/plugin.json`), with the app's Rust backend providing LSP and MCP servers.

## Phases

1. **Fix connector basics** — path bugs, intent mappings, license, dual manifests
2. **Agent mapping** — 9 agent definitions, new skills, orchestrator delegation model
3. **Hook improvements** — validate-artifact, save-context, subagent-review, config-driven injection
4. **MCP server** — Rust backend exposes artifact graph API over stdio
5. **LSP server** — Rust backend provides real-time frontmatter validation
6. **Plugin packaging** — dual-manifest testing, new slash commands

## Out of Scope

- App-level agent framework extensions (sub-agent pipelines, memory support)
- These are tracked separately and depend on more app-level functionality