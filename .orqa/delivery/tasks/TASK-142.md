---
id: TASK-142
title: Replace React examples with Svelte 5 equivalents in tailwind-design-system skill
description: Replace all React code examples in the tailwind-design-system skill with Svelte 5 equivalents using shadcn-svelte patterns.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
acceptance:
  - All React code examples replaced with Svelte 5 equivalents
  - Examples use shadcn-svelte component patterns (not Radix UI)
  - Examples use Svelte 5 runes ($props, $derived) not Svelte 4 patterns
  - Design system principles and Tailwind guidance unchanged
  - Skill still works as general Tailwind knowledge (not overly OrqaStudio-specific)
---
## What

The `tailwind-design-system` skill (`layer: canon`) uses React code examples throughout — `React.ButtonHTMLAttributes`, `forwardRef`, Radix UI primitives. Replace all React examples with Svelte 5 equivalents using shadcn-svelte patterns.

Preserve the design system principles (tokens, variants, responsive patterns) — only change the code examples from React to Svelte 5.

## How

1. Open the `tailwind-design-system` skill file in `.orqa/team/skills/`
2. Read through every code block and identify which use React patterns
3. For each React example, write an equivalent Svelte 5 version using `$props()`, `$derived()`, and shadcn-svelte component patterns
4. Replace Radix UI primitive references with bits-ui equivalents where applicable
5. Verify no Svelte 4 patterns (`export let`, `$:`) appear in the updated examples

## Verification

- [ ] All React code examples replaced with Svelte 5 equivalents
- [ ] Examples use shadcn-svelte component patterns (not Radix UI)
- [ ] Examples use Svelte 5 runes ($props, $derived) not Svelte 4 patterns
- [ ] Design system principles and Tailwind guidance unchanged
- [ ] Skill still works as general Tailwind knowledge (not overly OrqaStudio-specific)
