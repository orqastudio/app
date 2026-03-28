---
id: "TASK-9f3a2a69"
type: "task"
title: "Dead code removal"
description: "Find and remove unused functions, imports, components, and modules across Rust and TypeScript codebases."
status: ready
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "Rust dead code scan completed (cargo clippy dead_code, unused_imports warnings)"
  - "TypeScript dead code scan completed (unused exports, unreachable imports)"
  - "Svelte unused component scan completed"
  - "All confirmed dead code removed in a single commit"
  - "make check passes after removal"
  - "No false positives removed (public API, conditional compilation, plugin interfaces verified)"
relationships:
  - target: "EPIC-e24086ed"
    type: "delivers"
---

## What

Identify and remove dead code that accumulated during iterative development. Dead code increases cognitive load and can mask real issues in linter output.

## How

### Rust

1. Run `cargo clippy` with `dead_code` and `unused_imports` lints enabled
2. Check for unused `pub` functions by searching for callers via `search_regex`
3. Verify conditional compilation gates before removing anything behind `#[cfg(...)]`

### TypeScript/Svelte

1. Run ESLint with `no-unused-vars` and `no-unused-imports` rules
2. Search for exported functions/types with zero import sites
3. Check for Svelte components with zero usage across all `.svelte` files

### Verification

1. Every removal is verified to have zero callers/importers
2. `make check` passes after all removals
3. No public API surface is accidentally removed (check exports used by plugins)
