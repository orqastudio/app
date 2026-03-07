---
scope: system
---

# Reusable Components (NON-NEGOTIABLE)

## Shared Component Library

Before creating ANY new UI element, check `$lib/components/shared/`:

| Component | Purpose | Use When |
|-----------|---------|----------|
| `EmptyState` | Empty list/grid placeholder | ANY page with no data to show |
| `LoadingSpinner` | Loading indicator | ANY async data fetch |
| `ErrorDisplay` | Error message with retry | ANY error state |
| `PageToolbar` | Filter/action toolbar | ALL list/library pages |
| `StatusBadge` | Status indicator badge | ANY status display |
| `ProgressBar` | Progress indication | ANY progress display |
| `Panel` | Resizable side panel | ALL multi-panel layouts |
| `CodeBlock` | Syntax-highlighted code display | ANY code rendering |
| `MarkdownRenderer` | Markdown content display | ANY markdown rendering |
| `ConversationMessage` | Chat message bubble | ALL conversation displays |
| `ToolCallCard` | Tool call display with approval | ALL tool call rendering |
| `ConfirmDeleteDialog` | Destructive action confirmation | ANY delete/remove action |

## Rules

1. **Search before creating** — Before writing a new component, search `$lib/components/shared/` for existing ones
2. **No inline empty states** — NEVER write `<div class="py-12 text-center"><p>No items</p></div>`. Use `<EmptyState>`
3. **No inline loading states** — NEVER write custom spinners. Use `<LoadingSpinner>`
4. **No inline error states** — NEVER write custom error cards. Use `<ErrorDisplay>`
5. **No custom toolbars on list pages** — ALWAYS use `<PageToolbar>`
6. **Consistent page patterns** — All list pages follow: toolbar (conditional on data) -> loading -> empty -> grid
7. **shadcn-svelte first** — Use shadcn-svelte primitives (Button, Card, Dialog, etc.) before building custom components

## Page Template (List Pages)

Every list page MUST follow this exact pattern:

```svelte
{#if items.length > 0}<PageToolbar>...</PageToolbar>{/if}
{#if loading}<LoadingSpinner />{:else if items.length === 0}<EmptyState ... />{:else}<grid>...</grid>{/if}
```

## Panel Layout Template

OrqaStudio uses a multi-panel layout (conversation + artifact panels). Panel components MUST:

- Be resizable via drag handles
- Support collapsed/expanded states
- Maintain state across navigation
- Use the shared `<Panel>` wrapper for consistent behavior

## ChunkHound Integration

Use `search_semantic` to find similar components across the *entire* frontend, not just `$lib/components/shared/`. Searching "empty state component" or "progress indicator" may find page-specific implementations that should be extracted to shared.

## Related Rules

- `coding-standards.md` — component size limits and variant patterns
- `chunkhound-usage.md` — semantic search for finding existing implementations
