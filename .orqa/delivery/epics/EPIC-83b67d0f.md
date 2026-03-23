---
id: EPIC-83b67d0f
type: epic
title: "Coding standards plugins — Svelte, Tauri, TypeScript, Rust with rule-driven enforcement"
description: "Created @orqastudio/plugin-svelte, @orqastudio/plugin-tauri (restructured to extend Rust), @orqastudio/plugin-typescript (new — tsconfig presets, ESLint base, config composition), and @orqastudio/plugin-rust (new — clippy/rustfmt/cargo-test, standards agent). Plugins provide linting/testing infrastructure. Rules define enforcement config. Organisation mode syncs standards with override tracking."
status: active
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: IDEA-2d7ac819
    type: realised-by
  - target: TASK-b784e910
    type: delivered-by
  - target: TASK-b38f4704
    type: delivered-by
  - target: TASK-a8c1a808
    type: delivered-by
  - target: TASK-daef57b3
    type: delivered-by
  - target: TASK-99bbc0a3
    type: delivered-by
  - target: TASK-e0427fb6
    type: delivered-by
  - target: TASK-a77f3e56
    type: delivered-by
  - target: TASK-77ec1aa5
    type: delivered-by
  - target: TASK-e5d1af77
    type: delivered-by
  - target: TASK-ae1fb7e5
    type: delivered-by
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---

# EPIC-83b67d0f: Coding Standards Plugins

## Goal

Replace standalone eslint-config and test-config libraries with plugin-driven coding standards. Rules define enforcement, plugins provide tooling, organisation mode syncs standards.

## Deliverables

1. `@orqastudio/plugin-svelte` — ESLint, svelte-check, Vitest
2. `@orqastudio/plugin-tauri` — restructured to extend the Rust plugin instead of duplicating Rust tooling
3. `@orqastudio/plugin-typescript` — **new**: tsconfig presets, ESLint base config, config composition utilities
4. `@orqastudio/plugin-rust` — **new**: clippy, rustfmt, cargo-test infrastructure, standards assessment agent
5. Updated enforcement schema (plugin/tool/config arrays)
6. Config generator (rules → tool config files) — started, not yet end-to-end
7. Core installer agent
8. `orqa check` integration
9. Official registry entries

## Remaining

- Config generator end-to-end (rules → generated tool config files)
- `orqa check` integration with plugin executors
- Official registry entries for all four plugins