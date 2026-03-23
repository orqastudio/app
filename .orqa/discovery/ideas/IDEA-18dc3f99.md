---
id: IDEA-18dc3f99
type: idea
title: "App plugin loader — aggregate LSP/MCP servers into agent context"
description: "The app's Rust plugin loader should aggregate LSP and MCP server declarations from all installed plugins and load them into the agent context at runtime. Enables the in-app agent framework to use plugin-provided language servers and tool servers."
status: captured
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-140: App Plugin Loader — LSP/MCP in Agent Context

The app's Rust plugin loader already reads `orqa-plugin.json` manifests for schemas, views, widgets, and relationships. Extend it to also aggregate `lspServers` and `mcpServers` declarations from all installed plugins, making them available to the in-app agent framework.

This enables:
- In-app agents get language intelligence from plugin LSP servers
- In-app agents get tool access from plugin MCP servers
- Central registration — one manifest format, multiple consumers (app, CLI, Claude Code)