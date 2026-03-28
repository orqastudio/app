---
id: KNOW-df3c489e
type: knowledge
title: "Svelte 5 Performance Reference"
domain: platform/svelte
description: "Patterns for optimizing Svelte 5 reactivity, preventing waterfalls in SvelteKit load functions, and avoiding common $effect anti-patterns."
summary: "Svelte 5 Performance Reference"
status: active
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: DOC-fd1d12bb
    type: synchronised-with
---

# Svelte 5 Performance Reference

## Universal Reactivity (.svelte.ts files)

```ts
// counter.svelte.ts — must use .svelte.ts extension
export const counter = $state({ count: 0 });
export function increment() { counter.count++; }
```

Use getters for derived values in module scope (no `$derived` at module level). Avoid initializing browser-only state at module level (SSR safety).

## Reactivity Anti-Patterns

| Anti-Pattern | Fix |
| --- | --- |
| `$effect` to set derived values | Use `$derived(count * 2)` |
| Circular `$effect` dependencies | Separate effects or event handlers |
| Reading reactive state inside `$effect` that also writes it | Use `untrack()` |
| Heavy computation in `$derived` | Debounce with `$effect` + `setTimeout` |
| `$effect` for DOM manipulation | Use `{#if}` or `class:` directives |

## Load Performance

**Prevent waterfalls** — use `Promise.all` for independent requests:

```ts
export const load = async ({ fetch }) => {
  const [user, posts] = await Promise.all([
    fetch('/api/user').then(r => r.json()),
    fetch('/api/posts').then(r => r.json()),
  ]);
  return { user, posts };
};
```

**Stream non-critical data** — return unawaited promises:

```ts
export const load = async ({ fetch }) => {
  const user = await fetch('/api/user').then(r => r.json());
  return {
    user,                                                    // immediate
    analytics: fetch('/api/analytics').then(r => r.json()),  // streamed
  };
};
```

Use `{#await data.analytics}` in components to handle streamed data.

| Data Type | Stream? | Reason |
| ----------- | --------- | -------- |
| User info / main content | No | Critical for layout |
| Analytics / recommendations | Yes | Supplementary |

## Component Testing (Vitest)

```ts
// vitest.config.ts — browser mode recommended for runes
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: { browser: { enabled: true, provider: 'playwright', name: 'chromium' } }
});
```

```ts
import { render, screen, fireEvent } from '@testing-library/svelte';
test('increments', async () => {
  render(Counter);
  await fireEvent.click(screen.getByRole('button'));
  expect(screen.getByRole('button')).toHaveTextContent('Count: 1');
});
```

## Rules

- Use `$derived` over `$effect` for computed values
- Use `untrack` to prevent unnecessary reactive dependencies
- Parallel-fetch independent data with `Promise.all`
- Stream non-critical data by returning unawaited promises
- Let Svelte handle DOM updates — no manual DOM manipulation in `$effect`
