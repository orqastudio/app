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
    type: grounded
  - target: TASK-016
    type: grounded
  - target: TASK-017
    type: grounded
  - target: TASK-018
    type: grounded
  - target: TASK-023
    type: grounded
  - target: TASK-026
    type: grounded
  - target: TASK-069
    type: grounded
  - target: TASK-075
    type: grounded
  - target: TASK-076
    type: grounded
  - target: TASK-077
    type: grounded
  - target: TASK-078
    type: grounded
  - target: TASK-082
    type: grounded
  - target: TASK-083
    type: grounded
  - target: TASK-161
    type: grounded
  - target: TASK-190
    type: grounded
  - target: TASK-403
    type: grounded
  - target: TASK-404
    type: grounded
  - target: TASK-405
    type: grounded
  - target: TASK-406
    type: grounded
  - target: TASK-407
    type: grounded
  - target: TASK-408
    type: grounded
  - target: TASK-420
    type: grounded
  - target: TASK-421
    type: grounded
  - target: TASK-422
    type: grounded
  - target: TASK-423
    type: grounded
  - target: TASK-428
    type: grounded
  - target: TASK-469
    type: grounded
  - target: TASK-470
    type: grounded
  - target: TASK-471
    type: grounded
  - target: TASK-472
    type: grounded
  - target: TASK-473
    type: grounded
  - target: TASK-475
    type: grounded
  - target: TASK-476
    type: grounded
  - target: TASK-478
    type: grounded
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
