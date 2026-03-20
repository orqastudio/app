---
id: DOC-SVE-5d832d1d
title: Svelte Development Guide
description: How to develop with Svelte 5 in OrqaStudio projects — runes, component patterns, testing, and coding standards.
category: how-to
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: KNOW-SVE-89d35141
    type: synchronised-with
  - target: KNOW-SVE-88f32b6a
    type: synchronised-with
  - target: KNOW-SVE-fd2b84c4
    type: synchronised-with
  - target: KNOW-SVE-90dd73ab
    type: synchronised-with
  - target: KNOW-1c708b68
    type: synchronised-with
  - target: KNOW-45b6ea05
    type: synchronised-with
  - target: KNOW-7e3c5886
    type: synchronised-with
  - target: KNOW-6a2bc391
    type: synchronised-with
  - target: KNOW-c40438c2
    type: synchronised-with
  - target: KNOW-344a1247
    type: synchronised-with
  - target: KNOW-6963d39f
    type: synchronised-with
  - target: KNOW-b29e340b
    type: synchronised-with
  - target: KNOW-e9baaf88
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
