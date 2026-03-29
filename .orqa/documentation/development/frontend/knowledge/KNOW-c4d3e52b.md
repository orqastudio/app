---
id: KNOW-c4d3e52b
type: knowledge
title: "SvelteKit Patterns Reference"
domain: platform/svelte
description: "Patterns for SvelteKit load functions, page props typing, form actions, and server vs universal load function selection."
tier: on-demand
summary: "SvelteKit Patterns Reference"
status: active
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: DOC-fd1d12bb
    type: synchronised-with
---

# SvelteKit Patterns Reference

## Load Function Types

Two types: universal (`+page.js`) and server-only (`+page.server.ts`).

| Use +page.server.ts | Use +page.js |
| --------------------- | -------------- |
| Secrets/credentials, DB, server APIs | Public APIs, non-serializable data (functions, classes) |

Server load: use `$env/static/private` for secrets. Universal load: only `$env/static/public`. Server load cannot return non-serializable values (functions, class instances).

```ts
// +page.server.ts — secrets safe, only runs on server
import { STRIPE_SECRET_KEY } from '$env/static/private';
export const load = async ({ fetch }) => {
  const res = await fetch('/api/charges', { headers: { Authorization: `Bearer ${STRIPE_SECRET_KEY}` } });
  return { charges: await res.json() };
};
```

## Page Props Typing

SvelteKit generates types in `./$types`. Use `$props()` for type-safe data access.

```svelte
<script lang="ts">
  import type { PageProps } from './$types';
  let { data, form }: PageProps = $props();
</script>
```

Layout uses `LayoutProps` with `children`. Error page uses `page` from `$app/state`.

## Form Actions Error Handling

Use `fail()` for validation (preserves form state), `throw error()` for unexpected errors (shows error page), `throw redirect()` for navigation.

```ts
import { fail, error } from '@sveltejs/kit';
export const actions = {
  default: async ({ request }) => {
    const data = await request.formData();
    if (!data.get('email')?.toString().includes('@'))
      return fail(400, { error: 'Invalid email', email: data.get('email') });
    try { await db.save(data); return { success: true }; }
    catch { throw error(500, 'Unable to save'); }
  }
};
```

| Situation | Function | Result |
| ----------- | ---------- | -------- |
| Validation error | `fail(400, {...})` | Form state preserved |
| Not found / server crash | `throw error(status, ...)` | Error page |
| Auth required | `throw redirect(303, ...)` | Redirect |

## SSR State Isolation

Module-level state on the server persists across requests, leaking data between users.

**Dangerous:** module-level `let` variables, global `$state` — shared across all requests on the server.

**Safe patterns:**

- `event.locals` — per-request, set in `hooks.server.ts`, accessed in load functions
- Return all user data from load functions, never from module scope
- `setContext` in layouts for component-tree-scoped state
- `browser` guard from `$app/environment` for client-only stores

### SSR Safety Checklist

- No module-level `let` storing user data
- No global `$state` set during SSR
- All user data flows through `locals` and load returns
