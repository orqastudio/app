---
id: RULE-033
title: Tooltips over title attributes
description: "Use shadcn Tooltip components for hover hints instead of native HTML title attributes. Tooltips provide consistent styling, positioning, and accessibility."
status: active
created: 2026-03-10
updated: 2026-03-10
layer: project
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Tooltip consistency enforces structural UI patterns
  - target: RULE-006
    type: informs
    rationale: Tooltip usage is a specific component standard within the broader coding standards
  - target: RULE-024
    type: informs
    rationale: shadcn Tooltip is a shared component — use it consistently rather than reinventing hover hints
  - target: PILLAR-001
    type: informs
---

All interactive elements that need hover hints MUST use the shadcn `Tooltip` component (`$lib/components/ui/tooltip`) instead of the native HTML `title` attribute.

## Why

- Native `title` attributes render as plain browser tooltips with inconsistent styling across platforms
- shadcn Tooltips match the design system (theme-aware, positioned, animated)
- Tooltips support richer content (multiple lines, icons) when needed
- Consistent UX across the entire application

## Pattern

```svelte
<Tooltip.Root>
  <Tooltip.Trigger>
    {#snippet child({ props })}
      <button {...props} onclick={handleClick}>Label</button>
    {/snippet}
  </Tooltip.Trigger>
  <Tooltip.Content side="top">
    <p>Hint text</p>
  </Tooltip.Content>
</Tooltip.Root>
```

## FORBIDDEN

- `title="..."` on interactive elements (buttons, links, badges) for hover hints
- Custom tooltip implementations that duplicate shadcn Tooltip functionality

## Exceptions

- `alt` attributes on images (accessibility, not hover hints)
- `title` on `<svg>` elements (accessibility requirement)

## Related Rules

- [RULE-006](RULE-006) (coding-standards) — component library usage
- [RULE-024](RULE-024) (reusable-components) — use shared components consistently
