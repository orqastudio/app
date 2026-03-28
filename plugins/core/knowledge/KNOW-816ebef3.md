---
id: KNOW-816ebef3
type: knowledge
name: artifact-creation
description: "How to create valid OrqaStudio artifacts — frontmatter requirements, relationship rules, schema compliance, and common patterns."
user-invocable: false
---

# Artifact Creation

## Frontmatter Requirements

Every artifact MUST have YAML frontmatter:

```yaml
---
id: TYPE-NNN
type: typename
title: "Human-readable title"
status: captured
created: YYYY-MM-DD
updated: YYYY-MM-DD
relationships:
  - target: RELATED-ID
    type: relationship-type
---
```

### Required Fields

| Field | Format | Notes |
| ----- | ------ | ----- |
| `id` | `PREFIX-NNN` | Unique, matches idPrefix from schema |
| `type` | string | Must match a type from core.json or plugin schemas |
| `status` | string | One of the canonical statuses (see table below) |

## Artifact Status — Query the Schema (NON-NEGOTIABLE)

Valid statuses are defined by **plugin schemas** (`statusTransitions` in `orqa-plugin.json`). Do NOT memorize or hardcode status values. Instead, discover them at runtime:

### How to Discover Valid Statuses

1. **Via MCP**: `graph_query({ type: "<artifact-type>" })` — returns schema metadata including valid statuses
2. **Via plugin file**: Read the plugin's `orqa-plugin.json` and find the `statusTransitions` for the artifact type
3. **Via schema.json**: Read the `schema.json` in the artifact's directory for the `status` enum

### Why Runtime Discovery

- Plugins define status vocabularies — different plugins may define different valid statuses
- Hardcoded status lists create maintenance burden and drift from the actual schema
- The plugin schema is the single source of truth; everything else is a stale copy

### Common Mistakes

Agents frequently use statuses that feel natural but are not in any plugin schema. Before writing any `status` field, query the schema. Common errors include using `done` instead of `completed`, `todo` instead of `captured`, `in-progress` instead of `active`, and `draft` instead of `captured`.

### Common Optional Fields

| Field | Format | Notes |
| ----- | ------ | ----- |
| `title` | string | Human-readable name |
| `description` | string | Brief description |
| `created` | `YYYY-MM-DD` | Creation date |
| `updated` | `YYYY-MM-DD` | Last update date |
| `relationships` | array | Typed connections to other artifacts |

## ID Allocation

Check existing artifacts to find the next available ID:

```bash
ls .orqa/delivery/tasks/ | sort -t- -k2 -n | tail -1
```

## Relationship Protocol

1. **Always bidirectional** — write both forward and inverse
2. **Read the target** — verify it exists before creating the relationship
3. **Check type constraints** — some relationships only apply between specific types
4. **Update both files** — the source AND the target artifact

Example: Creating a task that delivers to an epic:

```yaml
# In TASK-44bd295d.md
relationships:
  - target: EPIC-9b58fdcb
    type: delivers
```

```yaml
# In EPIC-9b58fdcb.md — add the inverse
relationships:
  - target: TASK-44bd295d
    type: delivered-by
```

## Common Patterns

### New Task

```yaml
---
id: TASK-NNN
type: task
title: "Task title"
status: captured
created: YYYY-MM-DD
updated: YYYY-MM-DD
relationships:
  - target: EPIC-NNN
    type: delivers
---

# TASK-NNN: Task Title

## Acceptance Criteria

1. ...
2. ...
```

### New Epic

```yaml
---
id: EPIC-NNN
type: epic
title: "Epic title"
status: captured
created: YYYY-MM-DD
updated: YYYY-MM-DD
relationships:
  - target: MS-NNN
    type: fulfils
  - target: TASK-NNN
    type: delivered-by
---
```

### New Decision

```yaml
---
id: AD-NNN
type: decision
title: "Decision title"
status: active
created: YYYY-MM-DD
updated: YYYY-MM-DD
relationships:
  - target: PILLAR-NNN
    type: grounded
---
```

## Validation

After creating artifacts:

1. Check frontmatter against `schema.json` in the target directory
2. Verify all relationship targets exist
3. Verify all inverses are present on target artifacts
4. Run `orqa enforce` to check graph integrity
