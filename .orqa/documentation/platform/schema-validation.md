---
id: DOC-32c6f5a6
type: doc
name: Schema Validation Reference
category: reference
status: active
relationships:
  - target: KNOW-b08d355c
    type: synchronised-with
---

# Schema Validation Reference

OrqaStudio validates the artifact graph using schemas, not custom code. The platform configuration (`core.json`) and plugin manifests (`orqa-plugin.json`) define what artifact types exist, how they connect, and what constraints apply. Both the CLI and app validators evaluate these schemas generically.

## The Schema Files

### Platform Configuration

**Location:** `libs/types/src/platform/core.json`

This file ships with every OrqaStudio installation and defines:

**Artifact Types** — 12 core types:

| Type | Prefix | Purpose |
|------|--------|---------|
| vision | VISION | The project's north star |
| pillar | PILLAR | Foundational principles that uphold the vision |
| persona | PERSONA | Target users the project serves |
| idea | IDEA | Seeds that spawn all other work |
| decision | AD | Architecture choices that drive work and govern rules |
| rule | RULE | Enforceable governance derived from decisions and lessons |
| lesson | IMPL | Knowledge learned from doing work |
| skill | SKILL | Agent-facing how-to instructions |
| agent | AGENT | Roles that observe artifacts and employ skills |
| doc | DOC | Human-facing documentation |
| pivot | PIVOT | Records of foundational changes |
| bug | BUG | Functional and display issues |

**Relationships** — 19 typed connections, each with:
- `key` / `inverse` — the verb pair (e.g. `grounded` / `grounded-by`)
- `from` / `to` — which artifact types can participate
- `semantic` — which category it belongs to
- `constraints` — validation rules (required, cardinality, status rules)

### Plugin Manifests

**Location:** `plugins/{name}/orqa-plugin.json`

Plugins add more artifact types, relationships, and constraints. The software project plugin adds milestones, epics, tasks, research, wireframes, bugs, and delivery relationships.

All schemas are merged at runtime. A plugin cannot override a platform relationship — conflicts are detected and resolved via the alias system.

## Validation Rules

### Every rule comes from the schema

There is no custom validation code. Both validators (TypeScript CLI and Rust app) read the merged schemas and evaluate them generically:

1. **Type validation** — does the `type` field match a registered artifact type?
2. **Required fields** — are `id`, `type`, `status` present? (from `frontmatter.required`)
3. **Status validity** — is the status value one of the 12 canonical statuses?
4. **Status transitions** — is this transition allowed? (from `statusTransitions`)
5. **Relationship key** — is this relationship key registered?
6. **Target exists** — does the target artifact ID exist in the graph?
7. **Inverse exists** — does the target have the inverse edge back?
8. **Type constraints** — does the source type match `from` and target type match `to`?
9. **Required relationships** — if `constraints.required`, does the source have enough instances?
10. **Cardinality** — if `constraints.maxCount`, are there too many instances?
11. **Status rules** — if `constraints.statusRules`, should a status transition be proposed?

### Status Rules

Some relationships trigger automatic status proposals based on the graph state:

| Relationship | Condition | Proposal |
|---|---|---|
| `depends-on` | Any target not completed | Source task → `blocked` |
| `depends-on` | All targets completed | Source task → `ready` |
| `delivers` | All source tasks completed | Target epic → `review` |
| `fulfils` | All source epics completed | Target milestone → `review` |

These are not hardcoded — they're defined in the relationship's `constraints.statusRules` array. Adding a new auto-transition is just adding a JSON entry.

## Adding New Validation

To add a new validation rule:

1. **Structural** — add to the artifact schema's `frontmatter` arrays
2. **Relationship** — add `constraints` to the relationship definition
3. **Status-dependent** — add a `statusRules` entry

Then both validators pick it up automatically. No code changes needed.

## Schema Location Summary

| What | Where |
|---|---|
| Platform types + relationships | `libs/types/src/platform/core.json` |
| Plugin types + relationships | `plugins/{name}/orqa-plugin.json` |
| TypeScript type definitions | `libs/types/src/plugin.ts` (RelationshipType, RelationshipConstraints, etc.) |
| Validator (CLI) | `libs/cli/src/validator/` |
| Validator (Rust) | `app/backend/src-tauri/src/domain/artifact_graph.rs` |
