---
id: PD-ecc96aef
type: principle-decision
title: Svelte 5 Runes Only
description: "Svelte 5 runes only ($state, $derived, $effect, $props). No Svelte 4 patterns."
status: completed
created: 2026-03-02
updated: 2026-03-13
relationships: []
---

## Decision

Use Svelte 5 runes exclusively (`$state`, `$derived`, `$effect`, `$props`). No Svelte 4 patterns.

## Rationale

Runes provide a cleaner, more predictable reactivity model. Mixing old and new patterns creates confusion and inconsistency.

## Consequences

All components use `$props()` instead of `export let`, `$derived()` instead of `$:`, and `{#snippet}` instead of `<slot>`.
