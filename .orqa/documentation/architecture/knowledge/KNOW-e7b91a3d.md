---
id: KNOW-e7b91a3d
type: knowledge
status: active
title: "Artifact Lifecycle and Relationship Flow"
description: "How artifacts are created, validated, related, and flow through methodology stages — the connective tissue of the governance model"
tier: always
created: 2026-03-29
roles: [governance-steward, writer, implementer, reviewer]
paths: [.orqa/, engine/graph/, engine/artifact/]
tags: [governance, artifacts, lifecycle, relationships, validation]
relationships:
  - type: synchronised-with
    target: DOC-fd3edf48
---

# Artifact Lifecycle and Relationship Flow

## What an Artifact Is

An artifact is a Markdown file with YAML frontmatter stored in `.orqa/`. It is the atomic unit of governance data.

**Required frontmatter fields (all types):**

- `id` — with correct prefix for the artifact type (e.g., `TASK-`, `KNOW-`, `DOC-`)
- `type` — the artifact type name (e.g., `task`, `knowledge`, `doc`)
- `title` — NOT `name` (use `title`)
- `description` — one-line purpose statement
- `status` — valid value from the type's state machine
- `created` — ISO date
- `updated` — ISO date

## ID Prefix by Type

| Artifact Type | ID Prefix | Stage |
| -------------- | ---------- | ------- |
| `doc` | `DOC-` | documentation |
| `knowledge` | `KNOW-` | documentation |
| `epic` | `EPIC-` | implementation |
| `task` | `TASK-` | implementation |
| `milestone` | `MS-` | implementation |
| `lesson` | `IMPL-` | learning |
| `rule` | `RULE-` | learning |
| `principle-decision` | `PD-` | learning |
| `planning-decision` | `PLANNING-` | planning |
| `persona` | `PERSONA-` | discovery |
| `pillar` | `PILLAR-` | discovery |
| `vision` | `VISION-` | discovery |
| `wireframe` | `WIREFRAME-` | discovery or planning |

## Relationship Storage Model

Relationships are **forward-only** — declared in the source artifact's frontmatter. The graph engine computes inverses automatically.

```yaml
# In TASK-abc123.md
relationships:
  - type: delivers
    target: EPIC-def456
```

The task declares it delivers the epic. The epic does NOT declare `delivered-by: task`. The graph computes that inverse on demand.

**This is a load-bearing constraint:** forward-only storage means relationship declarations have a clear owner and direction. Inverses are never manually written.

## Valid Relationship Types

Relationship types are declared by plugins in `orqa-plugin.json` under `provides.relationships`. Only types from the composed schema are valid. The current count is 41 relationship types across all plugins. Types cannot be invented — they must be declared in a plugin schema.

Key relationship groupings:

- **Delivery flow:** `delivers`, `tracks`, `drives`
- **Knowledge flow:** `informs`, `teaches`, `guides`, `documents`
- **Governance:** `governs`, `enforces`, `codifies`, `promoted-to`
- **Foundation:** `upholds`, `grounded`, `benefits`, `serves`
- **Synchronisation:** `synchronised-with` (pairs DOC + KNOW artifacts)

## Artifact Validation

Validation runs at three points in the lifecycle:

1. **Write time (LSP)** — real-time feedback in editor: invalid frontmatter fields, invalid status values, wrong relationship types, broken references
2. **On-demand (CLI)** — `orqa check` for human-readable reports
3. **Commit time (pre-commit)** — hard gate: blocked if any error-severity diagnostic found

Knowledge artifacts have additional constraints: 500-2,000 tokens, atomic (one sub-topic), self-contained.

## DOC + KNOW Pairing

Every documentation page must have a paired knowledge artifact:

| Artifact | Audience | Format |
| ---------- | ---------- | -------- |
| **DOC** | Human developers | Full explanation, context, examples, prose |
| **KNOW** | AI agents | Structured tables, rules, decision trees, forbidden patterns |

The pair is linked by a `synchronised-with` relationship. When one is updated, the other must be updated in the same commit.

Both live in the same directory structure: DOC in `documentation/<topic>/`, KNOW in `documentation/<topic>/knowledge/`.

## Relationship Flow Across Workflows

Relationships define flow both within a workflow and between workflows:

- **Within:** task delivers epic, research informs decision
- **Across:** discovery outputs feed planning inputs, planning decisions drive implementation epics

The graph engine computes the full traceability chain from discovery vision through to implementation tasks and learning lessons. No workflow is isolated.
