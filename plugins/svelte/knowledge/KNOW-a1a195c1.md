---
id: KNOW-a1a195c1
type: knowledge
title: Tailwind Design System
description: "Build scalable design systems with Tailwind CSS v4, design tokens, component libraries, and responsive patterns. Use when creating component libraries, implementing design systems, or standardizing UI patterns."
summary: "Build scalable design systems with Tailwind CSS v4, design tokens, component libraries, and responsive patterns. Use when creating component libraries, implementing design systems, or standardizing UI patterns."
status: active
created: 2026-03-01
updated: 2026-03-10
category: domain
user-invocable: false
relationships:
  - target: DOC-fd1d12bb
    type: synchronised-with
---

Tailwind CSS v4 design system patterns for Svelte 5 with CSS-first configuration, semantic tokens, and CVA component variants.

## Key v4 Changes

| v3 Pattern | v4 Pattern |
|---|---|
| `tailwind.config.ts` | `@theme` in CSS |
| `@tailwind base/components/utilities` | `@import "tailwindcss"` |
| `darkMode: "class"` | `@custom-variant dark (&:where(.dark, .dark *))` |
| `theme.extend.colors` | `@theme { --color-*: value }` |
| `require("tailwindcss-animate")` | CSS `@keyframes` in `@theme` |

## Theme Configuration

```css
@import "tailwindcss";

@theme {
  --color-background: oklch(100% 0 0);
  --color-foreground: oklch(14.5% 0.025 264);
  --color-primary: oklch(14.5% 0.025 264);
  --color-primary-foreground: oklch(98% 0.01 264);
  --color-destructive: oklch(53% 0.22 27);
  --color-border: oklch(91% 0.01 264);
  --color-ring: oklch(14.5% 0.025 264);
  --radius-sm: 0.25rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
  --animate-fade-in: fade-in 0.2s ease-out;
  @keyframes fade-in { from { opacity: 0; } to { opacity: 1; } }
}

@custom-variant dark (&:where(.dark, .dark *));
```

## Design Token Hierarchy

```
Brand Tokens (abstract) → Semantic Tokens (purpose) → Component Tokens (specific)
Example: oklch(45% 0.2 260) → --color-primary → bg-primary
```

## CVA Component Pattern

```svelte
<script lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  const buttonVariants = cva(
    'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:ring-2 disabled:opacity-50',
    {
      variants: {
        variant: {
          default: 'bg-primary text-primary-foreground hover:bg-primary/90',
          destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
          outline: 'border border-border bg-background hover:bg-accent',
          ghost: 'hover:bg-accent hover:text-accent-foreground',
        },
        size: { default: 'h-10 px-4 py-2', sm: 'h-9 px-3', lg: 'h-11 px-8', icon: 'size-10' },
      },
      defaultVariants: { variant: 'default', size: 'default' },
    }
  );

  type ButtonVariants = VariantProps<typeof buttonVariants>;
  let { variant = 'default', size = 'default', class: className = '', children, ...restProps }:
    { variant?: ButtonVariants['variant']; size?: ButtonVariants['size']; class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<button class={cn(buttonVariants({ variant, size }), className)} {...restProps}>
  {@render children?.()}
</button>
```

## Compound Components (shadcn-svelte)

Structure as named exports from a module, each a separate `.svelte` file re-exported via `index.ts`. Use bits-ui primitives as accessible foundation.

## Utility Function

```typescript
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
export function cn(...inputs: ClassValue[]) { return twMerge(clsx(inputs)); }
```

## Advanced v4 Patterns

- **`@utility`**: Define reusable custom utilities (`@utility text-gradient { ... }`)
- **`@theme inline`**: Reference other CSS variables (`--font-sans: var(--font-inter)`)
- **`@theme static`**: Always generate CSS variables even when unused
- **Namespace overrides**: `--color-*: initial;` to clear defaults
- **Alpha variants**: `color-mix(in oklab, var(--color-primary) 50%, transparent)`

## Rules

- Use `@theme` blocks, not `tailwind.config.ts`
- Use OKLCH colors for perceptual uniformity
- Use semantic tokens (`bg-primary` not `bg-blue-500`)
- Use CVA for type-safe component variants
- Use `$props()` not `export let`
- Always test dark mode
