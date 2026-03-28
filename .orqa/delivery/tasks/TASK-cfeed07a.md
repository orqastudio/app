---
id: "TASK-cfeed07a"
type: "task"
title: "Document plugin architecture and SDK extraction plan"
description: "Document the plugin system architecture and plan for component library / SDK extraction."
status: archived
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-bbad3d30"
acceptance:
  - "Document covers: current plugin capabilities, component library extraction plan, view registration API, theme tokens"
  - "Built-in vs plugin boundary documented with decision framework"
  - "Document lives in .orqa/documentation/development/"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-f50f84f8"
    type: "depends-on"
---

## What

Document the plugin architecture including the path to SDK extraction for dynamic plugin views.

## How

1. Map current plugin capabilities (CLI companion plugin, MCP integration)
2. Identify components that should be extracted to a shared SDK
3. Design view registration API concept
4. Document theme token system for plugin authors
5. Reference [RES-8fee4dad](RES-8fee4dad) built-in vs plugin framework

## Verification

A plugin author could read this doc and understand what's available, what's planned, and how to prepare.
