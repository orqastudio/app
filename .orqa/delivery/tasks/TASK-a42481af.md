---
id: TASK-a42481af
type: task
title: Design MCP host interface
description: "Designed the interface for future external MCP server support, defining how OrqaStudio will host and connect to MCP servers."
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - MCP host interface is designed for future implementation
  - Tool routing strategy accommodates both built-in and external tools
  - Configuration approach is documented
relationships:
  - target: EPIC-f132980b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-8dd8819a
    type: depended-on-by
---
## What

Designed the MCP host interface for future external MCP server support, covering connection lifecycle, tool routing, and configuration format.

## How

Defined the discover/connect/list-tools lifecycle states, designed a unified tool router that dispatches to either built-in handlers or external MCP servers by tool name prefix, and documented the `.mcp.json` configuration format.

## Verification

MCP host interface design is documented with lifecycle, routing strategy, and configuration format specified for future implementation.