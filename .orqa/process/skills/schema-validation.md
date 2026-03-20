---
id: SKILL-b08d355c
type: skill
name: Schema Validation
status: active
relationships:
  - target: DOC-32c6f5a6
    type: synchronised-with
  - target: DOC-a1b2c3d4
    type: synchronised-with

---

# Schema Validation

OrqaStudio uses schema-driven validation. There is no custom business logic in the validators — both the CLI (TypeScript) and the app (Rust) evaluate the same schemas generically. The schema IS the rule.

## Where schemas live

**Platform schemas** — `libs/types/src/platform/core.json`
- Artifact types (12): vision, pillar, persona, idea, decision, rule, lesson, skill, agent, doc, pivot, bug
- Relationships (19): each with from/to type constraints, semantic category, and optional validation constraints
- Semantic categories (6): foundation, lineage, governance, knowledge-flow, observation, synchronisation

**Plugin schemas** — `plugins/{name}/orqa-plugin.json`
- Additional artifact types (schemas array)
- Additional relationships (relationships array with same constraint format)
- Status transitions per artifact type
- Delivery hierarchy config

**Both are merged at runtime** — the validator loads core.json + all plugin manifests, merges into one vocabulary, then validates every artifact against it.

## What the validator checks

All checks are derived from the schema — no hardcoded rules:

### Structural checks (from type definitions)
- **Required frontmatter fields**: id, type, status (from schema `frontmatter.required`)
- **Valid artifact type**: type field matches a registered type key
- **Valid status**: status field matches canonical statuses
- **Status transitions**: transition is allowed per `statusTransitions` config

### Relationship checks (from relationship definitions)
- **Valid relationship key**: relationship type matches a registered key
- **Target exists**: relationship target ID exists in the graph
- **Inverse exists**: bidirectional inverse edge present on target
- **From/to type match**: source and target artifact types match the `from`/`to` arrays
- **Required relationships**: if `constraints.required = true`, the source type must have at least `minCount` instances
- **Max count**: if `constraints.maxCount` set, no more than that many instances

### Status rules (from relationship `constraints.statusRules`)
- **Computed transitions**: when relationship targets meet a condition (all-completed, any-not-completed), propose a status change on the source or target
- **Example**: task with `depends-on` targets not completed → propose `blocked`
- **Example**: epic with all `delivered-by` tasks completed → propose `review`

## How to add a new validation rule

1. **If it's a structural rule** — add it to the artifact schema's `frontmatter` or `statusTransitions`
2. **If it's a relationship rule** — add `constraints` to the relationship definition in core.json or the plugin manifest
3. **If it's a status-dependent rule** — add a `statusRules` entry to the relationship's constraints

You never write validation code. You write schema.

## Constraint schema reference

```json
{
  "constraints": {
    "required": true,
    "minCount": 1,
    "maxCount": 1,
    "requireInverse": true,
    "statusRules": [
      {
        "evaluate": "source",
        "condition": "all-targets-in",
        "statuses": ["completed"],
        "proposedStatus": "review",
        "description": "Human-readable explanation"
      }
    ]
  }
}
```

### `evaluate`
- `"source"` — evaluate the artifact that HAS this relationship
- `"target"` — evaluate the artifact that is TARGETED by this relationship (inverse direction)

### `condition`
- `"all-targets-in"` — all related targets have a status in the `statuses` list
- `"any-target-in"` — at least one target has a status in the list
- `"no-targets-in"` — no targets have a status in the list

### `proposedStatus`
The status to propose for the evaluated artifact if the condition is met.

## Running validation

```bash
orqa validate              # CLI — full integrity check
orqa validate --json       # JSON output for programmatic use
```

In the app, the Rust validator runs the same checks against the same schemas via `run_integrity_scan`.
