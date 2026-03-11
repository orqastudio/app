---
id: SKILL-041
title: Component Extraction
description: Methodology for identifying repeated UI patterns and extracting them into shared components. Covers detection heuristics, extraction criteria, prop design, and placement conventions.
status: active
created: "2026-03-11"
updated: "2026-03-11"
layer: project
scope: software-engineering
user-invocable: false
---

# Component Extraction

When building or modifying UI, agents must identify repeated patterns and extract them into shared components rather than duplicating markup. This skill teaches the detection heuristics, extraction criteria, and implementation patterns.

## Detection Heuristics

Before creating any new UI element, search for existing patterns:

1. **`search_semantic`** — "loading state component", "error display", "empty state", "status indicator"
2. **`search_regex`** — known class patterns like `animate-spin`, `text-muted-foreground`, `rounded-full`
3. **Grep for repeated markup** — `<Loader2`, `class="h-4 w-4 animate-spin"`, `{#if loading}`

### Known Repeated Patterns (from TASK-147 audit)

| Pattern | Occurrences | Files | Extraction Priority |
|---------|-------------|-------|-------------------|
| Setup step (icon + title + description + action) | 4 | `SetupPanel.svelte`, `ProviderSetup.svelte`, `SidecarSetup.svelte`, `ProjectSelector.svelte` | High |
| Inline status icons (colored dots/icons for status) | 19+ | Across conversation, artifact, settings components | High |
| Inline spinner (`<Loader2 class="h-4 w-4 animate-spin" />`) | 10+ | Scattered across components | Medium |
| Key-value metadata rows | 8+ | Detail panels, settings, artifact views | Medium |
| Centered placeholder (icon + heading + subtext) | 9+ | Various empty/placeholder states | Medium (overlaps EmptyState) |
| Button with loading spinner | 5+ | Forms, action panels | Low (simple pattern) |

## Extraction Criteria

Extract a pattern into a shared component when:

1. **3+ occurrences** — The pattern appears in 3 or more files
2. **Consistent structure** — The markup structure is substantially the same across occurrences
3. **Meaningful abstraction** — The component represents a recognizable UI concept (not just "div with padding")
4. **Prop-driven variants** — Differences between occurrences can be expressed as props

Do NOT extract when:

- The pattern appears only 1-2 times (premature abstraction)
- Occurrences differ significantly in structure (forced abstraction)
- The "component" would just be a styled div with no semantic meaning

## Component Design Patterns

### Props Over Slots (for simple content)

```svelte
<!-- GOOD: Props for simple text content -->
<SetupStep
  icon={Cpu}
  title="Configure Provider"
  description="Select your AI provider and enter credentials"
  action={{ label: "Configure", onclick: handleConfigure }}
/>

<!-- BAD: Slots for simple text -->
<SetupStep>
  <span slot="title">Configure Provider</span>
  <span slot="description">Select your AI provider...</span>
</SetupStep>
```

### Snippets for Complex Content

```svelte
<!-- GOOD: Snippets when content is rich -->
<MetadataRow label="Status">
  {#snippet value()}
    <StatusIndicator status={item.status} />
  {/snippet}
</MetadataRow>
```

### Variant Props Over Class Overrides

```svelte
<!-- GOOD: Size variant -->
<StatusIndicator status="active" size="sm" />
<StatusIndicator status="active" size="lg" />

<!-- BAD: Class override -->
<StatusIndicator status="active" class="h-2 w-2" />
```

## Placement Rules

| Component Type | Location | Example |
|---------------|----------|---------|
| Used across 3+ feature areas | `$lib/components/shared/` | `StatusIndicator`, `EmptyState` |
| Used within one feature area | Feature directory | `conversation/StreamingIndicator` |
| Layout primitives | `$lib/components/ui/` | shadcn components |

## Pre-Creation Checklist

Before creating ANY new component:

- [ ] Searched `$lib/components/shared/` for existing equivalent
- [ ] Searched with `search_semantic` for similar patterns across the codebase
- [ ] Checked RULE-024 component inventory for matches
- [ ] If the pattern exists inline in 3+ places, extract to shared instead of creating new
- [ ] If creating a new shared component, add it to RULE-024's inventory

## Integration with RULE-024

RULE-024 maintains the canonical component inventory. When extracting a new shared component:

1. Create the component in the correct location
2. Update RULE-024's inventory table
3. Replace all inline occurrences with the new component
4. Verify no inline implementations remain via `search_regex`

## Related Skills

- `orqa-composability` — the philosophy this skill implements at the component level
- `svelte5-best-practices` — Svelte 5 patterns for component props, snippets, and reactivity
- `ux-compliance-review` — reviews check for shared component usage
