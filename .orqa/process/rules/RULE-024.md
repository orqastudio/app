---
id: RULE-024
title: Reusable Components
description: "Check shared component library before creating new UI elements. Use EmptyState, LoadingSpinner, ErrorDisplay, etc."
status: active
created: 2026-03-07
updated: 2026-03-11
layer: project
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Reusable components enforce structural consistency in the UI layer
  - target: RULE-006
    type: informs
    rationale: Component size limits and variant patterns are part of the shared library standards
  - target: RULE-005
    type: informs
    rationale: Use semantic search to find existing implementations before creating new components
  - target: RULE-005
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RULE-005
  - target: DOC-016
    type: informed-by
    rationale: Referenced in documentation page Svelte Component Tree
  - target: RULE-006
    type: informed-by
  - target: RULE-033
    type: informed-by
  - target: PILLAR-001
    type: informs
---
## Shared Component Library

Before creating ANY new UI element, check the reusable component locations below.

### `$lib/components/shared/` — General-Purpose Components

| Component | Purpose | Use When |
|-----------|---------|----------|
| `EmptyState` | Empty list/grid placeholder with optional icon, title, description, action | ANY page with no data to show |
| `LoadingSpinner` | Branded (logo-pulse) or minimal (CSS) loading indicator, 3 sizes | ANY async data fetch |
| `ErrorDisplay` | Error message with destructive styling and optional retry | ANY error state |
| `ConfirmDeleteDialog` | AlertDialog wrapper for destructive action confirmation | ANY delete/remove action |
| `StatusIndicator` | Multi-mode status display (badge/dot/inline) with group-based color mapping | ANY artifact status display |
| `SmallBadge` | Compact badge wrapper (11px text) | Metadata tags, skill badges |
| `SearchInput` | Search input with icon prefix, sm/xs variants | ANY filterable list search |
| `MetadataRow` | Icon + label + badge array row | Displaying tagged metadata |
| `SelectMenu` | Dropdown select with check marks | ANY single-value selection |
| `ThinkingBlock` | Collapsible AI thinking content with auto-collapse | AI reasoning display |
| `ArtifactListItem` | Clickable list item with status dot, label, description | Artifact navigation lists |

### `$lib/components/content/` — Content Rendering

| Component | Purpose | Use When |
|-----------|---------|----------|
| `CodeBlock` | Syntax-highlighted code display with copy button | ANY code rendering |
| `MarkdownRenderer` | Markdown content display with artifact links | ANY markdown rendering |

### `$lib/components/tool/` — Tool Integration

| Component | Purpose | Use When |
|-----------|---------|----------|
| `ToolCallCard` | Tool call display with approval UI | ALL tool call rendering |

### `$lib/components/conversation/` — Conversation Components

| Component | Purpose | Use When |
|-----------|---------|----------|
| `MessageBubble` | Base chat message bubble | Custom message rendering |
| `UserMessage` / `AssistantMessage` / `SystemMessage` | Role-specific message wrappers | Conversation display |

## Rules

1. **Search before creating** — Before writing a new component, search `$lib/components/` for existing ones
2. **No inline empty states** — NEVER write `<div class="py-12 text-center"><p>No items</p></div>`. Use `<EmptyState>`
3. **No inline loading states** — NEVER write custom spinners. Use `<LoadingSpinner>`
4. **No inline error states** — NEVER write custom error cards. Use `<ErrorDisplay>`
5. **No inline status displays** — NEVER write custom status badges. Use `<StatusIndicator>`
6. **Consistent page patterns** — All list pages follow: loading -> empty -> content
7. **shadcn-svelte first** — Use shadcn-svelte primitives (Button, Card, Dialog, etc.) before building custom components

## Panel Layout

OrqaStudio uses a multi-panel layout (conversation + artifact panels) built on shadcn `Resizable` components (`PaneGroup`, `Pane`, `Handle`). Panel layouts MUST:

- Be resizable via drag handles
- Support collapsed/expanded states
- Maintain state across navigation

## ChunkHound Integration

Use `search_semantic` to find similar components across the *entire* frontend, not just `$lib/components/shared/`. Searching "empty state component" or "progress indicator" may find page-specific implementations that should be extracted to shared.

## Related Rules

- [RULE-006](RULE-006) (coding-standards) — component size limits and variant patterns
- [RULE-005](RULE-005) (code-search-usage) — semantic search for finding existing implementations
