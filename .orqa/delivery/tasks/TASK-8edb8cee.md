---
id: "TASK-8edb8cee"
type: "task"
title: "Register the connector plugin in Claude Code settings"
status: "captured"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-b07d074c"
    type: "delivers"
  - target: "TASK-72163412"
    type: "depends-on"
---

# TASK-8edb8cee: Register Connector Plugin in Claude Code Settings

## Acceptance Criteria

1. Plugin entry added to Claude Code's settings/configuration
2. Claude Code detects the plugin on startup (visible in plugin list or logs)
3. Both manifests validated — orqa-plugin.json (OrqaStudio side) and .claude-plugin/plugin.json (Claude Code side)
4. Plugin hooks registered with correct event bindings
5. MCP and LSP server endpoints configured and reachable
6. No errors or warnings during plugin load