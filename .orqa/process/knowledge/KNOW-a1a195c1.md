---
id: KNOW-a1a195c1
type: knowledge
title: Tailwind Design System
description: "Build scalable design systems with Tailwind CSS v4, design tokens, component libraries, and responsive patterns. Use when creating component libraries, implementing design systems, or standardizing UI patterns."
status: active
created: 2026-03-01
updated: 2026-03-10
category: domain
user-invocable: false
relationships:
  - target: DOC-fd1d12bb
    type: synchronised-with
---


Build production-ready design systems with Tailwind CSS v4, including CSS-first configuration, design tokens, component variants, responsive patterns, and accessibility.

> **Note**: This skill targets Tailwind CSS v4 (2024+). For v3 projects, refer to the [upgrade guide](https://tailwindcss.com/docs/upgrade-guide).

## When to Use This Skill

- Creating a component library with Tailwind v4
- Implementing design tokens and theming with CSS-first configuration
- Building responsive and accessible components
- Standardizing UI patterns across a codebase
- Migrating from Tailwind v3 to v4
- Setting up dark mode with native CSS features

## Key v4 Changes

| v3 Pattern                            | v4 Pattern                                                            |
| ------------------------------------- | --------------------------------------------------------------------- |
| `tailwind.config.ts`                  | `@theme` in CSS                                                       |
| `@tailwind base/components/utilities` | `@import "tailwindcss"`                                               |
| `darkMode: "class"`                   | `@custom-variant dark (&:where(.dark, .dark *))`                      |
| `theme.extend.colors`                 | `@theme { --color-*: value }`                                         |
| `require("tailwindcss-animate")`      | CSS `@keyframes` in `@theme` + `@starting-style` for entry animations |

## Quick Start

```css
/* app.css - Tailwind v4 CSS-first configuration */
@import "tailwindcss";

/* Define your theme with @theme */
@theme {
  /* Semantic color tokens using OKLCH for better color perception */
  --color-background: oklch(100% 0 0);
  --color-foreground: oklch(14.5% 0.025 264);

  --color-primary: oklch(14.5% 0.025 264);
  --color-primary-foreground: oklch(98% 0.01 264);

  --color-secondary: oklch(96% 0.01 264);
  --color-secondary-foreground: oklch(14.5% 0.025 264);

  --color-muted: oklch(96% 0.01 264);
  --color-muted-foreground: oklch(46% 0.02 264);

  --color-accent: oklch(96% 0.01 264);
  --color-accent-foreground: oklch(14.5% 0.025 264);

  --color-destructive: oklch(53% 0.22 27);
  --color-destructive-foreground: oklch(98% 0.01 264);

  --color-border: oklch(91% 0.01 264);
  --color-ring: oklch(14.5% 0.025 264);

  --color-card: oklch(100% 0 0);
  --color-card-foreground: oklch(14.5% 0.025 264);

  /* Ring offset for focus states */
  --color-ring-offset: oklch(100% 0 0);

  /* Radius tokens */
  --radius-sm: 0.25rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
  --radius-xl: 0.75rem;

  /* Animation tokens - keyframes inside @theme are output when referenced by --animate-* variables */
  --animate-fade-in: fade-in 0.2s ease-out;
  --animate-fade-out: fade-out 0.2s ease-in;
  --animate-slide-in: slide-in 0.3s ease-out;
  --animate-slide-out: slide-out 0.3s ease-in;

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes fade-out {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }

  @keyframes slide-in {
    from {
      transform: translateY(-0.5rem);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  @keyframes slide-out {
    from {
      transform: translateY(0);
      opacity: 1;
    }
    to {
      transform: translateY(-0.5rem);
      opacity: 0;
    }
  }
}

/* Dark mode variant - use @custom-variant for class-based dark mode */
@custom-variant dark (&:where(.dark, .dark *));

/* Dark mode theme overrides */
.dark {
  --color-background: oklch(14.5% 0.025 264);
  --color-foreground: oklch(98% 0.01 264);

  --color-primary: oklch(98% 0.01 264);
  --color-primary-foreground: oklch(14.5% 0.025 264);

  --color-secondary: oklch(22% 0.02 264);
  --color-secondary-foreground: oklch(98% 0.01 264);

  --color-muted: oklch(22% 0.02 264);
  --color-muted-foreground: oklch(65% 0.02 264);

  --color-accent: oklch(22% 0.02 264);
  --color-accent-foreground: oklch(98% 0.01 264);

  --color-destructive: oklch(42% 0.15 27);
  --color-destructive-foreground: oklch(98% 0.01 264);

  --color-border: oklch(22% 0.02 264);
  --color-ring: oklch(83% 0.02 264);

  --color-card: oklch(14.5% 0.025 264);
  --color-card-foreground: oklch(98% 0.01 264);

  --color-ring-offset: oklch(14.5% 0.025 264);
}

/* Base styles */
@layer base {
  * {
    @apply border-border;
  }

  body {
    @apply bg-background text-foreground antialiased;
  }
}
```

## Core Concepts

### 1. Design Token Hierarchy

```
Brand Tokens (abstract)
    └── Semantic Tokens (purpose)
        └── Component Tokens (specific)

Example:
    oklch(45% 0.2 260) → --color-primary → bg-primary
```

### 2. Component Architecture

```
Base styles → Variants → Sizes → States → Overrides
```

## Patterns

### Pattern 1: CVA (Class Variance Authority) Components

```svelte
<!-- components/ui/Button.svelte -->
<script lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  const buttonVariants = cva(
    // Base styles - v4 uses native CSS variables
    'inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
    {
      variants: {
        variant: {
          default: 'bg-primary text-primary-foreground hover:bg-primary/90',
          destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
          outline: 'border border-border bg-background hover:bg-accent hover:text-accent-foreground',
          secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
          ghost: 'hover:bg-accent hover:text-accent-foreground',
          link: 'text-primary underline-offset-4 hover:underline',
        },
        size: {
          default: 'h-10 px-4 py-2',
          sm: 'h-9 rounded-md px-3',
          lg: 'h-11 rounded-md px-8',
          icon: 'size-10',
        },
      },
      defaultVariants: {
        variant: 'default',
        size: 'default',
      },
    }
  );

  type ButtonVariants = VariantProps<typeof buttonVariants>;

  let {
    variant = 'default',
    size = 'default',
    class: className = '',
    children,
    ...restProps
  }: {
    variant?: ButtonVariants['variant'];
    size?: ButtonVariants['size'];
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
  } = $props();
</script>

<button
  class={cn(buttonVariants({ variant, size }), className)}
  {...restProps}
>
  {@render children?.()}
</button>
```

```svelte
<!-- Usage -->
<Button variant="destructive" size="lg">Delete</Button>
<Button variant="outline">Cancel</Button>
```

### Pattern 2: Compound Components (bits-ui / shadcn-svelte)

In shadcn-svelte, compound components are structured as named exports from a module, each as a
separate `.svelte` file re-exported via an `index.ts` barrel. Use bits-ui primitives (e.g.,
`bits-ui`) as the accessible foundation rather than Radix UI.

```svelte
<!-- components/ui/card/Card.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let {
    class: className = '',
    children,
  }: { class?: string; children?: Snippet } = $props();
</script>

<div class={cn('rounded-lg border border-border bg-card text-card-foreground shadow-sm', className)}>
  {@render children?.()}
</div>
```

```svelte
<!-- components/ui/card/CardHeader.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let { class: className = '', children }: { class?: string; children?: Snippet } = $props();
</script>

<div class={cn('flex flex-col space-y-1.5 p-6', className)}>
  {@render children?.()}
</div>
```

```svelte
<!-- components/ui/card/CardTitle.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let { class: className = '', children }: { class?: string; children?: Snippet } = $props();
</script>

<h3 class={cn('text-2xl font-semibold leading-none tracking-tight', className)}>
  {@render children?.()}
</h3>
```

```svelte
<!-- components/ui/card/CardContent.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let { class: className = '', children }: { class?: string; children?: Snippet } = $props();
</script>

<div class={cn('p-6 pt-0', className)}>
  {@render children?.()}
</div>
```

```svelte
<!-- Usage -->
<script lang="ts">
  import { Card, CardHeader, CardTitle, CardContent, CardFooter } from '$lib/components/ui/card';
  import Button from '$lib/components/ui/Button.svelte';
</script>

<Card>
  <CardHeader>
    <CardTitle>Account</CardTitle>
    <p class="text-sm text-muted-foreground">Manage your account settings</p>
  </CardHeader>
  <CardContent>
    <form>...</form>
  </CardContent>
  <div class="flex items-center p-6 pt-0">
    <Button>Save</Button>
  </div>
</Card>
```

### Pattern 3: Form Components

```svelte
<!-- components/ui/Input.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';

  let {
    class: className = '',
    type = 'text',
    error,
    id,
    ...restProps
  }: {
    class?: string;
    type?: string;
    error?: string;
    id?: string;
    [key: string]: unknown;
  } = $props();
</script>

<div class="relative">
  <input
    {type}
    {id}
    class={cn(
      'flex h-10 w-full rounded-md border border-border bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
      error && 'border-destructive focus-visible:ring-destructive',
      className
    )}
    aria-invalid={!!error}
    aria-describedby={error && id ? `${id}-error` : undefined}
    {...restProps}
  />
  {#if error}
    <p id={id ? `${id}-error` : undefined} class="mt-1 text-sm text-destructive" role="alert">
      {error}
    </p>
  {/if}
</div>
```

```svelte
<!-- components/ui/Label.svelte -->
<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let {
    class: className = '',
    children,
    ...restProps
  }: { class?: string; children?: Snippet; [key: string]: unknown } = $props();
</script>

<label
  class={cn('text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70', className)}
  {...restProps}
>
  {@render children?.()}
</label>
```

```svelte
<!-- Usage: login form with Svelte 5 runes -->
<script lang="ts">
  import Input from '$lib/components/ui/Input.svelte';
  import Label from '$lib/components/ui/Label.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  let email = $state('');
  let password = $state('');
  let emailError = $state('');
  let passwordError = $state('');

  function validate(): boolean {
    emailError = '';
    passwordError = '';
    if (!email.includes('@')) emailError = 'Invalid email address';
    if (password.length < 8) passwordError = 'Password must be at least 8 characters';
    return !emailError && !passwordError;
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!validate()) return;
    // submit logic here
  }
</script>

<form onsubmit={handleSubmit} class="space-y-4">
  <div class="space-y-2">
    <Label for="email">Email</Label>
    <Input id="email" type="email" bind:value={email} error={emailError} />
  </div>
  <div class="space-y-2">
    <Label for="password">Password</Label>
    <Input id="password" type="password" bind:value={password} error={passwordError} />
  </div>
  <Button type="submit" class="w-full">Sign In</Button>
</form>
```

### Pattern 4: Responsive Grid System

```svelte
<!-- components/ui/Grid.svelte -->
<script lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  const gridVariants = cva('grid', {
    variants: {
      cols: {
        1: 'grid-cols-1',
        2: 'grid-cols-1 sm:grid-cols-2',
        3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
        4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4',
        5: 'grid-cols-2 sm:grid-cols-3 lg:grid-cols-5',
        6: 'grid-cols-2 sm:grid-cols-3 lg:grid-cols-6',
      },
      gap: {
        none: 'gap-0',
        sm: 'gap-2',
        md: 'gap-4',
        lg: 'gap-6',
        xl: 'gap-8',
      },
    },
    defaultVariants: {
      cols: 3,
      gap: 'md',
    },
  });

  type GridVariants = VariantProps<typeof gridVariants>;

  let {
    cols = 3,
    gap = 'md',
    class: className = '',
    children,
  }: {
    cols?: GridVariants['cols'];
    gap?: GridVariants['gap'];
    class?: string;
    children?: Snippet;
  } = $props();
</script>

<div class={cn(gridVariants({ cols, gap }), className)}>
  {@render children?.()}
</div>
```

```svelte
<!-- components/ui/Container.svelte -->
<script lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  const containerVariants = cva('mx-auto w-full px-4 sm:px-6 lg:px-8', {
    variants: {
      size: {
        sm: 'max-w-screen-sm',
        md: 'max-w-screen-md',
        lg: 'max-w-screen-lg',
        xl: 'max-w-screen-xl',
        '2xl': 'max-w-screen-2xl',
        full: 'max-w-full',
      },
    },
    defaultVariants: { size: 'xl' },
  });

  type ContainerVariants = VariantProps<typeof containerVariants>;

  let {
    size = 'xl',
    class: className = '',
    children,
  }: {
    size?: ContainerVariants['size'];
    class?: string;
    children?: Snippet;
  } = $props();
</script>

<div class={cn(containerVariants({ size }), className)}>
  {@render children?.()}
</div>
```

```svelte
<!-- Usage -->
<script lang="ts">
  import Container from '$lib/components/ui/Container.svelte';
  import Grid from '$lib/components/ui/Grid.svelte';
  import ProductCard from '$lib/components/ProductCard.svelte';

  let { products } = $props<{ products: Product[] }>();
</script>

<Container>
  <Grid cols={4} gap="lg">
    {#each products as product (product.id)}
      <ProductCard {product} />
    {/each}
  </Grid>
</Container>
```

### Pattern 5: Native CSS Animations (v4)

```css
/* In your CSS file - native @starting-style for entry animations */
@theme {
  --animate-dialog-in: dialog-fade-in 0.2s ease-out;
  --animate-dialog-out: dialog-fade-out 0.15s ease-in;
}

@keyframes dialog-fade-in {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-0.5rem);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

@keyframes dialog-fade-out {
  from {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
  to {
    opacity: 0;
    transform: scale(0.95) translateY(-0.5rem);
  }
}

/* Native popover animations using @starting-style */
[popover] {
  transition:
    opacity 0.2s,
    transform 0.2s,
    display 0.2s allow-discrete;
  opacity: 0;
  transform: scale(0.95);
}

[popover]:popover-open {
  opacity: 1;
  transform: scale(1);
}

@starting-style {
  [popover]:popover-open {
    opacity: 0;
    transform: scale(0.95);
  }
}
```

```svelte
<!-- components/ui/dialog/DialogContent.svelte - Using bits-ui Dialog primitive -->
<script lang="ts">
  import { Dialog } from 'bits-ui';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  let {
    class: className = '',
    children,
  }: { class?: string; children?: Snippet } = $props();
</script>

<Dialog.Portal>
  <Dialog.Overlay
    class={cn(
      'fixed inset-0 z-50 bg-black/80',
      'data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out'
    )}
  />
  <Dialog.Content
    class={cn(
      'fixed left-1/2 top-1/2 z-50 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 border border-border bg-background p-6 shadow-lg sm:rounded-lg',
      'data-[state=open]:animate-dialog-in data-[state=closed]:animate-dialog-out',
      className
    )}
  >
    {@render children?.()}
  </Dialog.Content>
</Dialog.Portal>
```

### Pattern 6: Dark Mode with CSS (v4)

```typescript
// stores/theme.svelte.ts - Runes-based theme store
type Theme = 'dark' | 'light' | 'system';

function createThemeStore(storageKey = 'theme') {
  let theme = $state<Theme>('system');
  let resolvedTheme = $state<'dark' | 'light'>('light');

  function resolve(t: Theme): 'dark' | 'light' {
    if (t !== 'system') return t;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }

  function apply(t: Theme) {
    const resolved = resolve(t);
    document.documentElement.classList.remove('light', 'dark');
    document.documentElement.classList.add(resolved);
    resolvedTheme = resolved;

    const metaThemeColor = document.querySelector('meta[name="theme-color"]');
    if (metaThemeColor) {
      metaThemeColor.setAttribute('content', resolved === 'dark' ? '#09090b' : '#ffffff');
    }
  }

  function setTheme(newTheme: Theme) {
    theme = newTheme;
    localStorage.setItem(storageKey, newTheme);
    apply(newTheme);
  }

  // Initialise from storage on first use
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem(storageKey) as Theme | null;
    if (stored) theme = stored;
    apply(theme);
  }

  return {
    get theme() { return theme; },
    get resolvedTheme() { return resolvedTheme; },
    setTheme,
  };
}

export const themeStore = createThemeStore();
```

```svelte
<!-- components/ThemeToggle.svelte -->
<script lang="ts">
  import { Moon, Sun } from 'lucide-svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { themeStore } from '$lib/stores/theme.svelte';
</script>

<Button
  variant="ghost"
  size="icon"
  onclick={() => themeStore.setTheme(themeStore.resolvedTheme === 'dark' ? 'light' : 'dark')}
>
  <Sun class="size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
  <Moon class="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
  <span class="sr-only">Toggle theme</span>
</Button>
```

## Utility Functions

```typescript
// lib/utils.ts
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// Focus ring utility
export const focusRing = cn(
  "focus-visible:outline-none focus-visible:ring-2",
  "focus-visible:ring-ring focus-visible:ring-offset-2",
);

// Disabled utility
export const disabled = "disabled:pointer-events-none disabled:opacity-50";
```

## Advanced v4 Patterns

### Custom Utilities with `@utility`

Define reusable custom utilities:

```css
/* Custom utility for decorative lines */
@utility line-t {
  @apply relative before:absolute before:top-0 before:-left-[100vw] before:h-px before:w-[200vw] before:bg-gray-950/5 dark:before:bg-white/10;
}

/* Custom utility for text gradients */
@utility text-gradient {
  @apply bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent;
}
```

### Theme Modifiers

```css
/* Use @theme inline when referencing other CSS variables */
@theme inline {
  --font-sans: var(--font-inter), system-ui;
}

/* Use @theme static to always generate CSS variables (even when unused) */
@theme static {
  --color-brand: oklch(65% 0.15 240);
}

/* Import with theme options */
@import "tailwindcss" theme(static);
```

### Namespace Overrides

```css
@theme {
  /* Clear all default colors and define your own */
  --color-*: initial;
  --color-white: #fff;
  --color-black: #000;
  --color-primary: oklch(45% 0.2 260);
  --color-secondary: oklch(65% 0.15 200);

  /* Clear ALL defaults for a minimal setup */
  /* --*: initial; */
}
```

### Semi-transparent Color Variants

```css
@theme {
  /* Use color-mix() for alpha variants */
  --color-primary-50: color-mix(in oklab, var(--color-primary) 5%, transparent);
  --color-primary-100: color-mix(
    in oklab,
    var(--color-primary) 10%,
    transparent
  );
  --color-primary-200: color-mix(
    in oklab,
    var(--color-primary) 20%,
    transparent
  );
}
```

### Container Queries

```css
@theme {
  --container-xs: 20rem;
  --container-sm: 24rem;
  --container-md: 28rem;
  --container-lg: 32rem;
}
```

## v3 to v4 Migration Checklist

- [ ] Replace `tailwind.config.ts` with CSS `@theme` block
- [ ] Change `@tailwind base/components/utilities` to `@import "tailwindcss"`
- [ ] Move color definitions to `@theme { --color-*: value }`
- [ ] Replace `darkMode: "class"` with `@custom-variant dark`
- [ ] Move `@keyframes` inside `@theme` blocks (ensures keyframes output with theme)
- [ ] Replace `require("tailwindcss-animate")` with native CSS animations
- [ ] Update `h-10 w-10` to `size-10` (new utility)
- [ ] Use Svelte 5 `$props()` for component props (no `export let`)
- [ ] Consider OKLCH colors for better color perception
- [ ] Replace custom plugins with `@utility` directives

## Best Practices

### Do's

- **Use `@theme` blocks** - CSS-first configuration is v4's core pattern
- **Use OKLCH colors** - Better perceptual uniformity than HSL
- **Compose with CVA** - Type-safe variants
- **Use semantic tokens** - `bg-primary` not `bg-blue-500`
- **Use `size-*`** - New shorthand for `w-* h-*`
- **Add accessibility** - ARIA attributes, focus states

### Don'ts

- **Don't use `tailwind.config.ts`** - Use CSS `@theme` instead
- **Don't use `@tailwind` directives** - Use `@import "tailwindcss"`
- **Don't use `export let` or `$:` reactive statements** - Use Svelte 5 runes (`$props()`, `$state()`, `$derived()`)
- **Don't use arbitrary values** - Extend `@theme` instead
- **Don't hardcode colors** - Use semantic tokens
- **Don't forget dark mode** - Test both themes

## Resources

- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs)
- [Tailwind v4 Beta Announcement](https://tailwindcss.com/blog/tailwindcss-v4-beta)
- [CVA Documentation](https://cva.style/docs)
- [shadcn/ui](https://ui.shadcn.com/)
- [bits-ui Primitives](https://www.bits-ui.com/docs/introduction)
- [shadcn-svelte](https://www.shadcn-svelte.com/)
