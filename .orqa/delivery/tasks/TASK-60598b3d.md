---
id: TASK-60598b3d
type: task
title: Register the connector plugin in Claude Code settings
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-5edafb59
    type: delivers
  - target: TASK-c6dd7927
    type: depends-on
  - target: TASK-afeac1b7
    type: depended-on-by
---

# TASK-60598b3d: Register Connector Plugin in Claude Code Settings

## Acceptance Criteria

1. Plugin entry added to Claude Code's settings/configuration
2. Claude Code detects the plugin on startup (visible in plugin list or logs)
3. Both manifests validated — orqa-plugin.json (OrqaStudio side) and .claude-plugin/plugin.json (Claude Code side)
4. Plugin hooks registered with correct event bindings
5. MCP and LSP server endpoints configured and reachable
6. No errors or warnings during plugin load
