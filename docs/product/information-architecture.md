# Information Architecture

**Date:** 2026-03-02

How Forge's UI is structured. Defines the navigation model, view hierarchy, panel relationships, and what the user sees at each level of the application. This document drives wireframe design (Phase 0d) and component tree definition (Phase 0e).

---

## Layout Model

Forge uses a **four-zone VS Code-style layout**. The Activity Bar is a fixed CSS flex element; the three resizable zones (Explorer Panel, Sessions Panel, Chat Panel) are managed by PaneForge (shadcn-svelte Resizable).

```
┌──────────────────────────────────────────────────────────────────┐
│  Toolbar                                                          │
├────┬────────────────────┬─────────────┬──────────────────────────┤
│    │                    │             │                          │
│ AB │  Explorer Panel    │  Sessions   │   Chat Panel             │
│    │  (flexible)        │  Panel      │   (flexible)             │
│ 48 │                    │  (240px,    │                          │
│ px │  Artifact browser/ │  collapsible│   Always: Conversation   │
│    │  viewer/editor,    │  )          │                          │
│ I  │  Dashboards,       │             │                          │
│ C  │  Settings          │  Session    │                          │
│ O  │                    │  list +     │                          │
│ N  │                    │  Project    │                          │
│ S  │                    │  info       │                          │
│    │                    │             │                          │
├────┴────────────────────┴─────────────┴──────────────────────────┤
│  Status Bar                                                       │
└──────────────────────────────────────────────────────────────────┘
```

### Zone Dimensions

| Zone | Default | Min | Max | Collapsible |
|------|---------|-----|-----|-------------|
| Activity Bar | 48px (fixed) | 48px | 48px | No |
| Explorer Panel | Flex (fills remaining) | 280px | — | No |
| Sessions Panel | 240px | 180px | 320px | Yes → 0px |
| Chat Panel | Flex (fills remaining) | 360px | — | No |
| Toolbar | Full width | — | — | No |
| Status Bar | Full width | — | — | No |

Explorer and Chat share remaining space ~50/50 after Activity Bar (48px) and Sessions Panel (240px).

---

## Toolbar

The toolbar spans the full window width and provides global context and actions.

| Element | Position | Description |
|---------|----------|-------------|
| Project name | Left | Currently open project. Click to switch projects. |
| Global search | Center | FTS5-powered search across sessions and artifacts. `Ctrl+K` / `Cmd+K`. |
| New session | Right | Start a new conversation session. `Ctrl+N` / `Cmd+N`. |

> **Note:** Settings gear removed from toolbar — now accessible via the Activity Bar.

---

## Activity Bar

A 48px fixed-width vertical icon rail on the far left. Provides direct navigation to artifact categories, dashboards, and settings without tab bars.

### Icon Groups

**Group 1 — Artifact Categories:**

| Icon | Label | Lucide Icon |
|------|-------|-------------|
| Docs | Docs (default active) | FileText |
| Agents | Agents | Bot |
| Rules | Rules | Shield |
| Skills | Skills | Zap |
| Hooks | Hooks | GitBranch |

**Separator**

**Group 2 — Dashboards (Phase 3-5):**

| Icon | Label | Lucide Icon |
|------|-------|-------------|
| Scanners | Scanners | ScanLine |
| Metrics | Metrics | BarChart3 |
| Learning | Learning | Lightbulb |

**Separator (bottom-aligned):**

| Icon | Label | Lucide Icon |
|------|-------|-------------|
| Settings | Settings | Settings |

### Active State

Active icon: 2px left border indicator (`--primary` color) + highlighted background. Tooltip on hover.

### View Mapping

| Activity Bar Icon | Explorer Panel Shows |
|-------------------|---------------------|
| Docs (default) | Docs artifact list → click opens viewer/editor |
| Agents | Agents artifact list → click opens viewer/editor |
| Rules | Rules artifact list → click opens viewer/editor |
| Skills | Skills artifact list → click opens viewer/editor |
| Hooks | Hooks artifact list → click opens viewer/editor |
| Scanners | Scanner dashboard |
| Metrics | Metrics dashboard |
| Learning | Learning loop view |
| Settings | Settings view |

---

## Explorer Panel

The Explorer Panel is always visible. It shows content determined by the Activity Bar selection.

When an artifact category is active, the Explorer Panel shows the artifact list for that category. Clicking an artifact opens it in the viewer/editor within the Explorer Panel. When a dashboard icon is active, the Explorer Panel shows that dashboard. The old five-tab bar (Docs | Agents | Rules | Skills | Hooks) is eliminated — the Activity Bar provides direct navigation to each category.

### Artifact List View

| Element | Description |
|---------|-------------|
| Artifact list | Each entry shows: name, brief description (first sentence or frontmatter), status indicator. Click to open in the artifact viewer. |
| New button | Create a new artifact in the selected category from a template. |
| Search/filter | Text filter within the current category. |

### Artifact Viewer

When the user clicks an artifact in the list, the Explorer Panel switches from the list to the artifact viewer. The conversation remains visible in the Chat Panel so the user can discuss the artifact with Claude simultaneously.

| Element | Description |
|---------|-------------|
| Breadcrumb | Navigation context: Category > Artifact name. Click category to return to the list. |
| Rendered view | Markdown rendering of the artifact content. YAML frontmatter displayed as structured metadata above the body. |
| Edit mode | Toggle to CodeMirror 6 source editing. Full markdown + YAML editing with syntax highlighting. Save: `Ctrl+S` / `Cmd+S`. |
| Back button | Returns to the artifact list view. |

### Settings View

Application and project settings, shown when the Settings icon is active in the Activity Bar.

| Section | Contents |
|---------|----------|
| Provider | Sidecar status, Claude Code CLI path, connection health indicator. |
| Project | Project root, scan settings, file watcher status. |
| Appearance | Theme (light/dark/system), font size, panel defaults. Per-project theming toggle (inherit project design tokens or use Forge defaults). |
| Keyboard shortcuts | Reference card for all keyboard shortcuts. |

### Scanner Dashboard (Phase 3+)

| Element | Description |
|---------|-------------|
| Scanner list | Each scanner with last result: pass/fail, timestamp. |
| Trend chart | Pass/fail rate over time (LayerChart). |
| Violation details | Expandable list of current violations with file location and description. |

### Metrics Dashboard (Phase 5)

| Element | Description |
|---------|-------------|
| KPI cards | Each metric as a card: current value, trend indicator, sparkline. |
| Charts | Detailed time-series charts for selected metrics (LayerChart). |
| Lesson log | Recent IMPL and RETRO entries with promotion status. |

---

## Sessions Panel

The Sessions Panel is 240px by default and collapsible via `Ctrl+B`. It has two tabs at the top.

### Tab: Sessions (Default)

The default tab. Shows conversation session history for the current project.

| Element | Description |
|---------|-------------|
| Session list | Chronological list of sessions. Each entry shows: title (auto-generated or user-named), date, message count, preview snippet. |
| Active session | Highlighted. Clicking another session loads it in the Chat Panel. |
| Search filter | Text filter to narrow the session list. |

### Tab: Project

Project-level information.

| Element | Description |
|---------|-------------|
| Project info | Detected stack (languages, frameworks), project root path. |
| Governance summary | Counts of artifacts: N agents, N rules, N skills, N hooks. Click any category to activate it in the Activity Bar. |
| Quick links | Scanner status, metrics, learning — Phase 3-5. |

---

## Chat Panel

The Chat Panel is always visible and always shows the active conversation. It is positioned at the far right. It is not collapsible. The core workflow is collaborating with Claude *on* artifacts — the conversation must remain visible while viewing, editing, or discussing any artifact in the Explorer Panel.

### View: Conversation (Only View)

| Element | Description |
|---------|-------------|
| Message stream | Scrollable list of messages. User messages are right-aligned, assistant messages left-aligned. |
| Content blocks | Each message contains typed content blocks: text (rendered markdown), code (syntax-highlighted), tool call cards, tool result cards, error blocks. |
| Tool call cards | Collapsible. Summary shows: tool name, input summary, result summary, duration. Expanded shows: full input, full output, diff view (for edits). Badge indicates status: pending, approved, denied, completed. |
| Streaming indicator | When the AI is responding: typing indicator + streaming tokens appear character by character in the current message. |
| Input area | Bottom of panel. Multi-line text input with markdown support. `Enter` to send, `Shift+Enter` for newline. Attachment button for files (Phase 2+). |
| Session header | Top of panel: session title (editable), model indicator, token usage. |

---

## Navigation Model

Navigation uses the Activity Bar and contextual panel switching, not a traditional menu or route-based navigation. The four zones are always visible (unless Sessions Panel is collapsed); the user's "location" is determined by the Activity Bar selection and what's showing in each panel.

| Action | Activity Bar | Explorer Panel | Sessions Panel | Chat Panel |
|--------|-------------|----------------|----------------|------------|
| Start app | Docs active | Docs artifact list | Sessions tab, session list | Conversation (active or welcome) |
| Click Agents icon | Agents active | Agents artifact list | Unchanged | Unchanged |
| Click an artifact | Unchanged | Switches to artifact viewer | Unchanged | Unchanged |
| Click Settings icon | Settings active | Shows settings view | Unchanged | Unchanged |
| Click Scanners icon | Scanners active | Shows scanner dashboard | Unchanged | Unchanged |
| Click a session in list | Unchanged | Unchanged | Highlights session | Loads that session's conversation |
| `Ctrl+K` global search | Unchanged | Shows search results | Unchanged | Unchanged |

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+1` through `Ctrl+5` | Switch artifact category (Docs / Agents / Rules / Skills / Hooks) |
| `Ctrl+B` | Toggle Sessions Panel |
| `Ctrl+Shift+S` | Scanner dashboard |
| `Ctrl+Shift+M` | Metrics dashboard |
| `Ctrl+Shift+L` | Learning loop |
| `Ctrl+,` | Settings |
| `Ctrl+K` | Global search |
| `Ctrl+N` | New session |
| `Ctrl+E` | Toggle edit mode (artifact viewer) |
| `Ctrl+S` | Save (in edit mode) |
| `Escape` | Close overlay / exit edit mode |

**Removed:** `Ctrl+\` (detail panel toggle — Explorer is always visible), `Ctrl+Shift+A` (artifact browser — now Activity Bar icons).

---

## State Management

### URL-less Navigation

Forge is a desktop application, not a web app. There are no URLs or routes. Navigation state is managed through a single `NavigationStore` using Svelte 5 runes:

```typescript
type ActivityBarItem = "docs" | "agents" | "rules" | "skills" | "hooks"
  | "scanners" | "metrics" | "learning" | "settings";

type ExplorerView = "artifact-list" | "artifact-viewer" | "artifact-editor"
  | "scanner-dashboard" | "metrics-dashboard" | "learning-loop" | "settings";

class NavigationStore {
  activeActivity = $state<ActivityBarItem>("docs");
  explorerView = $state<ExplorerView>("artifact-list");
  selectedArtifact = $state<string | null>(null);
  sessionsPanelTab = $state<"sessions" | "project">("sessions");
  sessionsPanelCollapsed = $state(false);

  switchActivity(item: ActivityBarItem) { /* sets explorerView based on item */ }
  openArtifact(path: string) { /* sets selectedArtifact + explorerView */ }
  toggleSessionsPanel() { /* toggles collapse */ }
}
```

### Persistence

- **Window state** (size, position, panel widths): `tauri-plugin-window-state`
- **Session history**: SQLite
- **Active session**: Restored on app restart via last-used session ID in `tauri-plugin-store`
- **Panel collapse state**: Restored via `tauri-plugin-window-state`

---

## Focus Order

Toolbar → Activity Bar → Explorer Panel → Sessions Panel → Chat Panel → Status Bar

---

## Responsive Behavior

| Window Width | Layout |
|-------------|--------|
| > 1200px | All zones open |
| 900-1200px | Sessions Panel auto-collapsed |
| 720-900px | Sessions as overlay Sheet |
| < 720px | Activity Bar as floating toggle; Chat as overlay Sheet |

Collapse priority: Sessions Panel → Chat Panel (overlay) → Activity Bar (floating). Explorer never collapses (focal point).

---

## Empty States

Every view has a meaningful empty state that guides the user toward the next action.

| View | Empty State | Call to Action |
|------|------------|----------------|
| Session list | "No sessions yet" | "Start a conversation" button → focuses input |
| Conversation | Welcome message explaining Forge | "Type a message to begin" in input placeholder |
| Artifact list (no .claude/) | "No governance artifacts found" | "Open a project with .claude/ directory" or "Create your first agent" |
| Artifact list (empty category) | "No {category} defined" | "Create new {category}" button |
| Scanner dashboard | "No scanner results" | "Scanners run automatically during implementation" |
| Metrics dashboard | "Not enough data" | "Metrics populate as you use Forge" |

---

## Phase 1 Scope

The MVP includes only the views and elements needed for the core journeys:

**Included:**
- Four-zone layout with PaneForge + Activity Bar
- Toolbar (project name, new session)
- Activity Bar (5 artifact categories + settings)
- Explorer Panel (artifact browser, artifact viewer, settings)
- Sessions Panel (session list, project info)
- Chat Panel (conversation, streaming, tool calls read-only, input)
- Status bar (connection status, sidecar state)
- Empty states for all included views
- Keyboard shortcuts for core actions

**Deferred:**
- Scanners / Metrics / Learning Activity Bar icons (Phase 3-5)
- Global search (Phase 2 — FTS5 infrastructure exists but UI deferred)
- Tool inspector (Phase 2)
- Tool approval controls (Phase 2)

---

## Related Documents

- [User Journeys](/product/journeys) — Workflows that this architecture supports
- [User Personas](/product/personas) — Who navigates this UI
- [MVP Feature Specification](/product/mvp-specification) — What's included in Phase 1
- AD-013: Frontend library selections — shadcn-svelte, PaneForge, CodeMirror 6
- AD-018: Four-zone VS Code-style layout — layout architecture decision
- AD-014: Persistence architecture — SQLite for session/artifact storage
