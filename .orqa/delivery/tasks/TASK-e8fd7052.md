---
id: "TASK-e8fd7052"
type: "task"
title: "Design MCP host interface"
description: "Designed the interface for future external MCP server support, defining how OrqaStudio will host and connect to MCP servers."
status: archived
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-02T00:00:00.000Z
acceptance:
  - "MCP host interface is designed for future implementation"
  - "Tool routing strategy accommodates both built-in and external tools"
  - "Configuration approach is documented"
relationships:
  - target: "EPIC-fe3b5ad5"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Designed the MCP host interface for future external MCP server support, covering connection lifecycle, tool routing, and configuration format.

## How

Defined the discover/connect/list-tools lifecycle states, designed a unified tool router that dispatches to either built-in handlers or external MCP servers by tool name prefix, and documented the `.mcp.json` configuration format.

## Verification

MCP host interface design is documented with lifecycle, routing strategy, and configuration format specified for future implementation.
