---
id: IDEA-a3f7c912
type: discovery-idea
status: captured
title: Single schema source generating both TS and Rust types
created: 2026-03-28T16:30:00.000Z
relationships:
  - type: benefits
    target: PERSONA-477971bf
---

# libs/schema/ as Single Source of Truth for Type Generation

## Context

TypeScript types (libs/types/) and Rust types (engine/types/) are currently maintained independently. This creates drift risk — the same artifact structures, relationship types, and frontmatter fields are defined in two languages with no mechanical guarantee of consistency.

## Proposal

Create `libs/schema/` containing JSON Schema definitions as the single source of truth:

```text
libs/schema/           # JSON Schema definitions (source of truth)
  platform/            # Core platform schemas (artifact types, relationships, etc.)
  artifacts/           # Artifact type frontmatter schemas
  generate.mjs         # Generates both TS and Rust from schemas

libs/types/            # Generated TypeScript interfaces + enums
engine/types/          # Generated Rust structs + hand-written behavior (traits, impls)
```

## Generation pipeline

1. Edit schema in `libs/schema/`
2. Run `node libs/schema/generate.mjs` (or `orqa generate-types`)
3. Outputs TS interfaces to `libs/types/src/generated/`
4. Outputs Rust structs to `engine/types/src/generated/`
5. Hand-written Rust behavior (trait impls, methods) imports from generated structs

## Enforcement

- Git hook: hash schemas, compare with generated output hashes
- If schema changed but generated types weren't regenerated → block commit
- Same hash-based verification model as DOC/KNOW sync

## Tools

- TS generation: existing `generate-types.mjs` (already works)
- Rust generation: `typify` crate or custom codegen from JSON Schema
- Both produce deterministic output from the same input

## Priority

Phase 10 or post-migration. The infrastructure for TS generation exists; Rust generation needs building.

## Interim Step (Accepted)

During migration, the settings navigation in `SettingsCategoryNav.svelte` is maintained as **static config** grouped by architecture section (Methodology / Sidecar / Connector / Plugins). This is the accepted interim step.

True schema-driven UI generation — where installing a new plugin automatically adds its settings section to the nav without any code change — is post-migration work. When `libs/schema/` is built out, the settings nav can derive its structure from plugin-declared schemas rather than the hardcoded `projectGroups` array.
