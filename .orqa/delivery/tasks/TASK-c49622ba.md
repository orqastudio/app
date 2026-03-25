---
id: "TASK-c49622ba"
type: "task"
title: "Define tool system and permission model"
description: "Defined file tools, search tools, and governance tools with their permission model and execution patterns."
status: "completed"
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-02T00:00:00.000Z
acceptance:
  - "All tool types are catalogued with input/output schemas"
  - "Permission model is specified"
  - "Tool execution flows through the sidecar NDJSON protocol"
relationships:
  - target: "EPIC-fe3b5ad5"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

Defined the complete tool system including file tools, search tools, and governance tools with input/output schemas, a permission model, and execution flow through the sidecar NDJSON protocol.

## How

Catalogued each tool with its Zod-style input schema and result format, defined the permission tiers (always-allow, confirm, deny), and documented how tool calls route from the sidecar through Rust to the filesystem or governance engine.

## Verification

Tool system documentation lists all tool types with schemas, permission model is specified per tool category, and execution flow through the NDJSON protocol is documented.