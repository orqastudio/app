---
id: DOC-fd1d12bb
type: doc
title: Svelte Development Guide
description: How to develop with Svelte 5 in OrqaStudio projects — runes, component patterns, testing, and coding standards.
category: how-to
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: KNOW-4260613a
    type: synchronised-with
  - target: KNOW-6cfacbb2
    type: synchronised-with
  - target: KNOW-3642842e
    type: synchronised-with
  - target: KNOW-be54e4de
    type: synchronised-with
  - target: KNOW-50382247
    type: synchronised-with
  - target: KNOW-a1a195c1
    type: synchronised-with
  - target: KNOW-96aaa407
    type: synchronised-with
  - target: KNOW-37496474
    type: synchronised-with
  - target: KNOW-df3c489e
    type: synchronised-with
  - target: KNOW-abb08445
    type: synchronised-with
  - target: KNOW-8cc0f5e4
    type: synchronised-with
  - target: KNOW-c4d3e52b
    type: synchronised-with
  - target: KNOW-5704b089
    type: synchronised-with
---

# Svelte Development Guide

This plugin provides TypeScript and Svelte development infrastructure for OrqaStudio projects.

## What It Provides

- **ESLint** — TypeScript and Svelte linting, configured from coding standards rules
- **svelte-check** — Svelte-specific type checking
- **Vitest** — Unit and component testing with @testing-library/svelte
- **Config generation** — tool configs generated from OrqaStudio rules, not hand-edited

## Coding Standards

Coding standards are defined as OrqaStudio rules, not config files. The rule contains enforcement entries that specify ESLint rules, svelte-check flags, and Vitest settings. The plugin generates the actual config files from these entries.

To change a standard: edit the rule. The config regenerates automatically.

## Svelte 5 Runes

Svelte 5 uses runes exclusively:

| Rune | Purpose |
|------|---------|
| `$state()` | Reactive state |
| `$derived()` | Computed values |
| `$derived.by(() => ...)` | Computed values with logic |
| `$effect()` | Side effects |
| `$props()` | Component props |
| `$bindable()` | Two-way binding props |

No Svelte 4 patterns (no `$:`, no `export let`, no stores API).

## Testing

Tests use Vitest + @testing-library/svelte. Test files live alongside the code they test. Test behaviour, not implementation.

## Sub-Project Overrides

In organisation mode, coding standards rules propagate from the org level. A sub-project can override specific rules with a rationale — tracked as a governance decision, not a silent config change.
