---
id: AD-0dbba717
title: Tool Implementation as MCP
description: Core tools implemented in Rust and exposed to Agent SDK as custom MCP server. Built-in SDK tools disabled.
status: completed
created: 2026-03-02
updated: 2026-03-02
relationships:
  - target: RES-7f87b149
    type: informed-by
    rationale: RES-7f87b149 found the Agent SDK supports tools:[] to disable built-ins, canUseTool for approval, and createSdkMcpServer() for custom tool exposure
  - target: EPIC-f0937072
    type: drives
  - target: EPIC-46e5f406
    type: drives
  - target: DOC-7290a3df
    type: documented-by
  - target: DOC-e5ff4baf
    type: documented-by
  - target: DOC-a184e87f
    type: documented-by
---
## Decision

OrqaStudio's core tools (Read, Write, Edit, Bash, Glob, Grep) are implemented natively in Rust and exposed to the Agent SDK as a custom MCP server via `createSdkMcpServer()`. The Agent SDK's built-in tools are disabled (`tools: []`). OrqaStudio also acts as an MCP host for user-provided MCP servers (extensibility).

## Rationale

Native Rust tools give OrqaStudio full control over tool execution, permission model, and UI rendering. Exposing them as MCP servers to the Agent SDK is the SDK's documented extension mechanism. Disabling built-in tools ensures Claude cannot read/write files without OrqaStudio's knowledge. The MCP host capability connects OrqaStudio to the 10,000+ MCP server ecosystem.

## Consequences

Each core tool must be implemented as both a Rust function (for execution) and an MCP tool definition (for the Agent SDK). The `canUseTool` callback routes approval requests to OrqaStudio's UI before execution. Tool results flow back to the Agent SDK, which sends them to Claude for the next conversation turn.
