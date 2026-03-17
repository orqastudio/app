---
id: SKILL-030
title: Svelte 5 Best Practices
description: "Svelte 5 runes, snippets, SvelteKit patterns, and modern best practices for TypeScript and component development. Use when writing, reviewing, or refactoring Svelte 5 components and SvelteKit applications. Triggers on: Svelte components, runes ($state, $derived, $effect, $props, $bindable, $inspect), snippets ({#snippet}, {@render}), event handling, SvelteKit data loading, form actions, Svelte 4 to Svelte 5 migration, store to rune migration, slots to snippets migration, TypeScript props typing, generic components, SSR state isolation, performance optimization, or component testing."
status: active
created: 2026-03-01
updated: 2026-03-10
layer: core
category: domain
file-patterns:
  - "ui/src/lib/components/**"
  - "ui/src/lib/stores/**"
user-invocable: false
license: MIT
metadata: null
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Rune patterns, snippet APIs, and TypeScript prop typing enforce explicit component contracts that are visible in code and reviewable
  - target: TASK-009
    type: informs
  - target: TASK-016
    type: informs
  - target: TASK-017
    type: informs
  - target: TASK-018
    type: informs
  - target: TASK-023
    type: informs
  - target: TASK-026
    type: informs
  - target: TASK-069
    type: informs
  - target: TASK-075
    type: informs
  - target: TASK-076
    type: informs
  - target: TASK-077
    type: informs
  - target: TASK-078
    type: informs
  - target: TASK-082
    type: informs
  - target: TASK-083
    type: informs
  - target: TASK-161
    type: informs
  - target: TASK-190
    type: informs
  - target: TASK-403
    type: informs
  - target: TASK-404
    type: informs
  - target: TASK-405
    type: informs
  - target: TASK-406
    type: informs
  - target: TASK-407
    type: informs
  - target: TASK-408
    type: informs
  - target: TASK-420
    type: informs
  - target: TASK-421
    type: informs
  - target: TASK-422
    type: informs
  - target: TASK-423
    type: informs
  - target: TASK-428
    type: informs
  - target: TASK-469
    type: informs
  - target: TASK-470
    type: informs
  - target: TASK-471
    type: informs
  - target: TASK-472
    type: informs
  - target: TASK-473
    type: informs
  - target: TASK-475
    type: informs
  - target: TASK-476
    type: informs
  - target: TASK-478
    type: informs
  - target: PILLAR-001
    type: informs
---


## Quick Reference

| Topic | When to Use | Reference |
|-------|-------------|-----------|
| **Runes** | $state, $derived, $effect, $props, $bindable, $inspect | [runes.md](references/runes.md) |
| **Snippets** | Replacing slots, {#snippet}, {@render} | [snippets.md](references/snippets.md) |
| **Events** | onclick handlers, callback props, context API | [events.md](references/events.md) |
| **TypeScript** | Props typing, generic components | [typescript.md](references/typescript.md) |
| **Migration** | Svelte 4 to 5, stores to runes | [migration.md](references/migration.md) |
| **SvelteKit** | Load functions, form actions, SSR, page typing | [sveltekit.md](references/sveltekit.md) |
| **Performance** | Universal reactivity, avoiding over-reactivity, streaming | [performance.md](references/performance.md) |

## Essential Patterns

### Reactive State

```svelte
<script>
  let count = $state(0);           // Reactive state
  let doubled = $derived(count * 2); // Computed value
</script>
```

### Component Props

```svelte
<script>
  let { name, count = 0 } = $props();
  let { value = $bindable() } = $props(); // Two-way binding
</script>
```

### Snippets (replacing slots)

```svelte
<script>
  let { children, header } = $props();
</script>

{@render header?.()}
{@render children()}
```

### Event Handlers

```svelte
<!-- Svelte 5: use onclick, not on:click -->
<button onclick={() => count++}>Click</button>
```

### Callback Props (replacing createEventDispatcher)

```svelte
<script>
  let { onclick } = $props();
</script>

<button onclick={() => onclick?.({ data })}>Click</button>
```

## Common Mistakes

1. **Using `let` without `$state`** - Variables are not reactive without `$state()`
2. **Using `$effect` for derived values** - Use `$derived` instead
3. **Using `on:click` syntax** - Use `onclick` in Svelte 5
4. **Using `createEventDispatcher`** - Use callback props instead
5. **Using `<slot>`** - Use snippets with `{@render}`
6. **Forgetting `$bindable()`** - Required for `bind:` to work
7. **Setting module-level state in SSR** - Causes cross-request leaks
8. **Sequential awaits in load functions** - Use `Promise.all` for parallel requests
9. **Duplicate keys in keyed `{#each}` blocks** - Concatenating data fields as keys (e.g. `item.id + item.name`) crashes when two items produce the same string. Always include the loop index as a suffix: `{#each items as item, i (item.id + item.name + i)}`, or use a guaranteed-unique ID field
