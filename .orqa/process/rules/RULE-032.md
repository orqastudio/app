---
id: RULE-032
title: Artifact Schema Compliance
description: "Every artifact's YAML frontmatter must validate against the JSON Schema defined in its artifact directory's schema.json file."
status: active
created: 2026-03-10
updated: 2026-03-13
relationships:
  - target: AD-034
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-034
  - target: AD-023
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-023
  - target: DOC-036
    type: documented-by
  - target: AGENT-008
    type: observed-by
---
Every artifact in `.orqa/` must have YAML frontmatter that validates against the JSON Schema in its directory's `schema.json` file. Fields not defined in the schema are rejected. Required fields must be present. Enum fields must use valid values.

## Source of Truth

Each artifact type directory contains a `schema.json` file (JSON Schema format) that is the single source of truth for:

- Which fields are required vs optional
- Field types and constraints
- Valid values for enum fields (status, priority, layer)
- Whether additional properties are allowed

| Directory | Schema |
|-----------|--------|
| `.orqa/process/pillars/` | `schema.json` |
| `.orqa/delivery/milestones/` | `schema.json` |
| `.orqa/delivery/epics/` | `schema.json` |
| `.orqa/delivery/tasks/` | `schema.json` |
| `.orqa/delivery/ideas/` | `schema.json` |
| `.orqa/delivery/research/` | `schema.json` |
| `.orqa/process/lessons/` | `schema.json` |
| `.orqa/process/decisions/` | `schema.json` |
| `.orqa/process/rules/` | `schema.json` |

## Schema Discovery

Schemas are discovered via `.orqa/project.json`'s `artifacts` array. The validator walks the config tree, finds which artifact directory a file belongs to, and loads `schema.json` from that directory. Adding a new artifact type only requires:

1. Create the directory under `.orqa/`
2. Add a `schema.json` defining the frontmatter shape
3. Register the path in `project.json`'s `artifacts` array

## Enforcement

1. **Pre-commit hook** — `.githooks/pre-commit` calls `.githooks/validate-artifacts.sh`, which delegates to `.githooks/validate-schema.mjs` (Node + ajv) on staged `.orqa/**/*.md` files. Validation failures block the commit.
2. **Agent self-compliance** — agents read the schema before creating or modifying artifacts
3. **Rust backend** (future) — the artifact scanner validates frontmatter using the `jsonschema` crate against the same `schema.json` files
4. **TypeScript frontend** (future) — the artifact editor validates on save using `ajv` against the same `schema.json` files

## Cross-Language Validation

Schemas use JSON Schema (draft 2020-12 compatible), validated by:

| Context | Library |
|---------|---------|
| Pre-commit hook (Node) | `ajv` v8 + `ajv-formats` |
| Rust backend (future) | `jsonschema` crate |
| TypeScript frontend (future) | `ajv` v8 |

All three share the same `schema.json` files — one source of truth, three consumers.

## Related Rules

- [RULE-004](RULE-004) (artifact-lifecycle) — status transitions and promotion gates
- [RULE-003](RULE-003) (artifact-config-integrity) — config paths must match disk
- [RULE-027](RULE-027) (structure-before-work) — artifacts must exist before implementation
