# Information Architecture

**Date:** 2026-03-02

How Forge's UI is structured. Defines the navigation model, view hierarchy, panel relationships, and what the user sees at each level of the application. This document drives wireframe design (Phase 0d) and component tree definition (Phase 0e).

---

## Layout Model

Forge uses a **three-pane layout** managed by PaneForge (shadcn-svelte Resizable):

```
┌─────────────────────────────────────────────────────────────┐
│  Toolbar                                                     │
├──────────┬──────────────────────────────┬───────────────────┤
│          │                              │                   │
│ Sidebar  │      Primary Panel           │   Detail Panel    │
│ (240px)  │      (flexible)              │   (360px)         │
│          │                              │                   │
│ Collapse │                              │  Collapse ←       │
│  →       │                              │                   │
│          │                              │                   │
│          │                              │                   │
│          │                              │                   │
│          │                              │                   │
├──────────┴──────────────────────────────┴───────────────────┤
│  Status Bar                                                  │
└─────────────────────────────────────────────────────────────┘
```

### Pane Behavior

| Pane | Default Width | Min Width | Collapsible | Content |
|------|--------------|-----------|-------------|---------|
| Sidebar | 240px | 180px | Yes (left) | Navigation, session history, project info |
| Primary | Flexible (fills remaining) | 400px | No | Main content — always conversation |
| Detail | 360px | 280px | Yes (right) | Contextual detail — artifact browser, settings, inspector |
| Toolbar | Full width | — | No | Project name, actions, global search |
| Status Bar | Full width | — | No | Connection status, sidecar state, usage indicators |

---

## Toolbar

The toolbar spans the full window width and provides global context and actions.

### Contents

| Element | Position | Description |
|---------|----------|-------------|
| Project name | Left | Currently open project. Click to switch projects. |
| Global search | Center | FTS5-powered search across sessions and artifacts. `Ctrl+K` / `Cmd+K`. |
| New session | Right | Start a new conversation session. `Ctrl+N` / `Cmd+N`. |
| Settings gear | Right | Open settings in detail panel. |

---

## Sidebar

The sidebar provides navigation and session history. It has two tabs at the top.

### Tab: Sessions

The default sidebar tab. Shows conversation session history for the current project.

| Element | Description |
|---------|-------------|
| Session list | Chronological list of sessions. Each entry shows: title (auto-generated or user-named), date, message count, preview snippet. |
| Active session | Highlighted. Clicking another session loads it in the primary panel. |
| Search filter | Text filter to narrow the session list. |

### Tab: Project

Project-level navigation.

| Element | Description |
|---------|-------------|
| Project info | Detected stack (languages, frameworks), project root path. |
| Governance summary | Counts of artifacts: N agents, N rules, N skills, N hooks. Click any category to open it in the detail panel artifact browser. |
| Quick links | Jump to: Agents, Rules, Skills, Hooks, Documentation, Architecture Decisions. |
| Scanner status | Summary of last scanner run: N pass, N fail. Click to open scanner dashboard in detail panel. (Phase 3+) |
| Metrics summary | Sessions today, scan pass rate. Click to open metrics dashboard in detail panel. (Phase 5) |
| Learning summary | N lessons captured, N promoted. Click to open learning loop in detail panel. (Phase 5) |

---

## Primary Panel

The primary panel **always** shows the active conversation. It is never replaced by another view. The core workflow is collaborating with Claude *on* artifacts — the conversation must remain visible while viewing, editing, or discussing any artifact.

### View: Conversation (Only View)

The active conversation session. This is where the user interacts with the AI.

| Element | Description |
|---------|-------------|
| Message stream | Scrollable list of messages. User messages are right-aligned, assistant messages left-aligned. |
| Content blocks | Each message contains typed content blocks: text (rendered markdown), code (syntax-highlighted), tool call cards, tool result cards, error blocks. |
| Tool call cards | Collapsible. Summary shows: tool name, input summary, result summary, duration. Expanded shows: full input, full output, diff view (for edits). Badge indicates status: pending, approved, denied, completed. |
| Streaming indicator | When the AI is responding: typing indicator + streaming tokens appear character by character in the current message. |
| Input area | Bottom of panel. Multi-line text input with markdown support. `Enter` to send, `Shift+Enter` for newline. Attachment button for files (Phase 2+). |
| Session header | Top of panel: session title (editable), model indicator, token usage. |

---

## Detail Panel

The detail panel provides contextual information alongside the primary panel. It has multiple views selectable by tabs or navigation context.

### View: Artifact Browser (Default when opened)

Browse governance artifacts by category.

| Element | Description |
|---------|-------------|
| Category tabs | Docs (default), Agents, Rules, Skills, Hooks. Each tab shows a list of artifacts in that category. Docs is the default because documentation is the most frequently touched artifact during active development; governance artifacts (Agents, Rules, etc.) are primarily modified during retrospectives. |
| Artifact list | Each entry shows: name, brief description (first sentence or frontmatter), status indicator. Click to open in the artifact viewer (replaces the browser in the detail panel). |
| New button | Create a new artifact in the selected category from a template. |
| Search/filter | Text filter within the current category. |

### View: Artifact Viewer

When the user clicks an artifact in the browser, the detail panel switches from the browser list to the artifact viewer. The conversation remains visible in the primary panel so the user can discuss the artifact with Claude simultaneously.

| Element | Description |
|---------|-------------|
| Breadcrumb | Navigation context: Category > Artifact name. Click category to return to the browser list. |
| Rendered view | Markdown rendering of the artifact content. YAML frontmatter displayed as structured metadata above the body. |
| Edit mode | Toggle to CodeMirror 6 source editing. Full markdown + YAML editing with syntax highlighting. Save: `Ctrl+S` / `Cmd+S`. |
| Back button | Returns to the artifact browser list view. |

### View: Settings

Application and project settings.

| Section | Contents |
|---------|----------|
| Provider | Sidecar status, Claude Code CLI path, connection health indicator. |
| Project | Project root, scan settings, file watcher status. |
| Appearance | Theme (light/dark/system), font size, panel defaults. Per-project theming toggle (inherit project design tokens or use Forge defaults). |
| Keyboard shortcuts | Reference card for all keyboard shortcuts. |

### View: Tool Inspector (Phase 2+)

Detailed view of a specific tool call, triggered by clicking a tool call card in the conversation.

| Element | Description |
|---------|-------------|
| Tool name + type | e.g., "Edit — docs/product/roadmap.md" |
| Input | Full input parameters with syntax highlighting. |
| Output | Full output/result with syntax highlighting. |
| Diff view | For file edits: unified diff with additions/deletions highlighted. |
| Approval controls | Approve / Deny / Modify buttons (Phase 2+). |

### View: Scanner Dashboard (Phase 3+)

Scanner results and trends.

| Element | Description |
|---------|-------------|
| Scanner list | Each scanner with last result: pass/fail, timestamp. |
| Trend chart | Pass/fail rate over time (LayerChart). |
| Violation details | Expandable list of current violations with file location and description. |

### View: Metrics Dashboard (Phase 5)

Learning loop metrics and KPIs.

| Element | Description |
|---------|-------------|
| KPI cards | Each metric as a card: current value, trend indicator, sparkline. |
| Charts | Detailed time-series charts for selected metrics (LayerChart). |
| Lesson log | Recent IMPL and RETRO entries with promotion status. |

---

## Navigation Model

### Primary Navigation

Navigation uses the sidebar tabs and contextual panel switching, not a traditional menu or route-based navigation. The three panels are always visible (unless collapsed); the user's "location" is determined by what's showing in each panel.

| Action | Sidebar | Primary | Detail |
|--------|---------|---------|--------|
| Start app | Sessions tab, session list | Conversation (active or welcome) | Collapsed |
| Click a session | Highlights session | Loads that session's conversation | Unchanged |
| Click "Agents" in Project tab | Unchanged | Unchanged | Opens artifact browser, Agents tab |
| Click an artifact in browser | Unchanged | Unchanged | Switches to artifact viewer for that artifact |
| Click settings gear | Unchanged | Unchanged | Opens settings view |
| Click scanner status in Project tab | Unchanged | Unchanged | Opens scanner dashboard (Phase 3+) |
| Click metrics summary in Project tab | Unchanged | Unchanged | Opens metrics dashboard (Phase 5) |
| Click learning summary in Project tab | Unchanged | Unchanged | Opens learning loop (Phase 5) |
| Click a tool call card | Unchanged | Unchanged | Opens tool inspector (Phase 2+) |
| `Ctrl+K` global search | Unchanged | Unchanged | Opens search results in detail panel |

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` / `Cmd+K` | Global search |
| `Ctrl+N` / `Cmd+N` | New session |
| `Ctrl+B` / `Cmd+B` | Toggle sidebar |
| `Ctrl+\` / `Cmd+\` | Toggle detail panel |
| `Ctrl+E` / `Cmd+E` | Toggle edit mode (in artifact viewer) |
| `Ctrl+S` / `Cmd+S` | Save (in edit mode) |
| `Escape` | Close detail panel / exit edit mode |
| `Ctrl+Shift+A` / `Cmd+Shift+A` | Open artifact browser |
| `Ctrl+Shift+S` / `Cmd+Shift+S` | Open scanner dashboard (Phase 3+) |
| `Ctrl+Shift+M` / `Cmd+Shift+M` | Open metrics dashboard (Phase 5) |
| `Ctrl+Shift+L` / `Cmd+Shift+L` | Open learning loop (Phase 5) |

---

## State Management

### URL-less Navigation

Forge is a desktop application, not a web app. There are no URLs or routes. Navigation state is managed through Svelte stores:

| Store | Type | Purpose |
|-------|------|---------|
| `activeSession` | `$state` | Currently displayed session ID |
| `detailView` | `$state` | "artifact-browser", "artifact-viewer", "settings", "tool-inspector", "scanner-dashboard", "metrics" |
| `selectedArtifact` | `$state` | Currently viewed/edited artifact path |
| `sidebarTab` | `$state` | "sessions" or "project" |
| `sidebarCollapsed` | `$state` | Boolean |
| `detailCollapsed` | `$state` | Boolean |

### Persistence

- **Window state** (size, position, panel widths): `tauri-plugin-window-state`
- **Session history**: SQLite
- **Active session**: Restored on app restart via last-used session ID in `tauri-plugin-store`
- **Panel collapse state**: Restored via `tauri-plugin-window-state`

---

## Empty States

Every view has a meaningful empty state that guides the user toward the next action.

| View | Empty State | Call to Action |
|------|------------|----------------|
| Session list | "No sessions yet" | "Start a conversation" button → focuses input |
| Conversation | Welcome message explaining Forge | "Type a message to begin" in input placeholder |
| Artifact browser (no .claude/) | "No governance artifacts found" | "Open a project with .claude/ directory" or "Create your first agent" |
| Artifact browser (empty category) | "No {category} defined" | "Create new {category}" button |
| Scanner dashboard | "No scanner results" | "Scanners run automatically during implementation" |
| Metrics dashboard | "Not enough data" | "Metrics populate as you use Forge" |

---

## Phase 1 Scope

The MVP includes only the views and elements needed for the core journeys (1, 3, 4 partial):

**Included:**
- Three-pane layout with PaneForge
- Toolbar (project name, new session, settings gear)
- Sidebar: Sessions tab (session list)
- Sidebar: Project tab (project info, governance summary, quick links)
- Primary: Conversation view (messages, streaming, tool call cards read-only, input) — always visible
- Detail: Artifact browser (browse by category)
- Detail: Artifact viewer (rendered markdown, edit mode) — opens when clicking an artifact, conversation stays visible
- Detail: Settings (provider status, project info, appearance)
- Status bar (connection status, sidecar state)
- Empty states for all included views
- Keyboard shortcuts for core actions

**Deferred:**
- Detail: Tool inspector (Phase 2)
- Detail: Scanner dashboard (Phase 3)
- Detail: Metrics dashboard (Phase 5)
- Global search (Phase 2 — FTS5 infrastructure exists but UI deferred)
- Tool approval controls (Phase 2)

---

## Related Documents

- [User Journeys](/product/journeys) — Workflows that this architecture supports
- [User Personas](/product/personas) — Who navigates this UI
- [MVP Feature Specification](/product/mvp-specification) — What's included in Phase 1
- AD-013: Frontend library selections — shadcn-svelte, PaneForge, CodeMirror 6
- AD-014: Persistence architecture — SQLite for session/artifact storage
