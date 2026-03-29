---
id: KNOW-dd5062c9
type: knowledge
title: Shared Validation Engine
domain: methodology/governance
summary: "OrqaStudio has **one validation engine** with **three consumers**. All artifact validation — frontmatter schemas, relationship types, status values, broken links, required fields — runs through a shared library."
description: |
  How the shared validation engine works: a single library in libs/validation/ consumed
  by three adapters (LSP real-time, CLI on-demand, pre-commit gate). Schema-driven from
  plugin schema.json files. Use when: adding validation rules, modifying artifact schemas,
  building enforcement tooling, or debugging validation failures.
tier: always
status: active
created: 2026-03-24
updated: 2026-03-24
category: architecture
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-dd5062c9
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
---

## Purpose

OrqaStudio has **one validation engine** with **three consumers**. All artifact validation — frontmatter schemas, relationship types, status values, broken links, required fields — runs through a shared library. No consumer implements its own validation logic.

---

## Architecture

```text
Plugin schema.json files
        │
        ▼
┌─────────────────────┐
│  libs/validation/    │  ← Single validation engine
│  (Rust crate)        │
└────┬───────┬────────┘
     │       │        │
     ▼       ▼        ▼
   LSP     CLI      Pre-commit
  (real-   (orqa    (.githooks/
   time)   check)   pre-commit)
```

### The Engine (libs/validation/)

The validation engine is a Rust crate that:

1. **Reads plugin schemas** — each artifact type directory has a `schema.json` (JSON Schema format)
2. **Validates frontmatter** — checks YAML frontmatter against the schema for required fields, valid enum values, correct types
3. **Validates relationships** — checks that relationship types are valid for the source artifact type (per plugin schema `provides.relationships`)
4. **Checks referential integrity** — broken `target` references, missing inverse relationships
5. **Reports diagnostics** — returns structured diagnostic objects with file path, line number, severity, message

### Three Consumers

| Consumer | When It Runs | Response to Violations |
| ---------- | ------------- | ---------------------- |
| **LSP adapter** (`orqa lsp`) | Real-time in the editor, on every file save | Red squiggles, warnings, completions |
| **CLI adapter** (`orqa check`) | On demand from terminal or `make check` | Human-readable error report, exit code |
| **Pre-commit adapter** (`.githooks/pre-commit`) | On every `git commit` | Blocks commit if errors found |

---

## What It Validates

| Check | Example |
| ------- | --------- |
| **Required fields** | `id`, `type`, `title`, `status` present in frontmatter |
| **Valid statuses** | `status: active` not `status: enabled` (per schema enum) |
| **Valid relationship types** | `synchronised-with` not `synced-with` (per plugin schema) |
| **Broken references** | `target: KNOW-999999` where that ID doesn't exist |
| **Missing inverses** | A `synchronised-with` B but B doesn't `synchronised-with` A |
| **Type constraints** | `created` is a date, `relationships` is an array |

## Schema Source

Schemas are **plugin-provided**. Each plugin's `orqa-plugin.json` declares `provides.schemas` and the artifact type directories contain `schema.json` files. The validation engine discovers schemas via the plugin system — it never hardcodes valid values.

---

## Agent Actions

| Situation | Action |
| ----------- | -------- |
| Adding a new artifact type | Create a `schema.json` in the type directory. The validation engine picks it up automatically. |
| Adding a new relationship type | Add it to the plugin's `provides.relationships` array in `orqa-plugin.json`. |
| Adding a new status value | Add it to the status enum in the relevant `schema.json`. |
| Validation failure on commit | Read the error, fix the frontmatter. Never bypass with `--no-verify`. |
| LSP shows red squiggle | Fix immediately. Do not defer. |

---

## FORBIDDEN

- Implementing validation logic in a consumer (LSP, CLI, or pre-commit) instead of the shared engine
- Hardcoding valid statuses, relationship types, or required fields outside of schema.json files
- Bypassing validation with `--no-verify` on commits
- Adding validation rules that don't trace back to a schema.json definition
