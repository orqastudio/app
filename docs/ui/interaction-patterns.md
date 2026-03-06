---
title: "Interaction Patterns"
category: ui
tags: []
created: 2026-03-02
updated: 2026-03-04
---

# Interaction Patterns

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend), [Claude Integration Research](/research/claude-integration), [Design System](/ui/design-system)

How Orqa Studio's UI responds to user actions. Covers streaming display, tool call approval, inline editing, panel behavior, and keyboard shortcuts.

---

## 1. Streaming Token Display

### Behavior

When the AI responds, tokens appear incrementally in the conversation panel.

| Phase | What Happens | Timing |
|-------|-------------|--------|
| **Send** | User message appears immediately. Input clears. Send button disables. | Instant |
| **Waiting** | Streaming indicator (animated dots) appears in a new assistant message bubble. | 0-2s |
| **First token** | Indicator replaced with actual text. Tokens append as they arrive. | < 2s |
| **Streaming** | Text renders as raw characters, accumulating into markdown. No re-rendering per token. | Continuous |
| **Flush** | At natural pause points (~500ms gap or end of block), accumulated text is re-rendered as formatted markdown. | Periodic |
| **Complete** | Final markdown render of full message. Code blocks get syntax highlighting. Tool call cards finalize. | End |

### Auto-Scroll

- While streaming, the conversation auto-scrolls to keep the latest content visible
- If the user scrolls up (breaks from bottom), auto-scroll pauses
- A "scroll to bottom" floating button appears when not at bottom
- Clicking the button or pressing `End` re-engages auto-scroll

### Implementation Pipeline

```
Sidecar stdout (NDJSON) → Rust parser → Channel<T> → Svelte $state → DOM
```

Tokens update a `$state` rune. Svelte 5's fine-grained reactivity updates only the text node being appended, not the entire message tree. This avoids layout thrashing during streaming.

### Error During Streaming

If the sidecar emits an error event mid-stream:
- Partial response is preserved (not discarded)
- Error block appends below the partial content
- Error block shows: error type, message, retry button
- Retry re-sends the last user message

---

## 2. Tool Call Flow

### Phase 1: Read-Only Display

Tool calls are displayed but not interactive. Claude executes tools automatically via the sidecar.

| Event | UI Response |
|-------|------------|
| Tool call starts | Card appears inline: tool name + "Running..." badge (blue) |
| Tool call streams | Input parameters populate in the card (collapsed) |
| Tool call completes | Badge changes to "Completed" (green). Output stored. |
| Tool call errors | Badge changes to "Error" (red). Error message in output. |

### Phase 2: Approval Flow

User approval required before tool execution.

```
Tool call received → Card appears with "Pending" badge (amber)
                  → Approve / Deny / Modify buttons visible
                  → Timer optional (auto-approve after N seconds)
```

| Action | Result |
|--------|--------|
| **Approve** | Tool executes. Badge → "Running..." → "Completed"/"Error" |
| **Deny** | Tool skipped. Badge → "Denied" (gray). AI informed. |
| **Modify** | Input parameters become editable. User adjusts, then approves. |
| **Auto-approve** | If enabled in settings, non-destructive tools (Read, Glob, Grep) auto-approve after 2s |

### Tool Call Card Interaction

- **Collapsed** (default): Single line showing tool icon + name + file path/summary + status badge
- **Click to expand**: Full input parameters + full output + diff view (for edits)
- **Click tool name**: Opens tool inspector in Explorer Panel (Phase 2+)
- **Copy button**: Copies tool output to clipboard

### Visual Hierarchy for Tool Types

| Tool | Icon | Summary Format |
|------|------|---------------|
| Read | File icon | `Read src/auth/handler.rs (42 lines)` |
| Write | Pencil icon | `Write src/auth/handler.rs (new file)` |
| Edit | Diff icon | `Edit src/auth/handler.rs (+3/-1 lines)` |
| Bash | Terminal icon | `Bash: cargo test (exit 0)` |
| Glob | Search icon | `Glob: src/**/*.rs (12 matches)` |
| Grep | Search icon | `Grep: "TODO" in src/ (5 matches)` |

---

## 3. Inline Editing

### Artifact Editing (Explorer Panel)

| State | UI |
|-------|----|
| **View mode** (default) | Rendered markdown. Edit button in top-right corner. |
| **Edit mode** | CodeMirror 6 editor replaces rendered view. Save/Cancel buttons appear. |
| **Unsaved changes** | Dot indicator on the tab/title. "Unsaved changes" in status area. |
| **Save** | `Ctrl+S`. File written to disk. Edit mode stays active. Toast: "Saved". |
| **Cancel** | Discards changes. Returns to view mode. If unsaved, confirmation dialog. |
| **External change** | File watcher detects change. Banner: "This file was modified externally. Reload?" |

### Hookify Rule Editing

Hookify rules (`.claude/hookify.*.local.md`) use YAML frontmatter with structured fields: `event` (FileEdit or Bash), `action` (block or warn), and `conditions` (including `pattern` for regex matching). Editing hookify files follows the same view/edit/save flow as other artifacts, but the editor should validate YAML frontmatter structure on save — malformed `event`, `action`, or `conditions` fields produce a warning toast before writing.

### Session Title Editing

- Click session title in header → inline text input
- Enter to confirm, Escape to cancel
- Blur (click away) confirms the edit
- Empty title reverts to auto-generated title

---

## 4. Panel Resizing & Collapsing

### Resize Handles

- PaneForge drag handles between the Nav Sub-Panel, Explorer, and Chat panes: 1px visible border, 8px invisible drag target
- Cursor changes to `col-resize` on hover
- Drag smoothly resizes with min/max constraints
- Double-click handle: collapse the Nav Sub-Panel (only collapsible zone)

### Collapse Animation

- Collapse duration: 200ms ease-out
- Collapsed Nav Sub-Panel: 0px (completely hidden)
- Explorer and Chat panels fill freed space
- Activity Bar is always visible (fixed 48px)

### Collapse Triggers

| Trigger | Effect |
|---------|--------|
| `Ctrl+B` | Toggle Nav Sub-Panel |
| `Ctrl+1` through `Ctrl+5` | Switch Activity Bar to artifact category |
| `Ctrl+,` | Switch Activity Bar to settings |
| Double-click Nav Sub-Panel handle | Collapse/expand Nav Sub-Panel |
| `Escape` (overlay open) | Close overlay |

### State Persistence

Panel widths and collapse states are persisted via `tauri-plugin-window-state` and restored on app restart.

---

## 5. Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action | Context |
|----------|--------|---------|
| `Ctrl+K` / `Cmd+K` | Global search | Always |
| `Ctrl+N` / `Cmd+N` | New session | Always |
| `Ctrl+B` / `Cmd+B` | Toggle Nav Sub-Panel | Always |
| `Ctrl+0` / `Cmd+0` | Project Dashboard | Always |
| `Ctrl+1` through `Ctrl+5` | Switch artifact category | Always |
| `Ctrl+,` / `Cmd+,` | Open settings | Always |
| `Escape` | Close overlay / collapse detail / exit edit mode | Context-dependent |

### Conversation Shortcuts

| Shortcut | Action | Context |
|----------|--------|---------|
| `Enter` | Send message | Input focused |
| `Shift+Enter` | Insert newline | Input focused |
| `Ctrl+Up` / `Cmd+Up` | Navigate to previous message | Conversation |
| `Ctrl+Down` / `Cmd+Down` | Navigate to next message | Conversation |
| `End` | Scroll to bottom / re-engage auto-scroll | Conversation |

### Editor Shortcuts

| Shortcut | Action | Context |
|----------|--------|---------|
| `Ctrl+E` / `Cmd+E` | Toggle edit mode | Artifact viewer |
| `Ctrl+S` / `Cmd+S` | Save | Edit mode |
| `Ctrl+Z` / `Cmd+Z` | Undo | Edit mode |
| `Ctrl+Shift+Z` / `Cmd+Shift+Z` | Redo | Edit mode |

### Navigation Shortcuts

| Shortcut | Action | Context |
|----------|--------|---------|
| `Tab` | Move focus between zones | Always |
| `Shift+Tab` | Move focus backward between zones | Always |

---

## 6. Transitions & Animation

### Duration Scale

| Category | Duration | Easing |
|----------|----------|--------|
| Interactive states (hover, focus, press) | 150ms | ease-out |
| Panel transitions (collapse, expand) | 200ms | ease-out |
| Content transitions (view/edit toggle) | 150ms | ease-in-out |
| Overlay entrance (dialog, popover) | 200ms | ease-out |
| Overlay exit | 150ms | ease-in |
| Toast entrance | 300ms | spring |

### Reduced Motion

When `prefers-reduced-motion: reduce` is active:
- All transitions set to 0ms (instant)
- Streaming indicator: static dots instead of animated
- No spring animations
- Panel collapse/expand: instant

---

## 7. Focus Management

### Focus Order

Logical tab order follows the visual layout: Toolbar → Activity Bar → Nav Sub-Panel → Explorer Panel → Chat Panel → Status Bar.

Within each pane, tab order follows reading order (top to bottom, left to right).

### Focus Trapping

- **Dialogs** trap focus within the dialog until closed
- **Command palette** (`Ctrl+K`) traps focus until closed or selection made
- **Popovers** trap focus but close on outside click

### Focus Restoration

When a dialog/overlay closes, focus returns to the element that triggered it.

---

## 8. Loading States

| Scenario | Loading UI |
|----------|-----------|
| App startup | Splash: Orqa Studio anvil mark + "Loading..." |
| Project scanning | Nav Sub-Panel: skeleton loader for tree/list content |
| Session loading | Chat Panel: skeleton loader for message list |
| Artifact loading | Explorer Panel: skeleton loader for markdown content |
| Search | Command palette: spinner + results populate incrementally |

### Skeleton Loaders

Use animated gradient shimmer on placeholder blocks matching the expected content layout. Gray rectangles for text lines, circles for avatars, rounded rectangles for badges.

---

## 9. Error States

| Error | Display | Recovery |
|-------|---------|----------|
| Sidecar not found | Status bar: red dot. Alert in conversation: "Claude Code CLI not found." Link to settings. | Install CLI, configure path in settings |
| Sidecar crashed | Status bar: red dot. Toast: "Connection lost. Reconnecting..." Auto-restart attempt. | Auto-restart. Manual: click status bar. |
| Provider error (rate limit) | Error block in conversation: "Rate limited. Retry in {N}s." Retry button with countdown. | Wait and retry |
| Provider error (auth) | Error block: "Authentication failed." Link to settings. | Update API key / CLI path |
| File watcher error | Toast: "File watching interrupted for {project}." | Rescan project in settings |
| Save failed | Toast (error): "Failed to save {filename}." Retry button. | Fix permissions, retry |
| Network offline | Status bar: yellow dot. All send actions disabled. "Offline" label. | Reconnect automatically |

---

## 10. Empty States

Every empty container has a meaningful message and a single clear call to action.

| View | Message | Action |
|------|---------|--------|
| Session dropdown (empty) | "No conversations yet" | "Start a conversation" → focus input |
| Welcome (no project) | Orqa Studio anvil + "Welcome to Orqa Studio" + feature summary | "Open Project" / "New Project" buttons |
| Empty Nav Sub-Panel | "No docs found" or "No {category} defined" | "Add items to get started" |
| Nav Sub-Panel (Hooks, no hookify rules) | "No hookify rules configured" | "Create hookify rule" button |
| Empty artifact category | "No {agents/rules/skills} defined" | "Create new {type}" button |
| Empty hooks view | "No hooks configured. Add lifecycle hooks to .claude/hooks/ or hookify rules to .claude/hookify.*.local.md" | "Create lifecycle hook" / "Create hookify rule" buttons |
| Empty search results | "No results for '{query}'" | Suggest broader search terms |
| No scanner results | "No scanner results yet" | "Scanners run during implementation" |
| No metrics | "Not enough data for metrics" | "Metrics populate as you use Orqa Studio" |
