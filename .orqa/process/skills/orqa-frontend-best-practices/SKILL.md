---
id: SKILL-042
title: OrqaStudio Frontend Best Practices
description: |
  Umbrella skill for all frontend implementation work. Establishes composability,
  coding standards, and component conventions as always-in-mind principles, then
  references deeper skills for Svelte 5, TypeScript, and Tailwind specifics.
  Use when: Any agent is about to write or modify frontend code (ui/, components, stores).
status: active
created: 2026-03-11
updated: 2026-03-11
layer: project
category: domain
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Umbrella conventions for component purity, store patterns, and Svelte 5 runes ensure UI code follows visible, enforceable standards
  - target: AD-013
    type: practices
    rationale: Auto-generated inverse of practices relationship from AD-013
  - target: AD-019
    type: practices
    rationale: Auto-generated inverse of practices relationship from AD-019
  - type: scoped-to
    target: AGENT-001
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
---
This skill ensures every frontend agent has the right mental model before writing code. It does not duplicate content from deeper skills — it establishes principles and points to the right references.

## Always In Mind

### Composability ([SKILL-008](SKILL-008))

Every component, store, and utility follows OrqaStudio's composability philosophy:

- **Small enough to understand in isolation** — components under 150 lines, stores with single responsibility
- **Pure enough to test without the world** — display components receive props only, no `invoke()` in `$lib/components/`
- **Typed enough to compose safely** — strict TypeScript, no `any`, all props typed via `$props()`
- **Swappable enough to replace without cascading changes** — stores call `invoke()`, components read stores

Load the full `composability` skill for the complete philosophy and anti-patterns.

### Coding Standards (DOC-021)

Read `.orqa/documentation/development/coding-standards.md` before writing any code. Key frontend standards:

- **Svelte 5 runes only** — `$state`, `$derived`, `$effect`, `$props`. No Svelte 4 patterns.
- **Strict TypeScript** — `strict: true`, no `any`, no `@ts-ignore`, no unsafe casts
- **shadcn-svelte first** — use the component library before building custom components
- **No emoji in UI** — use Lucide icons for all visual indicators
- **Function size** — components under 150 lines, utilities under 20 lines
- **No TODO comments** — use task artifacts instead

### Component Purity ([AD-004](AD-004))

The boundary between containers and display components is strict:

| Layer | Responsibility | May call `invoke()`? |
|-------|---------------|---------------------|
| Pages / containers | Data fetching, routing | Yes |
| Stores (`.svelte.ts`) | State management, IPC calls | Yes |
| Display components (`$lib/components/`) | Rendering props | **Never** |

### Shared Components ([RULE-024](RULE-024))

Before creating ANY new UI element, check `$lib/components/shared/` for existing components. Use `EmptyState`, `LoadingSpinner`, `ErrorDisplay`, `StatusBadge`, `CodeBlock`, `MarkdownRenderer`, and others before building custom alternatives.

### State Handling

Every component that displays data MUST handle all states:

| State | What to show |
|-------|-------------|
| Loading | `<LoadingSpinner />` |
| Error | `<ErrorDisplay />` |
| Empty | `<EmptyState />` |
| Loaded | Real content |

## Deeper Skills (Load When Needed)

| Skill | When to load |
|-------|-------------|
| `svelte5-best-practices` | Svelte component patterns, runes, snippets, event handling |
| `typescript-advanced-types` | Generic types, discriminated unions, utility types |
| `tailwind-design-system` | Design tokens, responsive patterns, component variants |
| `orqa-store-patterns` | Store anatomy, reactive data flow, store-to-component wiring |
| `orqa-store-orchestration` | Multi-store coordination, cross-store derived state |
| `component-extraction` | Identifying and extracting shared components |

## Verification

Before committing frontend code:

```bash
make typecheck       # svelte-check (type errors)
make lint-frontend   # ESLint (code quality)
make test-frontend   # Vitest (unit tests)
```

All three must pass. See [RULE-012](RULE-012) — all errors are your responsibility.
