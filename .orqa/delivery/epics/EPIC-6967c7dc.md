---
id: EPIC-6967c7dc
type: epic
title: Claude Code connector rewrite — dual-manifest plugin with LSP, MCP, agents, and hooks
description: Rewrites the Claude Code connector as a properly packaged dual-manifest plugin (orqa-plugin.json + .claude-plugin/plugin.json). Fixes path bugs and outdated intent mappings, maps all 9 OrqaStudio agents to Claude Code subagent definitions, adds new skills (artifact-creation, delegation-patterns, governance-context), improves hooks (validate-artifact, save-context, subagent-review), adds MCP server for artifact graph API, adds LSP server for real-time frontmatter validation, and adds new slash commands (/orqa-validate, /orqa-create).
status: active
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: fulfils
  - target: TASK-367f0026
    type: delivered-by
  - target: TASK-b5f9240b
    type: delivered-by
  - target: TASK-cd24193f
    type: delivered-by
  - target: TASK-c9880303
    type: delivered-by
  - target: TASK-6675ad7c
    type: delivered-by
  - target: TASK-0751c0ff
    type: delivered-by
  - target: TASK-e9a4f8f3
    type: delivered-by
  - target: TASK-907d579d
    type: delivered-by
  - target: TASK-b9c91655
    type: delivered-by
  - target: AD-05756de7
    type: driven-by
  - target: AD-48593e40
    type: driven-by
  - target: TASK-30b1a23d
    type: delivered-by
  - target: TASK-b2518dd2
    type: delivered-by
  - target: TASK-dc8d94c3
    type: delivered-by
  - target: TASK-74e31e06
    type: delivered-by
  - target: TASK-b331119d
    type: delivered-by
  - target: TASK-c72d3ebd
    type: delivered-by
  - target: TASK-ff8a8590
    type: delivered-by
  - target: TASK-d6c514a7
    type: delivered-by
  - target: TASK-f36a6838
    type: delivered-by
---

# EPIC-6967c7dc: Claude Code Connector Rewrite

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
