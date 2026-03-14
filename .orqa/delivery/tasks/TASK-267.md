---
id: TASK-267
title: Document plugin architecture and SDK extraction plan
description: Document the plugin system architecture and plan for component library / SDK extraction.
status: done
created: 2026-03-12
updated: 2026-03-12
epic: EPIC-057
depends-on:
  - TASK-266
assignee: AGENT-007
acceptance:
  - "Document covers: current plugin capabilities, component library extraction plan, view registration API, theme tokens"
  - Built-in vs plugin boundary documented with decision framework
  - Document lives in .orqa/documentation/development/
relationships:
  - target: EPIC-057
    type: belongs-to
    rationale: Task belongs to this epic
---

## What

Document the plugin architecture including the path to SDK extraction for dynamic plugin views.

## How

1. Map current plugin capabilities (CLI companion plugin, MCP integration)
2. Identify components that should be extracted to a shared SDK
3. Design view registration API concept
4. Document theme token system for plugin authors
5. Reference [RES-046](RES-046) built-in vs plugin framework

## Verification

A plugin author could read this doc and understand what's available, what's planned, and how to prepare.
