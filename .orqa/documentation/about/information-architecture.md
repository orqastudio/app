---
id: DOC-040
title: Information Architecture
description: Information architecture defining how content is organized and navigated within the application.
created: 2026-03-02
updated: 2026-03-08
sort: 6
relationships:
  - target: AD-018
    type: documents
    rationale: Documentation page references AD-018
  - target: AD-013
    type: documents
    rationale: Documentation page references AD-013
  - target: AD-019
    type: documents
    rationale: Documentation page references AD-019
  - target: AD-014
    type: documents
    rationale: Documentation page references AD-014
---

**Date:** 2026-03-02

How OrqaStudio™'s UI is structured. Defines the navigation model, view hierarchy, panel relationships, and what the user sees at each level of the application. This document drives wireframe design and component tree definition.

---

## First Run Flow

On first launch, OrqaStudio performs an AI provider check to determine whether an AI provider is available and configured:

1. **Provider detected** — Status bar shows the provider connection status (e.g., "Provider: Connected"). All features are available. The user proceeds to open or create a project.
2. **Provider not detected** — OrqaStudio shows a prominent but non-blocking setup prompt explaining what AI provider is required and how to install or configure it. The rest of the UI (project browsing, artifact viewing and editing) remains functional without AI. The setup prompt appears in the Explorer Panel welcome state and persists until the provider is detected.

This ensures OrqaStudio is always useful for governance management (browsing and editing `.orqa/` artifacts) even if the AI provider is not yet installed, while making it clear that AI features require a configured provider.

## Layout Model

OrqaStudio uses a **three-zone + nav sub-panel layout**. The Activity Bar is a fixed CSS flex element; the three resizable zones (Nav Sub-Panel, Explorer Panel, Chat Panel) are managed by PaneForge (shadcn-svelte Resizable).

```
┌──────────────────────────────────────────────────────────────┐
│  Toolbar                                                      │
├────┬────────┬────────────────────┬───────────────────────────┤
│    │  Nav   │                    │  [Session▼] [Auto▼]       │
│ AB │  Sub   │  Explorer Panel    │                           │
│    │  Panel │  (viewer/editor/   │  Chat Panel               │
│ 48 │        │   dashboard/       │  (conversation)           │
│ px │  Tree/ │   settings)        │                           │
│    │  list  │                    │                           │
│ I  │  nav   │                    │                           │
│ C  │        │                    │                           │
│ O  │ 200px  │                    │                           │
│ N  │ collap │                    │                           │
│ S  │ sible  │                    │                           │
├────┴────────┴────────────────────┴───────────────────────────┤
│  Status Bar                                                   │
└──────────────────────────────────────────────────────────────┘
```

### Zone Dimensions

| Zone | Default | Min | Max | Collapsible |
|------|---------|-----|-----|-------------|
| Activity Bar | 48px (fixed) | 48px | 48px | No |
| Nav Sub-Panel | 200px | 160px | 280px | Yes → 0px |
| Explorer Panel | Flex (fills remaining) | 280px | — | No |
| Chat Panel | Flex (fills remaining) | 360px | — | No |
| Toolbar | Full width | — | — | No |
| Status Bar | Full width | — | — | No |

Nav Sub-Panel, Explorer, and Chat share remaining space after Activity Bar (48px). PaneForge manages the three resizable zones; the Activity Bar sits outside PaneForge.

---

## Toolbar

The toolbar spans the full window width and provides global context and actions.

| Element | Position | Description |
|---------|----------|-------------|
| Project name | Left | Currently open project. Click to switch projects. |
| Global search | Center | FTS5-powered search across sessions and artifacts. `Ctrl+K` / `Cmd+K`. |

> **Note:** New session button removed from toolbar — session creation is via `Ctrl+N` or the session dropdown in the Chat Panel header. Settings accessible via Activity Bar.

---

## Activity Bar

A 48px fixed-width vertical icon rail on the far left. Provides direct navigation to the Project Dashboard, artifact categories, dashboards, and settings.

### Icon Groups

**Top — Project Dashboard:**

| Icon | Label | Lucide Icon |
|------|-------|-------------|
| Project Dashboard | Project Dashboard | LayoutDashboard |

**Group 1 — Artifact Categories:**

| Icon | Label | Lucide Icon |
|------|-------|-------------|
| Docs | Docs (default active) | FileText |
| Agents | Agents | Bot |
| Rules | Rules | Shield |
| Skills | Skills | Zap |
| Hooks | Hooks (lifecycle hooks + hookify) | GitBranch |

**Separator**

**Group 2 — Dashboards (post-MVP):**

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

| Activity Bar Icon | Nav Sub-Panel Shows | Explorer Panel Shows |
|-------------------|---------------------|---------------------|
| Project Dashboard | Hidden | Project dashboard |
| Docs (default) | Doc tree (sections > pages) | Selected doc viewer/editor |
| Agents | Agent list | Selected agent viewer/editor |
| Rules | Rule list | Selected rule viewer/editor |
| Skills | Skill list | Selected skill viewer/editor |
| Hooks | Hook list (lifecycle + hookify) | Selected hook viewer/editor |
| Scanners | Scanner list | Scanner results |
| Metrics | Metric categories | Metrics dashboard |
| Learning | Lesson/retro list | Selected lesson/retro |
| Settings | Settings categories | Settings form |

---

## Nav Sub-Panel

A collapsible 200px panel between the Activity Bar and Explorer Panel. Provides per-category sub-navigation. Toggle with `Ctrl+B`.

### Docs Category (Structured Tree)

The Docs category uses a hierarchical tree that mirrors the `.orqa/documentation/` directory structure:

```
Product
  ├── Vision
  ├── Governance
  ├── Personas
  ├── Journeys
  ├── Information Architecture
  ├── MVP Specification
  ├── Glossary
  └── Roadmap
Architecture
  ├── Decisions
  ├── IPC Commands
  ├── Rust Modules
  ├── Svelte Components
  ├── Streaming Pipeline
  ├── Tool Definitions
  ├── MCP Host
  ├── Error Taxonomy
  ├── SQLite Schema
  └── Wireframe Serving
UI
  ├── Design System
  ├── Brand Identity
  ├── Component Inventory
  ├── Interaction Patterns
  ├── Responsive Behavior
  └── Wireframes
    ├── Core Layout
    ├── Conversation View
    ├── Artifact Browser
    ├── Dashboard
    └── Settings & Onboarding
Development
  ├── Getting Started
  ├── Coding Standards
  └── Lessons
Process
  ├── Team
  ├── Orchestration
  ├── Definition of Ready
  ├── Definition of Done
  └── Retrospectives
Research
  ├── AI Provider Integration
  ├── Tauri v2
  ├── Frontend
  └── Persistence
```

Tree expand/collapse state persists across navigation. Single-click selects a page and loads it in the Explorer Panel viewer.

### Other Artifact Categories

Agents, Rules, Skills, and Hooks use a flat or categorized list with a search/filter input at the top. Click an item to open it in the Explorer Panel.

### Dashboard Categories

When Project Dashboard, Scanners, or Metrics is active, the Nav Sub-Panel is hidden — all content goes directly to the Explorer Panel.

When Learning is active, the Nav Sub-Panel shows a lesson/retro list; clicking an item opens it in the Explorer Panel.

### Settings

Settings shows a category list in the Nav Sub-Panel (Provider, Project, Appearance, Keyboard Shortcuts). Clicking a category shows the corresponding settings form in the Explorer Panel.

---

## Explorer Panel

The Explorer Panel is always visible. It shows content determined by the Activity Bar selection and the Nav Sub-Panel selection.

When an artifact category is active, the Explorer Panel shows the selected artifact's viewer/editor. If no artifact is selected, it shows the artifact list for that category (fallback when Nav Sub-Panel is collapsed). When a dashboard icon is active, the Explorer Panel shows that dashboard.

### Artifact Viewer

When the user clicks an artifact in the Nav Sub-Panel (or in the fallback list), the Explorer Panel shows the artifact viewer.

| Element | Description |
|---------|-------------|
| Breadcrumb | Navigation context: Category > Section > Artifact name. Click any level to navigate up. |
| Rendered view | Markdown rendering of the artifact content. YAML frontmatter displayed as structured metadata above the body. |
| Edit mode | Toggle to CodeMirror 6 source editing. Full markdown + YAML editing with syntax highlighting. Save: `Ctrl+S` / `Cmd+S`. |

### Project Dashboard

Shown when the Project Dashboard Activity Bar icon is active. Nav Sub-Panel is hidden.

| Element | Description |
|---------|-------------|
| Project info | Detected stack (languages, frameworks), project root path. |
| Governance summary | Counts of artifacts: N agents, N rules, N skills, N hooks. Click any category to activate it in the Activity Bar. |
| Quick links | Scanner status, metrics, learning — post-MVP features. |
| Recent sessions | Last 5 sessions with quick-resume links. |

### Settings View

Application and project settings, shown when the Settings icon is active in the Activity Bar. Settings category list in the Nav Sub-Panel.

| Section | Contents |
|---------|----------|
| Provider | Sidecar status, active AI provider name and path, connection health indicator. |
| Project | Project root, scan settings, file watcher status. |
| Appearance | Theme (light/dark/system), font size, panel defaults. Per-project theming toggle. |
| Keyboard shortcuts | Reference card for all keyboard shortcuts. |

### Scanner Dashboard (post-MVP)

| Element | Description |
|---------|-------------|
| Scanner list | Each scanner with last result: pass/fail, timestamp (in Nav Sub-Panel when active). |
| Trend chart | Pass/fail rate over time (LayerChart). |
| Violation details | Expandable list of current violations with file location and description. |

### Metrics Dashboard (post-MVP)

| Element | Description |
|---------|-------------|
| KPI cards | Each metric as a card: current value, trend indicator, sparkline. |
| Charts | Detailed time-series charts for selected metrics (LayerChart). |
| Lesson log | Recent IMPL and RETRO entries with promotion status. |

---

## Chat Panel

The Chat Panel is always visible and always shows the active conversation. It is positioned at the far right. It is not collapsible. The core workflow is collaborating with the AI *on* artifacts — the conversation must remain visible while viewing, editing, or discussing any artifact in the Explorer Panel.

### Session Header

The session header sits at the top of the Chat Panel and provides session context and switching.

| Element | Description |
|---------|-------------|
| Session dropdown | Active session title (clickable). Opens a dropdown with: recent sessions list, search filter, "New Session" button. |
| Model selector | Auto / Opus / Sonnet / Haiku dropdown. "Auto (recommended)" is the default — separated from specific models by a visual divider. |
| Provider status | AI provider connection status (e.g., "Provider: Connected" or "Provider: Not found"). In the current implementation using Claude Code CLI, this shows the CLI version (e.g., "CLI: v1.2.3"). |
| Token usage | Token count for the current session. |

### Auto-Session on Plan Mode

When a conversation triggers plan mode (user says "plan this", "how should we build X", or Claude determines planning is needed), OrqaStudio automatically:
1. Creates a new session titled `[Plan] <topic>`
2. Switches to the new session in the Chat Panel
3. Preserves the previous session in history

This keeps sessions focused on a single concern and makes session history a meaningful navigable record.

**Trigger:** Both explicit user requests ("plan this feature") and autonomous AI-initiated plan mode.

**Configuration:** Users can disable auto-session in Settings > Project.

### Conversation View

| Element | Description |
|---------|-------------|
| Message stream | Scrollable list of messages. User messages are right-aligned, assistant messages left-aligned. |
| Content blocks | Each message contains typed content blocks: text (rendered markdown), code (syntax-highlighted), tool call cards, tool result cards, error blocks. |
| Tool call cards | Collapsible. Summary shows: tool name, input summary, result summary, duration. Expanded shows: full input, full output, diff view (for edits). Badge indicates status: pending, approved, denied, completed. |
| Streaming indicator | When the AI is responding: typing indicator + streaming tokens appear character by character in the current message. |
| Input area | Bottom of panel. Multi-line text input with markdown support. `Enter` to send, `Shift+Enter` for newline. Attachment button for files (post-MVP). |

---

## Navigation Model

Navigation uses the Activity Bar, Nav Sub-Panel, and contextual panel switching — not a traditional menu or route-based navigation. All zones are always visible (unless Nav Sub-Panel is collapsed); the user's "location" is determined by the Activity Bar selection and the Nav Sub-Panel selection.

| Action | Activity Bar | Nav Sub-Panel | Explorer Panel | Chat Panel |
|--------|-------------|---------------|----------------|------------|
| Start app | Docs active | Doc tree | Docs overview | Conversation (active or welcome) |
| Click Agents icon | Agents active | Agent list | Agent list (if none selected) | Unchanged |
| Click an artifact in Nav | Unchanged | Highlights item | Opens artifact viewer | Unchanged |
| Click Project Dashboard | Dashboard active | Hidden | Project dashboard | Unchanged |
| Click Settings icon | Settings active | Settings categories | Settings form | Unchanged |
| Click Scanners icon | Scanners active | Scanner list | Scanner results | Unchanged |
| Open session dropdown | Unchanged | Unchanged | Unchanged | Shows session list dropdown |
| Click a session | Unchanged | Unchanged | Unchanged | Loads session conversation |
| `Ctrl+K` global search | Unchanged | Unchanged | Shows search results | Unchanged |

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+0` | Project Dashboard |
| `Ctrl+1` through `Ctrl+5` | Switch artifact category (Docs / Agents / Rules / Skills / Hooks) |
| `Ctrl+B` | Toggle Nav Sub-Panel |
| `Ctrl+,` | Settings |
| `Ctrl+K` | Global search |
| `Ctrl+N` | New session |
| `Ctrl+E` | Toggle edit mode (artifact viewer) |
| `Ctrl+S` | Save (in edit mode) |
| `Escape` | Close overlay / exit edit mode |

**Removed from [AD-018](AD-018):** `Ctrl+Shift+S` (Scanner dashboard), `Ctrl+Shift+M` (Metrics dashboard), `Ctrl+Shift+L` (Learning loop) — these are Activity Bar destinations, not keyboard shortcut targets. `Ctrl+B` repurposed from Sessions Panel toggle to Nav Sub-Panel toggle.

---

## State Management

### URL-less Navigation

OrqaStudio is a desktop application, not a web app. There are no URLs or routes. Navigation state is managed through a single `NavigationStore` using Svelte 5 runes:

```typescript
type ActivityBarItem = "project-dashboard" | "docs" | "agents" | "rules" | "skills" | "hooks"
  | "scanners" | "metrics" | "learning" | "settings";

type ExplorerView = "artifact-list" | "artifact-viewer" | "artifact-editor"
  | "project-dashboard" | "scanner-dashboard" | "metrics-dashboard" | "learning-loop" | "settings";

class NavigationStore {
  activeActivity = $state<ActivityBarItem>("docs");
  explorerView = $state<ExplorerView>("artifact-list");
  selectedArtifact = $state<string | null>(null);
  navPanelCollapsed = $state(false);

  switchActivity(item: ActivityBarItem) { /* sets explorerView + navPanel visibility based on item */ }
  openArtifact(path: string) { /* sets selectedArtifact + explorerView */ }
  toggleNavPanel() { /* toggles collapse */ }
}
```

### Session State

```typescript
class SessionStore {
  // ... existing session fields ...
  sessionDropdownOpen = $state(false);
  sessionSearchFilter = $state("");
}
```

### Auto-Session State

```typescript
class ConversationStore {
  // ... existing conversation fields ...
  // When plan mode is detected, create new session automatically
  autoSessionEnabled = $state(true); // user-configurable
}
```

### Persistence

- **Window state** (size, position, panel widths): `tauri-plugin-window-state`
- **Session history**: SQLite
- **Active session**: Restored on app restart via last-used session ID in `tauri-plugin-store`
- **Panel collapse state**: Restored via `tauri-plugin-window-state`
- **Nav Sub-Panel tree expand/collapse state**: `tauri-plugin-store`

---

## Focus Order

Toolbar → Activity Bar → Nav Sub-Panel → Explorer Panel → Chat Panel → Status Bar

---

## Responsive Behavior

| Window Width | Layout |
|-------------|--------|
| > 1200px | All zones open |
| 900-1200px | Nav Sub-Panel auto-collapsed |
| 720-900px | Nav Sub-Panel as overlay Sheet |
| < 720px | Activity Bar as floating toggle; Chat as overlay Sheet |

Collapse priority: Nav Sub-Panel → Chat Panel (overlay) → Activity Bar (floating). Explorer never collapses (focal point).

---

## Empty States

Every view has a meaningful empty state that guides the user toward the next action.

| View | Empty State | Call to Action |
|------|------------|----------------|
| Session dropdown | "No sessions yet" | "Start a conversation" prompt in input area |
| Conversation | Welcome message explaining OrqaStudio | "Type a message to begin" in input placeholder |
| Artifact list (no .orqa/) | "No governance framework detected" | "Set up governance for this project" button (initialises `.orqa/` scaffold) |
| Artifact list (empty category) | "No {category} defined" | "Create new {category}" button |
| Nav Sub-Panel (empty tree) | "No docs found" | "Add documentation to your `.orqa/documentation/` directory" |
| Project Dashboard (no project) | "No project open" | "Open a project" button |
| Scanner dashboard | "No scanner results" | "Scanners run automatically during implementation" |
| Metrics dashboard | "Not enough data" | "Metrics populate as you use OrqaStudio" |

---

## MVP Scope

The MVP includes only the views and elements needed for the core journeys:

**Included:**
- Three-zone layout with PaneForge + Activity Bar + Nav Sub-Panel
- Toolbar (project name)
- Activity Bar (Project Dashboard + 5 artifact categories + settings)
- Nav Sub-Panel (doc tree for Docs, flat lists for other categories)
- Explorer Panel (artifact browser, artifact viewer, project dashboard, settings)
- Chat Panel (conversation, streaming, tool calls read-only, input, session dropdown)
- Session dropdown (session switching, new session creation)
- Status bar (connection status, sidecar state, model display)
- Empty states for all included views
- Keyboard shortcuts for core actions

**Deferred:**
- Scanners / Metrics / Learning Activity Bar icons (post-MVP)
- Global search (post-MVP — FTS5 infrastructure exists but UI deferred)
- Tool inspector (post-MVP)
- Tool approval controls (post-MVP)
- Auto-session on plan mode (post-MVP — requires sidecar plan-mode detection)

---

## CLI Interoperability

All artifact changes in OrqaStudio are bidirectional with external tools:

- **OrqaStudio to external tools** — When a user creates or edits an artifact in OrqaStudio's artifact editor, the file is written to the `.orqa/` directory on disk (the source of truth). External tools that use symlinks (e.g., Claude Code CLI via `.claude/`) see the updated file immediately through the symlink.
- **External tools to OrqaStudio** — When an external tool modifies an `.orqa/` artifact directly (or through a symlink), OrqaStudio's file watcher detects the change within 500ms and updates the artifact browser and viewer.
- **No sync layer** — There is no synchronization protocol. All tools read and write the same files on disk. `.orqa/` is the shared source of truth.

This design means the AI provider status shown in the status bar is meaningful context: it tells the user whether AI-powered features are available, and it confirms that their governance artifacts are being read by the same provider their agents use.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Learning Through Reflection | N/A |
| Clarity Through Structure | Defines the navigation structure that makes governance artifacts (agents, rules, skills, hooks) browsable, editable, and first-class destinations in the UI — making governance tangible and manageable. |

---

## Related Documents

- User Journeys — Workflows that this architecture supports
- User Personas — Who navigates this UI
- MVP Feature Specification — What the MVP includes
- [AD-013](AD-013): Frontend library selections — shadcn-svelte, PaneForge, CodeMirror 6
- [AD-019](AD-019): Three-zone + Nav Sub-Panel layout — layout architecture decision
- [AD-014](AD-014): Persistence architecture — SQLite for session/artifact storage
