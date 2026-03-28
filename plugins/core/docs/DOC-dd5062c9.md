---
id: DOC-dd5062c9
type: doc
title: Shared Validation Engine
description: "How the shared validation engine works: a single TypeScript library in libs/validation/ consumed by three adapters (LSP, CLI, pre-commit). Schema-driven, plugin-aware, and the foundation for all artifact quality enforcement."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-dd5062c9
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
---

# Shared Validation Engine

## Overview

OrqaStudio has **one validation engine** used by three consumers. All artifact validation — frontmatter schemas, relationship types, status values, broken links, required fields — runs through a shared TypeScript library in `libs/validation/`. No consumer implements its own validation logic.

## Architecture

```text
                 Plugin schema.json files
                         │
                         ▼
              ┌──────────────────────┐
              │   libs/validation/   │
              │                      │
              │  Schema loading      │
              │  Frontmatter check   │
              │  Relationship check  │
              │  Reference integrity │
              │  Diagnostic output   │
              └───┬──────┬───────┬──┘
                  │      │       │
                  ▼      ▼       ▼
               LSP     CLI    Pre-commit
```

### The Engine

The validation engine is a pure TypeScript library with no runtime dependencies on the daemon, CLI, or any consumer. It exposes a validation API that takes:

- **Input**: A file path (or file content + metadata) and the set of loaded schemas
- **Output**: An array of diagnostic objects, each with file path, line range, severity (error/warning/info), and message

### Schema Source

Schemas are **plugin-provided**. Each artifact type directory (e.g., `.orqa/learning/rules/`, `.orqa/delivery/epics/`) contains a `schema.json` file in JSON Schema format. The validation engine discovers schemas via the plugin system's artifact configuration.

Plugins declare schemas in their `orqa-plugin.json` manifest under `provides.schemas`. The engine loads all registered schemas at startup and matches them to files by directory path.

## What It Validates

### Frontmatter Schema Validation

Every `.md` file in `.orqa/` has YAML frontmatter. The engine validates each field against the directory's `schema.json`:

| Check | Example Violation |
| ----- | ----------------- |
| Required fields present | Missing `id` or `status` in frontmatter |
| Valid enum values | `status: enabled` instead of `status: active` |
| Correct field types | `created: true` instead of `created: 2026-03-24` |
| No unknown fields | `priority: urgent` when schema doesn't define `priority` |

### Relationship Validation

Relationships in frontmatter (`relationships:` array) are checked against the plugin schema's `provides.relationships` definitions:

| Check | Example Violation |
| ----- | ----------------- |
| Valid relationship type | `type: synced-with` instead of `type: synchronised-with` |
| Valid source type | `implements` used on a knowledge artifact (only valid from docs) |
| Target exists | `target: KNOW-999999` when no such artifact exists |

### Referential Integrity

Cross-artifact references are checked for consistency:

| Check | Example Violation |
| ----- | ----------------- |
| Broken targets | A relationship pointing to a non-existent artifact ID |
| Missing inverses | A `synchronised-with` B but B doesn't list A |
| Orphaned artifacts | An epic referencing a task that doesn't reference back |

## Three Consumers

### 1. LSP Adapter (Real-Time)

The LSP mode (`orqa lsp`) calls the validation engine on every file save (or on-type, depending on configuration). Diagnostics are returned as LSP `Diagnostic` objects that the editor renders as:

- **Red squiggles** for errors (invalid status, missing required field)
- **Yellow squiggles** for warnings (missing inverse relationship)
- **Blue squiggles** for information (suggested improvements)

The LSP adapter also provides **completions** by reading schema enum values.

### 2. CLI Adapter (On-Demand)

The `orqa check` command runs the validation engine against all artifacts and outputs a human-readable report:

```text
ERROR  .orqa/learning/rules/RULE-abc123.md:5  Invalid status "enabled" — valid values: active, inactive
WARN   .orqa/delivery/tasks/TASK-def456.md:12  Missing inverse: EPIC-789 does not reference this task
```

Exit code 0 = clean, exit code 1 = errors found.

### 3. Pre-Commit Adapter (Gate)

The pre-commit hook (`.githooks/pre-commit`) runs the validation engine against **staged files only**. If any error-severity diagnostic is found, the commit is blocked.

The pre-commit adapter also logs violations to `.state/precommit-violations.jsonl` for the stability tracker (see [DOC-a16b7bc7](DOC-a16b7bc7)).

## Adding New Validation Rules

1. **If the rule is schema-based**: Add the constraint to the relevant `schema.json` file. The engine picks it up automatically.
2. **If the rule is cross-artifact**: Add a new check function in `libs/validation/` that the engine calls during its integrity pass.
3. **If the rule is plugin-specific**: Add the constraint to the plugin's schema definitions. The engine loads plugin schemas dynamically.

All three consumers automatically gain the new rule — there is nothing to update per-consumer.

## File Locations

| File | Purpose |
| ---- | ------- |
| `libs/validation/` | Shared validation engine library |
| `plugins/*/schema.json` (per artifact directory) | JSON Schema definitions for artifact types |
| `plugins/*/orqa-plugin.json` | Plugin manifest declaring schemas and relationship types |

## Related Documents

- [KNOW-dd5062c9](KNOW-dd5062c9) — Agent-facing knowledge pair for this documentation page
- [DOC-1f4aba8f](DOC-1f4aba8f) — Three-Layer Enforcement Model (validation engine is Layer 1 and Layer 3)
- [DOC-a16b7bc7](DOC-a16b7bc7) — Demoted Rule Stability Tracking (uses pre-commit violation logging)
