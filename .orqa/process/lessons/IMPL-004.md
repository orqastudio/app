---
id: IMPL-004
title: Svelte 5 derived requires .by() for function expressions
description: "Use $derived(expr) for simple expressions or $derived.by(() => expr) for function bodies. Never use $derived(() => expr) — it creates a derived value that is the function itself, causing infinite re-render loops.\n"
status: active
created: 2026-03-07
updated: 2026-03-07
maturity: understanding
recurrence: 1
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Correct reactive patterns ensure structural UI consistency"
  - target: PILLAR-001
    type: informs
---
## What Happened

Using `$derived(() => expression)` instead of `$derived(expression)` or `$derived.by(() => expression)` caused components to enter infinite re-render loops, hanging the app window.

## Why It Was Unexpected

The syntax `$derived(() => ...)` looks valid — it's a function returning a value. But in Svelte 5, `$derived()` expects either a bare expression (`$derived(x + y)`) or uses `$derived.by()` for function bodies. Wrapping a bare expression in an arrow function creates a derived value that IS the function itself, not its return value, triggering unexpected reactivity behavior.

## The Correct Approach

```typescript
// CORRECT — bare expression
let count = $derived(items.length);

// CORRECT — function body with .by()
let filtered = $derived.by(() => items.filter(i => i.active));

// WRONG — causes infinite loop
let count = $derived(() => items.length);
```

## Where This Hit

- `ArtifactMasterDetail.svelte` — `$derived(() => ...)` on artifact list filtering
- `ArtifactNav.svelte` — `$derived(() => ...)` on navigation item computation
- `ArtifactLanding.svelte` — `$derived(() => ...)` on category selection

All three were fixed in the same session by converting to `$derived.by(() => ...)`.

## Prevention

The `svelte5-best-practices` skill documents this pattern. ESLint could potentially catch it with a custom rule.

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
