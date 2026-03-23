---
id: "DOC-52bbfba5"
title: "Svelte Component Tree"
description: "Component tree structure and hierarchy of the Svelte 5 frontend application."
created: "2026-03-02"
updated: "2026-03-10"
category: "architecture"
tags:
  - "frontend"
  - "svelte"
  - "components"
  - "stores"
  - "architecture"
sort: 8
relationships:
  - target: "AD-8d552e96"
    type: "documents"
    rationale: "Documentation page references AD-8d552e96"
  - target: "AD-61087142"
    type: "documents"
    rationale: "Documentation page references AD-61087142"
  - target: "RULE-cb65b5d0"
    type: "documents"
    rationale: "Documentation page references RULE-cb65b5d0"
  - target: "AD-afc78f6e"
    type: "documents"
    rationale: "Documentation page references AD-afc78f6e"
  - target: "AD-fcd55d44"
    type: "documents"
    rationale: "Documentation page references AD-fcd55d44"
---
**References:** [Frontend Research](RES-df5560cb), Information Architecture

Describes the Svelte component directory structure, store architecture, IPC wrapper pattern, and data flow for the OrqaStudio frontend.

---

## Directory Structure

```
ui/
├── app.html                          # Tauri entry HTML
├── app.css                           # Global styles (Tailwind directives, CSS variables)
├── app.d.ts                          # Global TypeScript declarations
├── routes/
│   ├── +layout.svelte                # Root layout — ThemeProvider, global keybindings
│   ├── +layout.ts                    # SvelteKit layout module
│   └── +page.svelte                  # Single page — AppLayout container
├── lib/
│   ├── components/
│   │   ├── ui/                       # shadcn-svelte primitives (auto-generated)
│   │   ├── layout/                   # Application shell components
│   │   ├── conversation/             # Conversation domain
│   │   ├── artifact/                 # Artifact viewer domain
│   │   ├── navigation/               # Navigation sub-panels
│   │   ├── dashboard/                # Project dashboard
│   │   ├── settings/                 # Settings screens
│   │   ├── governance/               # Governance scan and analysis
│   │   ├── enforcement/              # Enforcement rule display
│   │   ├── lessons/                  # Lessons management
│   │   ├── setup/                    # First-run setup wizard
│   │   ├── tool/                     # Tool call display
│   │   ├── content/                  # Content rendering (markdown, code)
│   │   └── shared/                   # Shared reusable components
│   ├── stores/                       # Svelte 5 $state stores (.svelte.ts)
│   ├── ipc/                          # Tauri invoke() wrapper
│   ├── sdk/                          # Client-side SDK integrations
│   ├── types/                        # TypeScript type definitions
│   └── utils/                        # Pure utility functions
└── static/
    └── favicon.png
```

---

## Component Directories

### `layout/` — Application Shell

| Component | Purpose |
|-----------|---------|
| `AppLayout.svelte` | Root shell — composes all zones |
| `ActivityBar.svelte` | Icon rail — controls which view is active |
| `ActivityBarItem.svelte` | Single activity bar icon with active indicator |
| `NavSubPanel.svelte` | Collapsible left navigation panel |
| `Toolbar.svelte` | Top bar — project name, new session button |
| `StatusBar.svelte` | Bottom bar — connection status, version |
| `WelcomeScreen.svelte` | Shown when no active session |
| `MenuBar.svelte` | Native menu bar |
| `WindowControls.svelte` | Custom window controls (close/minimise/maximise) |
| `SettingsDialog.svelte` | Modal settings dialog |
| `NewProjectDialog.svelte` | New project creation dialog |
| `InitConfirmDialog.svelte` | Confirm project initialisation |
| `AboutDialog.svelte` | About screen |

### `conversation/` — Conversation Domain

| Component | Purpose |
|-----------|---------|
| `ConversationView.svelte` | Scrollable message list + input area |
| `SessionHeader.svelte` | Session title, model selector, token badge |
| `SessionDropdown.svelte` | Session switcher dropdown |
| `ModelSelector.svelte` | Model selection control |
| `MessageBubble.svelte` | Base bubble container for any message |
| `UserMessage.svelte` | User message with bubble |
| `AssistantMessage.svelte` | Assistant message — text + tool calls |
| `SystemMessage.svelte` | System/context message display |
| `MessageInput.svelte` | Text input area and send button |
| `StreamingIndicator.svelte` | Animated indicator during streaming |
| `ContextEntry.svelte` | Single entry in the context sidebar |
| `ContextDetailDialog.svelte` | Full-detail view of a context entry |

### `artifact/` — Artifact Viewer Domain

| Component | Purpose |
|-----------|---------|
| `ArtifactLanding.svelte` | Category landing — shows artifact list for a type |
| `ArtifactMasterDetail.svelte` | Two-pane list + detail layout |
| `ArtifactViewer.svelte` | Generic artifact viewer (renders frontmatter + body) |
| `AgentViewer.svelte` | Specialised viewer for agent definitions |
| `HookViewer.svelte` | Specialised viewer for hooks |
| `SkillViewer.svelte` | Specialised viewer for skills |
| `RuleViewer.svelte` | Specialised viewer for rules |
| `FrontmatterHeader.svelte` | Renders YAML frontmatter as structured header |
| `GateQuestions.svelte` | Renders pillar gate questions |
| `ArtifactLink.svelte` | Inline link to another artifact |
| `Breadcrumb.svelte` | Navigation breadcrumb trail |

### `navigation/` — Navigation Sub-Panels

| Component | Purpose |
|-----------|---------|
| `ArtifactNav.svelte` | List navigation for artifact categories (agents, rules, etc.) |
| `GroupSubPanel.svelte` | Navigation for grouped artifact entries |
| `SettingsCategoryNav.svelte` | Settings category navigation |

### `dashboard/` — Project Dashboard

| Component | Purpose |
|-----------|---------|
| `ProjectDashboard.svelte` | Project overview — metadata, governance counts, stack |

### `settings/` — Settings Screens

| Component | Purpose |
|-----------|---------|
| `SettingsView.svelte` | Settings root — routes to sub-sections |
| `ProjectGeneralSettings.svelte` | Project name, description, icon |
| `ProjectGovernanceSettings.svelte` | Governance artifact management |
| `ProjectScanningSettings.svelte` | Codebase scan configuration |
| `ProjectSetupWizard.svelte` | First-run project configuration wizard |
| `AppearanceSettings.svelte` | Theme and display settings |
| `ModelSettings.svelte` | AI model configuration |
| `ProviderSettings.svelte` | AI provider configuration |
| `ShortcutsSettings.svelte` | Keyboard shortcut reference |
| `SidecarStatusCard.svelte` | Sidecar process status display |
| `CliStatusCard.svelte` | Claude CLI installation status |
| `CliSubscriptionInfo.svelte` | Claude subscription plan info |

### `governance/` — Governance Scan and Analysis

| Component | Purpose |
|-----------|---------|
| `GovernanceScanPanel.svelte` | Scan controls and results overview |
| `GovernanceBootstrapWizard.svelte` | Wizard for first-time governance setup |
| `RecommendationList.svelte` | List of AI-generated governance recommendations |
| `RecommendationCard.svelte` | Single recommendation with status controls |
| `CoverageIndicator.svelte` | Visual indicator of governance area coverage |

### `enforcement/` — Enforcement Display

| Component | Purpose |
|-----------|---------|
| `ViolationBadge.svelte` | Badge showing block/warn violation count |

### `lessons/` — Lessons Management

| Component | Purpose |
|-----------|---------|
| `LessonsPanel.svelte` | Lessons panel container |
| `LessonList.svelte` | Filterable list of lesson entries |
| `LessonViewer.svelte` | Detail view for a single lesson |

### `setup/` — First-Run Setup Wizard

| Component | Purpose |
|-----------|---------|
| `SetupWizard.svelte` | Setup wizard shell — routes through steps |
| `ClaudeCliStep.svelte` | Check and install Claude CLI step |
| `ClaudeAuthStep.svelte` | Claude authentication step |
| `SidecarStep.svelte` | Sidecar process setup step |
| `EmbeddingModelStep.svelte` | Embedding model download step |
| `SetupComplete.svelte` | Setup complete confirmation |

### `tool/` — Tool Call Display

| Component | Purpose |
|-----------|---------|
| `ToolCallCard.svelte` | Individual tool call with input/output |
| `ToolCallGroup.svelte` | Grouped tool calls within a message |
| `ToolCallSummary.svelte` | Collapsed summary of tool call results |
| `ToolApprovalDialog.svelte` | Approval prompt for tool calls requiring user consent |

### `content/` — Content Rendering

| Component | Purpose |
|-----------|---------|
| `MarkdownRenderer.svelte` | Renders markdown with custom element support |
| `MarkdownLink.svelte` | Artifact-aware link renderer within markdown |
| `CodeBlock.svelte` | Syntax-highlighted code with copy button |

### `shared/` — Shared Reusable Components

| Component | Purpose |
|-----------|---------|
| `EmptyState.svelte` | Empty list/grid placeholder with icon, title, optional action |
| `ErrorDisplay.svelte` | Error message with optional retry |
| `LoadingSpinner.svelte` | Loading indicator for async operations |
| `ConfirmDeleteDialog.svelte` | Destructive action confirmation dialog |
| `ArtifactListItem.svelte` | Standard list row for artifact entries |
| `StatusIndicator.svelte` | Coloured dot status indicator |
| `ThinkingBlock.svelte` | Expandable thinking/reasoning block |
| `MetadataRow.svelte` | Key-value metadata display row |
| `SearchInput.svelte` | Debounced search input field |
| `SelectMenu.svelte` | Styled select menu control |
| `SmallBadge.svelte` | Compact badge for labels and counts |

---

## Store Architecture

All stores use Svelte 5 runes exclusively [AD-8d552e96](AD-8d552e96). Store files use the `.svelte.ts` extension to enable runes outside of components. Each store is a class instance exported as a singleton.

| Store | File | Purpose |
|-------|------|---------|
| **Session** | `session.svelte.ts` | Active session, session list, session metadata |
| **Navigation** | `navigation.svelte.ts` | Active activity, explorer view, selected artifact path, config-driven nav tree |
| **Conversation** | `conversation.svelte.ts` | Message list, streaming state, streaming content accumulation, context entries |
| **Artifact** | `artifact.svelte.ts` | Selected artifact, artifact index per category |
| **Project** | `project.svelte.ts` | Active project, project list, project settings, scan results, artifact config |
| **Settings** | `settings.svelte.ts` | App-level and project-level settings |
| **Governance** | `governance.svelte.ts` | Governance scan results, AI analysis, recommendations |
| **Enforcement** | `enforcement.svelte.ts` | Enforcement rules, violations, block/warn counts |
| **Lessons** | `lessons.svelte.ts` | Lesson list, promotion candidates |
| **Setup** | `setup.svelte.ts` | First-run setup state, step tracking, CLI and sidecar status |

---

## IPC Wrapper

The `lib/ipc/invoke.ts` module wraps Tauri's `invoke()` with consistent error handling. Stores call this wrapper rather than the raw Tauri API directly. The wrapper parses JSON-encoded `OrqaError` responses from Rust and re-throws them in a form that TypeScript code can discriminate on.

There is no `commands/` directory — invoke calls are made directly from stores using the `ipc/invoke` wrapper.

---

## SDK Integrations

`lib/sdk/artifact-graph.svelte.ts` provides a reactive client for the artifact graph Tauri commands (added in the graph commands implementation).

---

## Component Patterns

### Container vs. Display

Per [AD-61087142](AD-61087142), only containers and the route entry points call `invoke()`. Display components receive data via `$props()`.

| Boundary | Files |
|----------|-------|
| **Data-fetching containers** | `routes/+page.svelte`, `routes/+layout.svelte` |
| **Stores (call invoke)** | All files in `lib/stores/` |
| **Display components (props only)** | All files in `lib/components/` |

Display components communicate user intent upward via callback props (e.g., `onSend`, `onSelect`, `onSave`). The container or store receives these callbacks and performs the `invoke()` call.

### Shared Component Usage

Per [RULE-cb65b5d0](RULE-cb65b5d0), standard states must use shared components:

- Empty states: always `<EmptyState>` from `shared/`
- Loading states: always `<LoadingSpinner>` from `shared/`
- Error states: always `<ErrorDisplay>` from `shared/`
- Destructive actions: always `<ConfirmDeleteDialog>` from `shared/`

### shadcn-svelte Primitives

The `ui/` directory contains auto-generated shadcn-svelte components. These are used directly in custom components via `$lib/components/ui/<name>`. They are not modified manually.

---

## Data Flow

### Request/Response (invoke)

```
User Action → Display Component → callback prop → Store → invoke() → Rust command
                                                                         ↓
Component re-renders ← $state mutation ← Store update ← Result<T, E>
```

### Streaming (Channel<T>)

High-frequency token streaming uses Tauri `Channel<T>` rather than `emit/listen`:

```
Claude API → Sidecar (Bun) → NDJSON stdout → Rust Channel<T> → +page.svelte callback
                                                                       ↓
                                                            conversationStore.$state mutation
                                                                       ↓
                                                          ConversationView re-renders (fine-grained)
```

### Low-Frequency Events (emit/listen)

File-system changes, sidecar lifecycle, and scan-complete notifications use Tauri events emitted from the Rust backend and listened to in `+layout.svelte` or `+page.svelte`.

---

## Conventions Summary

| Convention | Rule | Reference |
|-----------|------|-----------|
| Runes only | `$state`, `$derived`, `$effect`, `$props` everywhere. No `export let`, no `$:`, no `<slot>`. | [AD-8d552e96](AD-8d552e96) |
| Component purity | `$lib/components/` files receive props only. No `invoke()` inside components. | [AD-61087142](AD-61087142) |
| Stores call invoke | Stores are the only layer (besides routes) that call the `ipc/invoke` wrapper. | [AD-61087142](AD-61087142) |
| Callback props | User intent flows up via callback props to stores or containers. | [AD-61087142](AD-61087142) |
| shadcn-svelte | Use `$lib/components/ui/<name>` for primitives. Do not rewrite them. | [AD-afc78f6e](AD-afc78f6e) |
| Store files | `.svelte.ts` extension, class-based with `$state` fields, exported singleton. | [AD-8d552e96](AD-8d552e96) |
| Streaming | `Channel<T>` for token streams; `emit/listen` for app-level events. | [AD-fcd55d44](AD-fcd55d44) |
| Snippets over slots | Use `{#snippet}` and `{@render}` for composition; never `<slot>`. | [AD-8d552e96](AD-8d552e96) |

---

## shadcn-svelte Components (Library)

Components used directly from the shadcn-svelte library. No custom code needed beyond standard usage.

### Core UI

| Component | Usage |
|-----------|-------|
| **Button** | Send message, new session, save, cancel, create artifact |
| **Input** | Search filters, session search, artifact search, settings fields |
| **Textarea** | Message input (multi-line) |
| **Label** | Form labels in settings |
| **Checkbox** | Settings toggles |
| **Select** | Model selector, theme mode (light/dark/system) |
| **Separator** | Visual dividers between sections |

### Layout

| Component | Usage |
|-----------|-------|
| **Resizable** (PaneForge) | Three-zone resizable layout within PaneForge |
| **ScrollArea** | Message stream, artifact list, session list |
| **Collapsible** | Tool call cards (collapsed/expanded), settings sections |

### Data Display

| Component | Usage |
|-----------|-------|
| **Badge** | Tool call status, artifact type, connection status |
| **Card** | Message bubbles, tool call cards, KPI cards, artifact summary cards |
| **Table** | Keyboard shortcuts reference, settings display |
| **Tooltip** | Truncated text hover, icon descriptions, shortcut hints |

### Overlay

| Component | Usage |
|-----------|-------|
| **Dialog** | Confirmation dialogs (delete artifact, discard changes), project creation |
| **Popover** | Project switcher, model info |
| **Sheet** | Mobile/narrow responsive alternative to collapsed panels |
| **Command** | Global search (`Ctrl+K`) with result list |
| **DropdownMenu** | Context menus, session actions, artifact actions |

### Feedback

| Component | Usage |
|-----------|-------|
| **Alert** | Error messages, warnings, first-run guidance |
| **Sonner** (Toast) | Save confirmation, copy-to-clipboard, connection status changes |

---

## Custom Markdown Rendering Blocks

Extensions to the markdown renderer for OrqaStudio-specific content types.

| Block | Syntax | Rendering |
|-------|--------|-----------|
| **Wireframe image** | `![wireframe](orqa://wireframe/core-layout?theme=dark)` | Serves cached wireframe image from local store. Selects light/dark/brand variant based on active theme. |
| **UX flow navigation** | `<!-- orqa:ux-flow screens="screen1,screen2,screen3" -->` | Renders clickable wireframe sequence with forward/back navigation. |
| **Artifact link** | `[agent:backend-engineer]` | Renders as a styled badge/chip linking to the artifact in the viewer. |
| **Tool call reference** | `[tool:Read src/main.rs]` | Renders as a tool call summary card (collapsed). |

---

## Third-Party Libraries

| Library | Purpose | Component Usage |
|---------|---------|----------------|
| `@humanspeak/svelte-markdown` | Markdown rendering | MarkdownRenderer |
| `svelte-codemirror-editor` | Code/markdown editing (Svelte 5 runes) | MarkdownEditor, ArtifactEditor |
| `svelte-highlight` | Syntax highlighting in rendered markdown | CodeBlock |
| `mode-watcher` | Dark/light/system mode management | ThemeToggle |
| `lucide-svelte` | Icon library | All components with icons |
| `paneforge` | Resizable panel layout | AppLayout |

---

## Related Documents

- Information Architecture — Layout model and navigation
- Architecture Decisions — [AD-8d552e96](AD-8d552e96) (runes), [AD-61087142](AD-61087142) (purity), [AD-fcd55d44](AD-fcd55d44) (streaming)
- SQLite Schema — Database tables backing the stores
