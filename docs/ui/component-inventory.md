# Component Inventory

**Date:** 2026-03-02 | **Informed by:** [Frontend Research](/research/frontend), [Design System](/ui/design-system), [Information Architecture](/product/information-architecture), [Wireframes](/ui/wireframes/)

Complete inventory of UI components needed for Phase 1. Split into shadcn-svelte library components (use as-is), custom application components (must be built), and custom markdown rendering blocks.

---

## shadcn-svelte Components (Library)

Components used directly from the shadcn-svelte library. No custom code needed beyond standard usage.

### Core UI

| Component | Usage | Phase |
|-----------|-------|-------|
| **Button** | Send message, new session, save, cancel, create artifact | 1 |
| **Input** | Search filters, session search, artifact search, settings fields | 1 |
| **Textarea** | Message input (multi-line) | 1 |
| **Label** | Form labels in settings | 1 |
| **Checkbox** | Settings toggles (e.g., per-project theming) | 1 |
| **Select** | Model selector, theme mode (light/dark/system) | 1 |
| **Separator** | Visual dividers between sections | 1 |

### Layout

| Component | Usage | Phase |
|-----------|-------|-------|
| **Resizable** (PaneForge) | Three-zone resizable layout (Nav Sub-Panel, Explorer, Chat) within PaneForge | 1 |
| **ScrollArea** | Message stream, artifact list, session list | 1 |
| **Collapsible** | Tool call cards (collapsed/expanded), settings sections | 1 |

### Data Display

| Component | Usage | Phase |
|-----------|-------|-------|
| **Badge** | Tool call status (Completed/Error/Pending), artifact type, connection status | 1 |
| **Card** | Message bubbles, tool call cards, KPI cards (Phase 5), artifact summary cards | 1 |
| **Table** | Keyboard shortcuts reference, settings display | 1 |
| **Tooltip** | Truncated text hover, icon descriptions, shortcut hints | 1 |

### Overlay

| Component | Usage | Phase |
|-----------|-------|-------|
| **Dialog** | Confirmation dialogs (delete artifact, discard changes), project creation | 1 |
| **Popover** | Project switcher, model info | 1 |
| **Sheet** | Mobile/narrow responsive alternative to collapsed panels | 1 |
| **Command** | Global search (`Ctrl+K`) with result list | 1 |
| **DropdownMenu** | Context menus, session actions, artifact actions | 1 |

### Feedback

| Component | Usage | Phase |
|-----------|-------|-------|
| **Alert** | Error messages, warnings, first-run guidance | 1 |
| **Sonner** (Toast) | Save confirmation, copy-to-clipboard, connection status changes | 1 |

---

## Custom Application Components (Must Build)

Components specific to Forge that are not provided by shadcn-svelte.

### Layout Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `ActivityBar` | Fixed 48px vertical icon rail. Renders artifact category icons, dashboard icons, and settings icon. Manages active state with 2px left border indicator. | Button, Tooltip | 1 |
| `ActivityBarItem` | Individual icon button within the Activity Bar. Receives icon, label, active state, shortcut hint as props. | Button, Tooltip | 1 |
| `NavSubPanel` | Collapsible 200px panel between Activity Bar and Explorer. Renders per-category navigation: DocTreeNav for Docs, ArtifactListNav for other categories. Toggle via Ctrl+B. | ScrollArea, Collapsible | 1 |

### Conversation Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `ConversationView` | Full conversation panel: header, message stream, input area. Manages scroll position, streaming state, auto-scroll behavior. | ScrollArea, Card | 1 |
| `SessionHeader` | Session dropdown (session switching), model selector dropdown, token usage counter. | DropdownMenu, Select, Badge | 1 |
| `MessageBubble` | Single message rendering. Accepts role (user/assistant/system), content blocks, timestamp. | Card | 1 |
| `UserMessage` | User message variant. Shows person icon, name, timestamp, rendered markdown content. | MessageBubble | 1 |
| `AssistantMessage` | Assistant message variant. Shows AI icon, name, timestamp, rendered content + inline tool call cards. | MessageBubble, ToolCallCard | 1 |
| `SystemMessage` | System message variant. Muted styling, centered, used for session context and errors. | MessageBubble | 1 |
| `MessageInput` | Multi-line text input with Enter-to-send, Shift+Enter for newline. Send button. Placeholder text. | Textarea, Button | 1 |
| `StreamingIndicator` | Animated dots / typing indicator shown while AI is generating. | — | 1 |

### Tool Call Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `ToolCallCard` | Collapsible card for a single tool call. Shows tool name, input summary, status badge. Expands to show full input/output. | Collapsible, Card, Badge | 1 |
| `ToolCallInput` | Formatted display of tool call input parameters. Syntax-highlighted JSON/code. | CodeBlock | 1 |
| `ToolCallOutput` | Formatted display of tool call result. Handles text, code, diff, error. | CodeBlock, DiffView | 1 |
| `DiffView` | Unified diff display with green additions and red deletions. For Edit/Write tool calls. | — | 1 |
| `ToolApprovalControls` | Approve / Deny / Modify buttons for pending tool calls. | Button, Badge | 2 |

### Content Rendering Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `MarkdownRenderer` | Renders markdown content using `@humanspeak/svelte-markdown`. Custom renderers for code blocks, links, images. | `svelte-markdown` | 1 |
| `CodeBlock` | Syntax-highlighted code block with language label and copy button. | `svelte-highlight`, Button | 1 |
| `MarkdownEditor` | CodeMirror 6 source editor for markdown/YAML files. Save (`Ctrl+S`), cancel, unsaved indicator. | `svelte-codemirror-editor` | 1 |
| `FrontmatterDisplay` | Renders YAML frontmatter as structured metadata badges/cards above markdown body. | Badge, Card | 1 |

### Artifact Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `ArtifactBrowser` | Category-tabbed list of governance artifacts. Search filter. New button. Hooks category renders both lifecycle hooks and hookify rules with a subtype filter (All / Lifecycle / Hookify). | Tabs, ScrollArea, Input, Button | 1 |
| `ArtifactListItem` | Single artifact in the browser list. Icon, name, description snippet, click to open. For hookify rules, shows event type (file/bash), action (block/warn), and pattern summary instead of frontmatter description. | Badge | 1 |
| `ArtifactViewer` | Rendered view of a single artifact. Breadcrumb, frontmatter display, markdown body, edit toggle. | MarkdownRenderer, FrontmatterDisplay | 1 |
| `ArtifactEditor` | Source editing view. CodeMirror 6 with markdown+YAML support. Save/cancel actions. | MarkdownEditor | 1 |
| `Breadcrumb` | Navigation breadcrumb: Project > Category > Artifact name. | — | 1 |

### Navigation Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `DocTreeNav` | Hierarchical tree navigation for the Docs category. Mirrors docs/ directory structure. Expand/collapse state persists. | Collapsible, ScrollArea | 1 |
| `ArtifactListNav` | Flat or categorized list navigator for Agents, Rules, Skills, Hooks categories. Search filter input. | ScrollArea, Input | 1 |
| `SessionDropdown` | Session switcher in Chat Panel header. Shows recent sessions, search filter, "New Session" button. | DropdownMenu, Input, Button | 1 |
| `ProjectDashboard` | Project overview: metadata, detected stack, governance counts, quick links. Shown when Project Dashboard is active in Activity Bar. | Card, Badge | 1 |
| `ProjectSwitcher` | Dropdown for switching between recent projects. | Popover, Command | 1 |
| `StatusBar` | Bottom bar: connection status, sidecar state, Claude Code version. | Badge | 1 |

### Settings Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `SettingsView` | Settings panel with collapsible sections. | Collapsible | 1 |
| `ProviderSettings` | Claude Code CLI path, sidecar status, connection health. | Input, Badge, Button | 1 |
| `ProjectSettings` | Project root, detected stack, file watcher status, rescan button. | Input, Badge, Button | 1 |
| `AppearanceSettings` | Theme toggle (light/dark/system), per-project theming toggle. | Select, Checkbox | 1 |
| `ThemeToggle` | Light/dark/system mode switch button. | `mode-watcher`, Button | 1 |
| `ShortcutsReference` | Keyboard shortcuts reference card. | Table | 1 |

### Layout Components

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `AppLayout` | Root layout: toolbar + three-zone body (Activity Bar + Nav Sub-Panel + PaneForge) + status bar. Manages zone collapse state. | Resizable | 1 |
| `Toolbar` | Top bar: project name, search, new session. | Button, Command | 1 |
| `WelcomeScreen` | First-run / empty project state. Forge branding, setup guidance. | Card, Button | 1 |

### Dashboard Components (Later Phases)

| Component | Description | Builds On | Phase |
|-----------|-------------|-----------|-------|
| `ScannerDashboard` | Scanner results list, trend chart, violation details. | Table, Card | 3 |
| `MetricsDashboard` | KPI cards, time-series charts, lesson log. | Card, LayerChart | 5 |
| `KPICard` | Single KPI: value, trend arrow, sparkline. | Card, LayerChart | 5 |
| `LessonCard` | IMPL/RETRO entry with promotion workflow. | Card, Badge, Button | 5 |

---

## Custom Markdown Rendering Blocks

Extensions to the markdown renderer for Forge-specific content types.

| Block | Syntax | Rendering | Phase |
|-------|--------|-----------|-------|
| **Wireframe image** | `![wireframe](forge://wireframe/core-layout?theme=dark)` | Serves cached wireframe image from local store. Selects light/dark/brand variant based on active theme. | 1.5 |
| **UX flow navigation** | `<!-- forge:ux-flow screens="screen1,screen2,screen3" -->` | Renders clickable wireframe sequence with forward/back navigation. | 1.5 |
| **Artifact link** | `[agent:backend-engineer]` | Renders as a styled badge/chip linking to the artifact in the viewer. | 1 |
| **Tool call reference** | `[tool:Read src/main.rs]` | Renders as a tool call summary card (collapsed). | 1 |

---

## Third-Party Libraries

| Library | Purpose | Component Usage |
|---------|---------|----------------|
| `@humanspeak/svelte-markdown` v0.8.13 | Markdown rendering | MarkdownRenderer |
| `svelte-codemirror-editor` v2.1.0 | Code/markdown editing (Svelte 5 runes) | MarkdownEditor, ArtifactEditor |
| `svelte-highlight` | Syntax highlighting in rendered markdown | CodeBlock |
| `mode-watcher` | Dark/light/system mode management | ThemeToggle |
| `lucide-svelte` | Icon library | All components with icons |
| `paneforge` v1.0.2 | Resizable panel layout | AppLayout |

---

## Component Count Summary

| Category | Phase 1 | Phase 2+ | Total |
|----------|---------|----------|-------|
| shadcn-svelte (library) | 21 | 0 | 21 |
| Custom application | 34 | 5 | 39 |
| Custom markdown blocks | 2 | 2 | 4 |
| **Total** | **57** | **7** | **64** |
