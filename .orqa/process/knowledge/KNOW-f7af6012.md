---
id: KNOW-f7af6012
type: knowledge
name: schema-lookup-before-write
title: "Query Artifact Schemas Before Writing Frontmatter"
description: "Agents must query plugin schemas via MCP to discover valid status values and field constraints before creating or modifying any artifact. Never rely on memorized or hardcoded status vocabularies."
layer: core
user-invocable: false
thinking-mode: governance
status: active
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: AGENT-1dab5ebe
    type: employed-by
    rationale: "Auto-generated inverse of employed-by relationship from AGENT-1dab5ebe" []
---
# Query Artifact Schemas Before Writing Frontmatter

## The Problem

Agents frequently write invalid `status` values in artifact frontmatter because they rely on memorized or assumed values. Common errors:

- `done` instead of `completed`
- `todo` instead of `captured`
- `in-progress` instead of `active`
- `draft` instead of `captured`

These errors cause schema validation failures at commit time and break graph integrity.

## The Root Cause

Status vocabularies are defined by **plugins** (`orqa-plugin.json`), not hardcoded anywhere. Different plugins may define different valid statuses for different artifact types. Memorizing status values creates a maintenance burden and drifts from the actual schema.

## The Pattern: Query Before Write

Before creating or modifying ANY artifact's frontmatter, agents MUST:

1. **Identify the artifact type** (task, epic, idea, rule, etc.)
2. **Query the schema** to discover valid field values
3. **Use only values the schema permits**

### How to Query

**Option 1 — MCP graph tools (preferred):**
```
graph_query({ type: "<artifact-type>" })
```
This returns schema metadata including valid statuses for the type.

**Option 2 — Read the plugin schema directly:**
Find the plugin that defines the artifact type, then read its `orqa-plugin.json`. Look for the `statusTransitions` section for that type. The keys of `statusTransitions` are the valid statuses.

**Option 3 — Read the directory schema:**
Read `schema.json` in the artifact's directory (e.g., `.orqa/delivery/tasks/schema.json`). The `status` field's `enum` array lists valid values.

## Where Schemas Live

| Source | Location | What It Defines |
|--------|----------|-----------------|
| Plugin schema | `plugins/<name>/orqa-plugin.json` | Artifact types, status transitions, relationship types |
| Directory schema | `<artifact-dir>/schema.json` | Field requirements, types, enum values |
| Core schema | `.orqa/core.json` | Base schema (intentionally empty — plugins extend it) |

## Key Principle

**Plugins are the single source of truth for artifact type definitions.** `core.json` is intentionally empty. All artifact types, their valid statuses, and their status transitions come from installed plugins.

## When This Applies

- Creating any new artifact (task, epic, idea, research, decision, lesson, rule, etc.)
- Updating the `status` field of any existing artifact
- Writing any enum-constrained frontmatter field

## Example Signals

- "Create a new task"
- "Update the status of EPIC-..."
- "Mark this as done" (agent must translate to the schema-valid equivalent)
- "Set status to in-progress" (agent must query schema to find the valid value)

## FORBIDDEN

- Writing a `status` value from memory without checking the schema
- Assuming all artifact types share the same status vocabulary
- Hardcoding status lists in agent definitions or knowledge artifacts
