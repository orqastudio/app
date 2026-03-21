---
id: "TASK-d9d85326"
title: "Document plugin architecture and SDK extraction plan"
description: "Document the plugin system architecture and plan for component library / SDK extraction."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-ec1b3785"
acceptance:
  - "Document covers: current plugin capabilities, component library extraction plan, view registration API, theme tokens"
  - "Built-in vs plugin boundary documented with decision framework"
  - "Document lives in .orqa/documentation/development/"
relationships:
  - target: "EPIC-4726cb3b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-76e79dba"
    type: "depends-on"
  - target: "TASK-91bc09f9"
    type: "depended-on-by"
---
## What

Document the plugin architecture including the path to SDK extraction for dynamic plugin views.

## How

1. Map current plugin capabilities (CLI companion plugin, MCP integration)
2. Identify components that should be extracted to a shared SDK
3. Design view registration API concept
4. Document theme token system for plugin authors
5. Reference [RES-00ec6dd1](RES-00ec6dd1) built-in vs plugin framework

## Verification

A plugin author could read this doc and understand what's available, what's planned, and how to prepare.
