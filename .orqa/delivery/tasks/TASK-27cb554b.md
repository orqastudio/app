---
id: TASK-27cb554b
type: task
title: "Migrate relationship validation hooks to daemon delegation"
description: "Replace two independent JavaScript relationship validation implementations (app hooks and plugin hooks) with daemon delegation. Both hooks load relationship vocabularies from plugin manifests — this should happen once in Rust."
status: captured
priority: P3
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "app/.githooks/validate-relationships.mjs delegates relationship validation to daemon"
  - "plugins/githooks/hooks/validate-relationships.mjs delegates relationship validation to daemon"
  - "No JavaScript-based relationship vocabulary loading remains in hooks"
  - "Daemon validates relationship types, target presence, and from/to type constraints"
  - "Pre-commit hooks correctly reject invalid relationship types"
relationships:
  - target: EPIC-5ab0265a
    type: delivers
    rationale: "Task delivers work to the deduplication epic"
---

## What

Two JavaScript implementations load valid relationship types from plugin manifests:
- App hooks (`app/.githooks/validate-relationships.mjs`, 168 lines) — loads into `Map` with rich metadata
- Plugin hooks (`plugins/githooks/hooks/validate-relationships.mjs`, 203 lines) — loads into `Set` with just keys

Both walk `plugins/` and `connectors/` directories reading `orqa-plugin.json` files. The Rust validation crate should own this logic.

## How

1. Ensure the Rust validation crate's relationship checking covers vocabulary validation (from plugin manifests)
2. Expose relationship validation via daemon `/parse` or a dedicated endpoint
3. Migrate both JS hooks to thin daemon adapters
4. Handle daemon-unavailable gracefully

## Files

- `app/.githooks/validate-relationships.mjs` — app hook
- `plugins/githooks/hooks/validate-relationships.mjs` — plugin hook
- `libs/validation/src/checks/` — Rust validation checks