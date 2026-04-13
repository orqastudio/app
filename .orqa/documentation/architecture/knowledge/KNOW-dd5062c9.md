---
id: KNOW-dd5062c9
type: knowledge
status: active
title: "Shared Validation Engine"
description: "One validation engine in engine/validation/ consumed by three adapters (LSP, CLI, pre-commit) — schema-driven, plugin-aware, foundation for all artifact quality enforcement"
tier: always
created: 2026-03-29
roles: [implementer, reviewer, governance-steward]
paths: [engine/validation/, engine/enforcement/]
tags: [architecture, validation, lsp, cli, pre-commit, schema]
relationships:
  - type: synchronised-with
    target: DOC-dd5062c9
---

# Shared Validation Engine

## Architecture

OrqaStudio has **one validation engine** used by three consumers. All artifact validation — frontmatter schemas, relationship types, status values, broken references, required fields — runs through a shared engine. No consumer implements its own validation logic.

```text
Plugin schema.json files
        │
        ▼
  Shared validation engine (engine/validation/ or engine/validation/)
        │
   ┌────┴────┬────────┐
   ▼         ▼        ▼
  LSP     CLI      Pre-commit
```

## What It Validates

### Frontmatter Schema Validation

| Check | Example Violation |
| ------- | ------------------- |
| Required fields present | Missing `id` or `status` |
| Valid enum values | `status: enabled` instead of `status: active` |
| Correct field types | `created: true` instead of a date |
| No unknown fields | `priority: urgent` when schema doesn't define it |

### Relationship Validation

| Check | Example Violation |
| ------- | ------------------- |
| Valid relationship type | `type: synced-with` instead of `synchronised-with` |
| Valid source/target types | `implements` used on a knowledge artifact (wrong type) |
| Target exists | `target: KNOW-999999` when no such artifact exists |

### Referential Integrity

| Check | Example Violation |
| ------- | ------------------- |
| Broken targets | Relationship pointing to non-existent artifact ID |
| Missing inverses | A `synchronised-with` B but B doesn't list A |
| Orphaned artifacts | Epic referencing task that doesn't reference back |

## Three Consumers

### 1. LSP Adapter (Real-Time)

`orqa lsp` calls the engine on every file save. Returns LSP `Diagnostic` objects the editor renders as squiggles. Also provides completions by reading schema enum values.

### 2. CLI Adapter (On-Demand)

`orqa check` runs engine against all artifacts, outputs human-readable report.
Exit code 0 = clean, exit code 1 = errors found.

### 3. Pre-Commit Adapter (Gate)

Pre-commit hook runs engine against **staged files only**. If any error-severity diagnostic found, commit is blocked. Also logs violations to `.state/precommit-violations.jsonl` for stability tracking.

## Schema Source: Plugin-Provided

Schemas are plugin-provided. Each plugin declares schemas in `orqa-plugin.json` under `provides.schemas`. The engine loads all registered schemas at startup and matches them to files by directory path and artifact type.

The engine never hardcodes valid values. Adding a new status to a plugin schema makes it immediately valid across all three enforcement layers.

## Adding New Validation Rules

1. **Schema-based rule:** Add constraint to the relevant `schema.json` file. Engine picks it up automatically.
2. **Cross-artifact rule:** Add check function to the shared validation crate.
3. **Plugin-specific rule:** Add constraint to plugin's schema definitions.

All three consumers automatically gain the new rule — nothing to update per-consumer.
