---
id: TASK-e5435ce9
title: Define tool system and permission model
description: "Defined file tools, search tools, and governance tools with their permission model and execution patterns."
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - All tool types are catalogued with input/output schemas
  - Permission model is specified
  - Tool execution flows through the sidecar NDJSON protocol
relationships:
  - target: EPIC-f132980b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-8dd8819a
    type: depended-on-by
---
## What

Defined the complete tool system including file tools, search tools, and governance tools with input/output schemas, a permission model, and execution flow through the sidecar NDJSON protocol.

## How

Catalogued each tool with its Zod-style input schema and result format, defined the permission tiers (always-allow, confirm, deny), and documented how tool calls route from the sidecar through Rust to the filesystem or governance engine.

## Verification

Tool system documentation lists all tool types with schemas, permission model is specified per tool category, and execution flow through the NDJSON protocol is documented.
