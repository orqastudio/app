# App Frontend Inventory (`app/src/`)

Complete file-by-file inventory of the Svelte 5 / SvelteKit / Tauri frontend.

---

## Root Files

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `app.css` | Global CSS — Tailwind base/components/utilities, custom properties for theme colors, scrollbar styling, tippy.js theme | `@tailwind`, CSS custom properties | N/A | None |
| `app.d.ts` | TypeScript ambient declarations — extends `Window` with `__orqa` shared module registry for plugin runtime loading | N/A | N/A | None |

---

## Routes (`routes/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `+layout.ts` | SvelteKit layout config — disables SSR, enables CSR and prerendering for Tauri desktop context | N/A | N/A | None |
| `+layout.svelte` | Root layout — renders `<slot>` children inside a full-viewport div, imports global CSS | `app.css` | N/A | None |
| `+page.svelte` | Root page — renders `AppLayout` | `AppLayout` | N/A | None |

---

## Assets (`lib/assets/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `banner.png` | App banner / logo image | N/A | N/A | None |
| `setup-background.png` | Background image for setup wizard | N/A | N/A | None |

---

## Utilities (`lib/utils.ts`, `lib/utils/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `utils.ts` | Re-exports from `@orqastudio/sdk` (`cn` class merge utility) | `@orqastudio/sdk` | N/A | None |
| `category-colors.ts` | Maps lesson categories to Tailwind color classes | None | N/A | **HARDCODED**: `CATEGORY_COLORS` map (process, technical, team, governance, delivery, general) to specific Tailwind color classes |
| `tool-display.ts` | Display labels and icons for tool calls in conversation | None | N/A | **HARDCODED**: `TOOL_ICONS` map (read_file, write_file, search, etc.), `TOOL_LABELS` map, `CAPABILITY_LABELS` map (read, write, execute, web_search, mcp_tools) |
| `artifact-view.ts` | Exports `ActivityView` type and `isActivityView()` guard; maps navigation activity keys to artifact categories | `@orqastudio/types` | N/A | None |
| `dev-console.ts` | Overrides `console.log/warn/error` in production to suppress noisy output; injects version from Tauri plugin | `@tauri-apps/api/core` | N/A | None |
| `frontmatter.ts` | Parses YAML frontmatter from markdown strings, returns `{ frontmatter, body }` | `yaml` | N/A | None |

---

## Plugins (`lib/plugins/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `loader.ts` | Runtime plugin loader — gets plugin install path via `plugin_get_path` IPC, resolves `index.js` entry, loads via dynamic import using `convertFileSrc`, calls `plugin.activate()` with context object | `@tauri-apps/api/core` (invoke, convertFileSrc), `shared-modules.ts` | N/A | None |
| `shared-modules.ts` | Registers shared modules on `window.__orqa.sharedModules` so plugins can import framework deps without bundling them. Exposes: svelte, svelte/reactivity, @orqastudio/sdk, @orqastudio/types, @orqastudio/svelte-components/* | `svelte`, `svelte/reactivity`, `@orqastudio/sdk`, `@orqastudio/types`, `@orqastudio/svelte-components/*` | N/A | None |

---

## Services (`lib/services/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `graph-layout.svelte.ts` | Graph layout service — manages Web Worker for cose-bilkent layout computation. Sends graph elements to worker, receives computed positions. Uses Svelte 5 `$state` for reactivity. Singleton via `graphLayoutService` export | `graph-layout.worker.ts` (as Worker URL) | N/A | None |

---

## Graph Visualization (`lib/graph-viz.svelte.ts`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `graph-viz.svelte.ts` | `GraphVisualiser` singleton — manages cytoscape instance lifecycle. Handles node/edge creation from artifact graph data, styling (color by type, shape by category), event handlers (node click navigation, tooltips). `getGraphViz()` accessor | `cytoscape`, `@orqastudio/sdk`, `@orqastudio/types` | N/A | None |

---

## Workers (`lib/workers/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `graph-layout.worker.ts` | Web Worker — receives cytoscape elements, runs cose-bilkent layout algorithm, posts computed node positions back to main thread | `cytoscape`, `cytoscape-cose-bilkent` | N/A | None |

---

## Components: Artifact (`lib/components/artifact/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `AcceptanceCriteria.svelte` | Renders acceptance criteria list from artifact frontmatter. Each criterion is a checkbox with label. Toggleable via `artifactGraphSDK.updateField` | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `criteria: {text,met}[]`, `artifactPath: string`, `readonly: boolean` | None |
| `AgentViewer.svelte` | Specialized viewer for agent-type artifacts. Shows role, capabilities, stage context, knowledge refs in a structured card layout | `@orqastudio/svelte-components/pure`, `ArtifactLink` | `frontmatter: Record<string,unknown>` | None |
| `ArtifactLanding.svelte` | Category landing page — grid of artifact type cards with icon, label, description, count, and "view" button for each type within a category | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `category: string` | **HARDCODED**: `categoryConfig` record mapping category keys to `{icon, label, description}` (process, delivery, discovery, governance, principles) |
| `ArtifactLink.svelte` | Inline artifact reference chip — colored pill showing artifact ID or resolved title with click-to-navigate. Resolves display mode and color from project settings | `@orqastudio/sdk`, `@orqastudio/types` (DEFAULT_ARTIFACT_LINK_COLORS) | `id: string` | **HARDCODED**: `DEFAULT_ARTIFACT_LINK_COLORS` fallback map (imported from types but provides per-prefix color defaults) |
| `ArtifactMasterDetail.svelte` | Two-pane layout — artifact list (ArtifactNav) on left, detail view (ArtifactViewer) on right with resizable splitter | `ArtifactNav`, `ArtifactViewer`, `@orqastudio/svelte-components/pure` | `category: ActivityView` | None |
| `ArtifactViewer.svelte` | Main artifact detail view — loads markdown content, renders FrontmatterHeader + PipelineStepper + ReferencesPanel + TraceabilityPanel + AcceptanceCriteria + GateQuestions + specialized viewers (AgentViewer, RuleViewer, SkillViewer, HookViewer) + MarkdownRenderer. Handles artifact content loading via IPC | `@orqastudio/sdk`, many child components | `artifactPath: string` | **HARDCODED**: Fallback `stages` array `[{key:"draft",label:"Draft"}, ...]` when project settings have no statuses |
| `Breadcrumb.svelte` | Breadcrumb trail for artifact navigation. Shows clickable path segments derived from artifact path | `@orqastudio/svelte-components/pure` (Icon) | `segments: {label,key?,path?}[]`, `onNavigate` | None |
| `FrontmatterHeader.svelte` | Renders artifact frontmatter as structured header — priority badge, status badge, title, metadata fields (chips for arrays, links for artifact refs, booleans as toggles), inline editing for select fields | `@orqastudio/sdk`, `ArtifactLink`, `@orqastudio/svelte-components/pure` | `frontmatter: Record<string,unknown>`, `artifactPath: string`, `showTitle: boolean` | **HARDCODED**: `SKIP_FIELDS` set, `CHIP_FIELDS` set, `LINK_FIELDS` set, `BOOLEAN_FIELDS` set, `FIELD_ORDER` array, priority classes map (P0-P3 to color classes) |
| `GateQuestions.svelte` | Renders gate questions checklist from artifact frontmatter. Similar to AcceptanceCriteria but for quality gates | `@orqastudio/sdk`, `@orqastudio/svelte-components/pure` | `questions: {text,met}[]`, `artifactPath: string`, `readonly: boolean` | None |
| `HookViewer.svelte` | Specialized viewer for hook-type artifacts. Shows trigger event, hook type, conditions, and configuration | `@orqastudio/svelte-components/pure` | `frontmatter: Record<string,unknown>` | None |
| `PipelineStepper.svelte` | Visual pipeline stepper — row of circles connected by lines showing artifact progress through status stages. Clickable circles for allowed transitions (driven by project config `transitions` array). Labels below | `@orqastudio/sdk` (getStores) | `stages: {key,label}[]`, `status: string`, `path: string` | None — transitions driven by project config |
| `ReferencesPanel.svelte` | Collapsible panel showing incoming/outgoing artifact relationships. Groups by relationship type. Toggle between list view and RelationshipGraphView. Overflow handling with "show more" | `@orqastudio/sdk`, `ArtifactLink`, `RelationshipGraphView`, `@orqastudio/svelte-components/pure` | `artifactPath: string` | None |
| `RelationshipGraphView.svelte` | Mini cytoscape graph showing an artifact's direct relationships as a radial layout. Center node = current artifact, connected nodes colored by type | `cytoscape`, `@orqastudio/sdk`, `@orqastudio/types` | `artifactId: string`, `incomingRefs: ArtifactRef[]`, `outgoingRefs: ArtifactRef[]` | None |
| `RelationshipsList.svelte` | Lists artifact relationships in a flat, grouped display. Used inside artifact detail views | `ArtifactLink`, `@orqastudio/sdk` | `refs: ArtifactRef[]`, `direction: "incoming" \| "outgoing"` | None |
| `RuleViewer.svelte` | Specialized viewer for rule-type artifacts. Shows rule trigger, action (block/warn), scope, conditions | `@orqastudio/svelte-components/pure`, `ViolationBadge` | `frontmatter: Record<string,unknown>` | None |
| `SkillViewer.svelte` | Specialized viewer for skill-type artifacts. Shows trigger pattern, capability requirements, description | `@orqastudio/svelte-components/pure` | `frontmatter: Record<string,unknown>` | None |
| `TraceabilityPanel.svelte` | Shows full traceability chain (upstream deliverables -> current -> downstream) using `artifactGraphSDK.getTraceabilityChain()`. Tree-like display with collapsible levels | `@orqastudio/sdk`, `ArtifactLink`, `@orqastudio/svelte-components/pure` | `artifactId: string` | **HARDCODED**: `iconForType` mapping (epic, task, milestone, etc. to icon names) |

---

## Components: Content (`lib/components/content/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `CodeBlock.svelte` | Syntax-highlighted code block with copy button. Language label in header | `@orqastudio/svelte-components/pure` | `code: string`, `lang: string` | None |
| `DiagramCodeBlock.svelte` | Dispatches diagram rendering — detects mermaid vs plantuml from language tag, renders appropriate diagram component or falls back to CodeBlock | `MermaidDiagram`, `PlantUmlDiagram`, `CodeBlock` | `code: string`, `lang: string` | None |
| `DynamicArtifactTable.svelte` | Renders markdown tables with artifact-aware sorting. Auto-detects sortable columns, applies artifact-specific sort orders for priority and status fields | `ArtifactLink`, `@orqastudio/svelte-components/pure` | `header: string[][]`, `body: string[][]` | **HARDCODED**: `PRIORITY_ORDER` map (P0=0..P3=3), `STATUS_ORDER` map (draft=0..done=5..blocked=6) |
| `MarkdownLink.svelte` | Custom markdown link renderer — detects artifact ID patterns in link text/href and renders as `ArtifactLink` chips instead of plain links | `ArtifactLink` | `href: string`, `text: string` | **HARDCODED**: `ARTIFACT_ID_RE` regex with specific prefixes (EPIC, TASK, MS, RES, DEC, AD, REQ, RISK, etc.) |
| `MarkdownRenderer.svelte` | Full markdown renderer using `@humanspeak/svelte-markdown`. Custom renderers for code blocks (CodeBlock/DiagramCodeBlock), links (MarkdownLink), tables (DynamicArtifactTable) | `@humanspeak/svelte-markdown`, custom renderers | `source: string` | None |
| `MermaidDiagram.svelte` | Renders mermaid diagram from source string. Uses mermaid.js `render()` API. Error fallback to code display | `mermaid` | `code: string` | None |
| `PlantUmlDiagram.svelte` | Renders PlantUML diagram by encoding source and loading from PlantUML server | None (image tag with server URL) | `code: string` | None |

---

## Components: Conversation (`lib/components/conversation/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `AssistantMessage.svelte` | Renders assistant (AI) messages — markdown content with MarkdownRenderer, thinking blocks (collapsible), tool call groups | `MarkdownRenderer`, `ToolCallSummary`, `@orqastudio/svelte-components/pure` | `message: Message` | None |
| `ContextDetailDialog.svelte` | Dialog showing injected context details — structured view with collapsible custom prompt and governance prompt sections, or parsed injected messages. Raw view tab | `@orqastudio/svelte-components/pure` | `entry: ContextEntryType`, `open: boolean` (bindable) | None |
| `ContextEntry.svelte` | Single context injection indicator in conversation — shows system prompt sent or context injected event with character counts. Clickable to open ContextDetailDialog | `ContextDetailDialog`, `@orqastudio/svelte-components/pure` | `entry: ContextEntryType` | None |
| `ConversationView.svelte` | Full conversation panel — SessionHeader + message list (UserMessage/AssistantMessage/SystemMessage/ContextEntry/ToolApprovalDialog/StreamingIndicator) + MessageInput. Manages sessions via `conversationStore`, auto-scrolls, handles send/approve/deny | `@orqastudio/sdk` (getStores), many child components | None (uses stores directly) | None |
| `MessageBubble.svelte` | Styled message wrapper — provides consistent bubble styling for different message roles (user/assistant/system) | `@orqastudio/svelte-components/pure` | `role: string`, `children: Snippet` | None |
| `MessageInput.svelte` | Chat input — auto-resizing textarea with send button, keyboard shortcuts (Enter to send, Shift+Enter for newline), disabled state during streaming | `@orqastudio/svelte-components/pure` | `onSend: (text) => void`, `disabled: boolean` | None |
| `ModelSelector.svelte` | Model picker dropdown using SelectMenu — shows available models from `model-options.ts` | `SelectMenu`, `model-options.ts` | `selected: string`, `onSelect: (model) => void` | Uses `CLAUDE_MODELS` from model-options.ts |
| `SessionDropdown.svelte` | Popover dropdown listing all sessions — search, select, create new, delete with confirmation. Shows session title, status badge, message count, relative time, preview | `@orqastudio/svelte-components/pure` | `sessions: SessionSummary[]`, `activeSessionId`, `onSelect`, `onNewSession`, `onDelete`, `onRetry`, `children` (trigger snippet) | None |
| `SessionHeader.svelte` | Session title bar — shows editable session title, session history dropdown trigger, new session button | `SessionDropdown`, `@orqastudio/svelte-components/pure` | `session: Session`, `sessions: SessionSummary[]`, `onNewSession`, `onUpdateTitle`, `onSelectSession`, `onDeleteSession`, `onRetryLoadSessions` | None |
| `StreamingIndicator.svelte` | Animated dots indicator shown during AI response streaming | `@orqastudio/svelte-components/pure` | None | None |
| `SystemMessage.svelte` | Renders system messages in conversation with distinct styling | `@orqastudio/svelte-components/pure` | `message: Message` | None |
| `UserMessage.svelte` | Renders user messages in conversation with markdown support | `MarkdownRenderer` | `message: Message` | None |
| `model-options.ts` | Exports `CLAUDE_MODELS` array of model options | None | N/A | **HARDCODED**: `CLAUDE_MODELS` array: `[{value:"auto",label:"Auto"}, {value:"claude-opus-4-6",...}, {value:"claude-sonnet-4-6",...}, {value:"claude-haiku-4-5",...}]` |

---

## Components: Dashboard (`lib/components/dashboard/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `DashboardCard.svelte` | Generic reusable card wrapper for dashboard widgets — title, description, action slot, children slot | `@orqastudio/svelte-components/pure` (Card*) | `title?: string`, `description?: string`, `action?: Snippet`, `children?: Snippet`, `class?: string` | None |
| `DecisionQueueWidget.svelte` | Shows pending actions (artifacts with status "review") and unblocked epics (status "ready"). Two tabs: Actions and Epics. Navigate on click | `@orqastudio/sdk` (getStores), `ArtifactLink` | None (uses stores) | **HARDCODED**: `actionLabel` function maps artifact type to label (decision -> "Decide", task -> "Assign", etc.) |
| `GraphHealthWidget.svelte` | Compact graph health summary card — shows connectivity score, error/warning counts from integrity checks, scan/auto-fix buttons | `@orqastudio/svelte-components/pure`, `@orqastudio/types` | `checks: IntegrityCheck[]`, `loading`, `fixing`, `scanned`, `graphHealth`, `onScan`, `onAutoFix` | None |
| `HealthTrendWidget.svelte` | Mini sparkline chart showing error/warning counts over time from health snapshots | `@orqastudio/sdk` (getStores), `@orqastudio/types` (HealthSnapshot) | None (uses stores) | None |
| `ImprovementTrendsWidget.svelte` | Dual-line chart — governance artifact count growth + error/warning trend over time. Uses health snapshots aligned with governance artifact creation dates | `@orqastudio/sdk` (getStores), `@orqastudio/types` (HealthSnapshot) | None (uses stores) | **HARDCODED**: `artifactGraphSDK.byType("rule")`, `.byType("lesson")`, `.byType("decision")` — specific governance artifact types |
| `IntegrityWidget.svelte` | Full integrity scan results table — filterable by severity (Error/Warning/Info) and category. Sortable columns. Auto-scans on graph load. Stores health snapshot after scan. Collapsed "all clear" when no issues | `@orqastudio/sdk` (getStores), `ArtifactLink`, `@orqastudio/svelte-components/pure` | None (uses stores) | None — categories come from `IntegrityCategory` type |
| `LessonVelocityWidget.svelte` | Shows lesson lifecycle velocity — counts per stage (identified, active, promoted, resolved) with colored dots and labels | `@orqastudio/sdk` (getStores) | None (uses stores) | **HARDCODED**: `stages` array with fixed stage keys, labels, and Tailwind color classes (identified=amber, active=blue, promoted=green, resolved=slate) |
| `MilestoneContextCard.svelte` | Shows active milestone with P1 epic progress bar — derives active milestone from graph, finds P1 epics linked via `delivers` relationship, shows completion percentage | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `PipelineWidget.svelte` | Visual delivery pipeline — derives pipeline stages from registered relationship types, counts artifacts per stage, shows flow edges between stages using `PipelineStages` component | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` (PipelineStages) | None (uses stores) | None — stages derived from relationship registry |
| `ProjectDashboard.svelte` | Dashboard container — project header with icon, name, description. Scrollable grid of widgets: MilestoneContextCard, IntegrityWidget, PipelineWidget, ImprovementTrendsWidget, GraphHealthWidget, LessonVelocityWidget, DecisionQueueWidget, ToolStatusWidget. Manages integrity scan state | `@orqastudio/sdk` (getStores), all widget components | None (uses stores) | None |
| `ToolStatusWidget.svelte` | Lists CLI tools from plugins with run status, last result summary, duration, and run buttons. Loads via `cli_tool_status` IPC, runs via `run_cli_tool` IPC | `@orqastudio/sdk` (invoke), `@orqastudio/svelte-components/pure` | None (uses IPC) | None |

---

## Components: Enforcement (`lib/components/enforcement/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `ViolationBadge.svelte` | Small badge showing "Block" or "Warn" with tooltip showing rule name | `@orqastudio/svelte-components/pure` (Icon, Tooltip) | `action: "Block" \| "Warn"`, `ruleName: string` | None |

---

## Components: Governance (`lib/components/governance/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `ViolationsPanel.svelte` | Full violation history panel — rule name search, action filter (all/block/warn), scrollable violation list with timestamps, rule names, violation details | `@orqastudio/svelte-components/pure` | `violations: StoredEnforcementViolation[]`, `loading`, `error`, `onRetry` | None |

---

## Components: Graph (`lib/components/graph/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `FullGraphView.svelte` | Main full-page cytoscape graph view — GraphVisualiser + graphLayoutService for Web Worker layout. Health panel sidebar (togglable). Navigates to artifacts on node click. Uses `preset` layout with worker-computed positions | `cytoscape`, `@orqastudio/sdk`, `graph-viz.svelte.ts`, `graph-layout.svelte.ts`, `GraphHealthPanel` | None (uses stores/singletons) | None |
| `GraphHealthPanel.svelte` | Sidebar showing graph health metrics — clusters, connectivity, orphan %, avg degree, broken refs, density, bidirectionality, traceability. Tooltips explain each metric. Severity coloring. Delta comparisons vs previous snapshot | `@orqastudio/svelte-components/pure` | `health: GraphHealthData \| null`, `snapshots: HealthSnapshot[]`, `loading: boolean`, `onRefresh` | **HARDCODED**: Threshold-based severity coloring (e.g., orphan >15% = destructive, >5% = warning; connectivity <50% = destructive, <80% = warning; etc.) |

---

## Components: Layout (`lib/components/layout/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `AboutDialog.svelte` | Simple about dialog — logo, app name "OrqaStudio", version from Tauri plugin | `@orqastudio/svelte-components/pure` | `open: boolean` (bindable) | None |
| `ActivityBar.svelte` | 48px left sidebar with navigation items. Two modes: new navigation tree model (from `navigationStore.topLevelNavItems`) and legacy mode (from `projectStore.artifactConfig`). Bottom section: artifact graph, search, settings | `@orqastudio/sdk` (getStores), `@orqastudio/types`, `ActivityBarItem` | None (uses stores) | None |
| `ActivityBarItem.svelte` | Icon button (40x40) with tooltip for activity bar | `@orqastudio/svelte-components/pure` (Icon, Tooltip) | `icon: string`, `label: string`, `active: boolean`, `onclick` | None |
| `AppLayout.svelte` | Main app shell — initializes stores, settings, dev console, setup wizard. Listens for `artifact-changed` events. Layout: Toolbar + (SetupWizard \| ProjectSetupWizard \| ActivityBar + NavSubPanel + ResizablePaneGroup + ConversationView) + StatusBar + ArtifactSearchOverlay + ErrorToast | `@orqastudio/sdk` (getStores), many child components | None (root component) | None |
| `ExplorerRouter.svelte` | Routes to core views (ProjectDashboard, FullGraphView, WelcomeScreen), plugin views (PluginViewContainer), artifact detail views, or placeholder | `@orqastudio/sdk` (getStores), core view components, `PluginViewContainer` | None (uses stores) | **HARDCODED**: `CORE_VIEWS` record mapping view keys to components |
| `InitConfirmDialog.svelte` | Confirmation dialog for initializing a non-Orqa folder as a project | `@orqastudio/svelte-components/pure` | `open: boolean` (bindable), `folderPath: string`, `onConfirm` | None |
| `MenuBar.svelte` | File/Edit/Help dropdown menus — File (New/Open/Close/Exit), Edit (Settings), Help (About). Hover-follow behavior between menus | `@orqastudio/svelte-components/pure`, Tauri dialog/window APIs | `onNewProject`, `onAbout`, `onSettings` | None |
| `NavSubPanel.svelte` | 200px level-2 navigation panel — routes to SettingsCategoryNav, GroupSubPanel, or ArtifactNav based on active activity/group | `@orqastudio/sdk` (getStores), `SettingsCategoryNav`, `GroupSubPanel`, `ArtifactNav` | None (uses stores) | None |
| `NewProjectDialog.svelte` | Project creation dialog — two options: Create From Scratch or Initialize Existing Folder | `@orqastudio/svelte-components/pure`, Tauri dialog API | `open: boolean` (bindable), `onSelectFolder`, `onCreateNew` | None |
| `SettingsDialog.svelte` | 85vh/90vw modal — SettingsCategoryNav sidebar (app mode) + SettingsView content area | `@orqastudio/sdk` (getStores), `SettingsCategoryNav`, `SettingsView` | `open: boolean` (bindable) | None |
| `StatusBar.svelte` | Bottom bar — OrqaStudio brand, model name (clickable), startup task indicator, token counts (in/out), artifact count (clickable refresh), sidecar status dot, daemon status dot | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | **HARDCODED**: `sidecarPluginName = "@orqastudio/plugin-claude"` |
| `Toolbar.svelte` | Title bar — app icon (project icon or logo), MenuBar, WindowControls. Handles window dragging, double-click maximize. Hosts AboutDialog, SettingsDialog, NewProjectDialog, InitConfirmDialog | `@orqastudio/sdk` (getStores), Tauri window API, child dialog components | None (uses stores) | None |
| `WelcomeScreen.svelte` | Landing page with "Open Project" button using Tauri file dialog | `@orqastudio/svelte-components/pure`, Tauri dialog API | None | None |
| `WindowControls.svelte` | Minimize/Maximize/Close window buttons using Tauri window API | `@tauri-apps/api/window`, `@orqastudio/svelte-components/pure` (Icon) | None | None |

---

## Components: Lessons (`lib/components/lessons/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `LessonList.svelte` | Scrollable lesson list grouped by status (active/promoted/resolved) with category color badges, recurrence counts, promotion candidate markers | `@orqastudio/svelte-components/pure`, `category-colors.ts` | `lessons: Lesson[]`, `loading`, `error`, `selectedId`, `onSelect`, `onRetry` | Uses `categoryColor()` from utils (which has hardcoded color map) |
| `LessonViewer.svelte` | Lesson detail view — recurrence counter, promotion candidate banner, metadata row, markdown body via MarkdownRenderer | `MarkdownRenderer`, `category-colors.ts`, `@orqastudio/svelte-components/pure` | `lesson: Lesson`, `onIncrementRecurrence` | Uses `categoryColor()` from utils |
| `LessonsPanel.svelte` | Master-detail layout — 240px LessonList sidebar + LessonViewer. Uses `lessonStore` and `projectStore` from SDK | `@orqastudio/sdk` (getStores), `LessonList`, `LessonViewer` | None (uses stores) | None |

---

## Components: Navigation (`lib/components/navigation/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `ArtifactNav.svelte` | Artifact list panel — flat list and tree modes. Sort/filter/group via ArtifactToolbar. Collapsible tree sections, breadcrumbs, README filtering. Per-category view state stored in SvelteMap | `@orqastudio/sdk` (getStores), `ArtifactToolbar`, `ArtifactListItem` (connected), `@orqastudio/svelte-components/pure` | `category: ActivityView` | None |
| `ArtifactSearchOverlay.svelte` | Full-screen search overlay (Ctrl+Space) — searches artifact graph by ID, title, description. Keyboard navigation (up/down/enter/escape). Status icons, project badges, type labels. Max 50 results | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `ArtifactToolbar.svelte` | Sort dropdown (radio group) + filter popover (checkboxes per field) + group-by selector for artifact lists | `@orqastudio/svelte-components/pure` | `sortableFields`, `filterableFields`, `navigationConfig`, `nodes`, `currentSort`, `currentFilters`, `currentGroup`, change handlers | None |
| `GroupSubPanel.svelte` | Sub-category list within a navigation group — resolves icons from config then navTree | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `group: string` | None |
| `SettingsCategoryNav.svelte` | Settings sidebar navigation — separate categories for app mode and project mode | `@orqastudio/svelte-components/pure` | `mode: "app" \| "project"`, `activeSection?: string`, `onSectionChange` | **HARDCODED**: `appCategories` array `[{key:"provider",...}, {key:"model",...}, {key:"appearance",...}, {key:"shortcuts",...}]` and `projectCategories` array `[{key:"general",...}, ..., {key:"plugins",...}]` |

---

## Components: Plugin (`lib/components/plugin/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `PluginViewContainer.svelte` | Runtime plugin view loader — gets plugin install path via `plugin_get_path` IPC, loads bundled JS via `convertFileSrc`, mounts via `module.mount()` or Svelte 5 `mount()`. Cleans up on destroy | `@tauri-apps/api/core` (invoke, convertFileSrc), `shared-modules.ts` | `pluginName: string`, `viewKey: string` | None |

---

## Components: Settings (`lib/components/settings/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `AppearanceSettings.svelte` | Theme mode selector — system/light/dark using SelectMenu | `@orqastudio/svelte-components/pure` (SelectMenu), `@orqastudio/sdk` (getStores) | None (uses stores) | None |
| `CliStatusCard.svelte` | CLI installed/version/path/auth status display — re-check and re-authenticate buttons. Renders CliSubscriptionInfo when authenticated | `@orqastudio/sdk` (getStores), `CliSubscriptionInfo`, `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `CliSubscriptionInfo.svelte` | Subscription type badge, rate limit tier, token expiry countdown, scopes list | `@orqastudio/svelte-components/pure` | `cliInfo: CliInfo` | None |
| `ConflictResolutionDialog.svelte` | AI-powered plugin conflict resolution — fetches rename suggestions via sidecar one-shot message, shows strategies (rename-new/rename-existing/rename-both), custom alias input | `@orqastudio/sdk` (getStores, buildConflictResolutionPrompt, parseConflictResolutionResponse), `@orqastudio/svelte-components/pure` | `open: boolean` (bindable), `conflictKey`, `existingPlugin`, `newPlugin`, `onResolve` | None |
| `ModelSettings.svelte` | Default model selector dropdown | `@orqastudio/svelte-components/pure` (SelectMenu), `@orqastudio/sdk` (getStores) | None (uses stores) | **HARDCODED**: `modelOptions` array (auto, claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5) |
| `NavigationSettings.svelte` | Read-only navigation tree viewer — shows items with type (builtin/plugin/group), plugin source, hidden status. Also shows installed plugins with schema/view/relationship counts | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `PluginBrowser.svelte` | Full plugin management — installed/official/community tabs, detail views with manifest breakdown (schemas, relationships, views, widgets, CLI tools, hooks), install from registry or manual (GitHub repo or local path), uninstall, conflict resolution integration | `@orqastudio/sdk` (getStores), `PluginInstallDialog`, `ConflictResolutionDialog`, `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `PluginInstallDialog.svelte` | Plugin install confirmation — shows provides summary, optional navigation items preview, accept/reject/cancel buttons | `@orqastudio/svelte-components/pure` | `open: boolean` (bindable), `manifest`, `onAccept`, `onReject`, `onCancel` | None |
| `ProjectArtifactLinksSettings.svelte` | Artifact link display configuration — per-type display mode toggle (ID vs Title), color picker with reset to default. Uses `DEFAULT_ARTIFACT_LINK_COLORS` from types | `@orqastudio/svelte-components/pure`, `@orqastudio/types` (DEFAULT_ARTIFACT_LINK_COLORS) | `settings: ProjectSettings`, `onSave` | Uses `DEFAULT_ARTIFACT_LINK_COLORS` (imported from types, not hardcoded locally) |
| `ProjectDeliverySettings.svelte` | Delivery type CRUD — each type has key, label, path, parent type selector, parent relationship, gate field. Add/delete with confirmation | `@orqastudio/svelte-components/pure` | `settings: ProjectSettings`, `onSave` | None |
| `ProjectGeneralSettings.svelte` | Project name input, description textarea, icon upload/remove. Auto-saves on blur | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `ProjectScanningSettings.svelte` | Model selector, show-thinking toggle, custom system prompt textarea, excluded paths list (add/remove), detected stack display (languages/frameworks/package manager), re-scan button | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | **HARDCODED**: `modelOptions` array (same as ModelSettings — auto, claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5) |
| `ProjectSetupWizard.svelte` | First-run project setup wizard — project name input, scan button, detected stack/governance display, save configuration | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | **HARDCODED**: Default excluded paths `["node_modules", ".git", "target", "dist", "build"]` |
| `ProjectStatusSettings.svelte` | Status machine editor — full CRUD for status definitions. Each status has key, label, icon, spin toggle, allowed transitions (toggle chips), auto-transition rules (condition -> target). Drag-and-drop reorder. Delete with cascade cleanup | `@orqastudio/svelte-components/pure` | `settings: ProjectSettings`, `onSave` | None — fully data-driven from project settings |
| `ProviderSettings.svelte` | Container for ProviderSwitcher + SidecarStatusCard + CliStatusCard. Auto-checks CLI on mount | `ProviderSwitcher`, `SidecarStatusCard`, `CliStatusCard` | None (uses stores) | None |
| `ProviderSwitcher.svelte` | Lists sidecar providers from pluginRegistry, allows switching active provider. Shows restart-required notice | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |
| `RelationshipSettings.svelte` | Read-only view of PLATFORM_RELATIONSHIPS (from @orqastudio/types) + plugin-contributed relationships. Shows key/inverse, from/to type constraints, source label | `@orqastudio/sdk` (getStores), `@orqastudio/types` (PLATFORM_RELATIONSHIPS) | None (uses stores) | None |
| `SettingsView.svelte` | Router dispatching to setting panels by active section key: ProviderSettings, ModelSettings, AppearanceSettings, ShortcutsSettings, ProjectGeneralSettings, ProjectScanningSettings, NavigationSettings, RelationshipSettings, ProjectArtifactLinksSettings, ProjectDeliverySettings, ProjectStatusSettings, PluginBrowser | All settings panel components | None (uses stores) | None |
| `ShortcutsSettings.svelte` | Read-only keyboard shortcuts reference card | `@orqastudio/svelte-components/pure` | None | **HARDCODED**: `shortcuts` array `[{keys:"Ctrl+Space", action:"Search artifacts"}, ...]` (5 shortcuts) |
| `SidecarStatusCard.svelte` | Sidecar connection status, PID, uptime, CLI version, error display, restart button | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | None (uses stores) | None |

---

## Components: Setup (`lib/components/setup/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `ClaudeAuthStep.svelte` | Setup step — verifies Claude CLI authentication. Auto-checks on mount. Shows authenticated status with subscription type, or instructions to run `claude` for login | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `onComplete: () => void` | None |
| `ClaudeCliStep.svelte` | Setup step — checks for Claude Code CLI installation. Shows version and path when found, or install instructions with docs link | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `onComplete: () => void` | None |
| `EmbeddingModelStep.svelte` | Setup step — checks/downloads embedding model. Polls startup tracker for download progress. Shows "all-MiniLM-L6-v2" when ready | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `onComplete: () => void` | **HARDCODED**: Model name "all-MiniLM-L6-v2" in status display |
| `SetupComplete.svelte` | Setup step — final confirmation. Shows checklist (CLI installed, authenticated, sidecar connected, embedding model ready). "Get Started" button calls `setupStore.completeSetup()` | `@orqastudio/sdk` (getStores), `@orqastudio/svelte-components/pure` | `onComplete: () => void` | **HARDCODED**: Fixed 4-step checklist labels |
| `SetupWizard.svelte` | Setup wizard container — step indicator dots, step counter, routes to ClaudeCliStep -> ClaudeAuthStep -> SidecarStep -> EmbeddingModelStep -> SetupComplete based on `setupStore.stepId`. Background image overlay | `@orqastudio/sdk` (getStores), all step components, `setup-background.png` | `onComplete: () => void` | **HARDCODED**: Step routing by fixed step IDs (claude_cli, claude_auth, sidecar, embedding_model, complete) |
| `SidecarStep.svelte` | Setup step — starts sidecar process. Checks connection, attempts restart if needed. Shows PID when connected | `@orqastudio/sdk` (getStores, extractErrorMessage, logger), `@orqastudio/svelte-components/pure` | `onComplete: () => void` | None |

---

## Components: Tool (`lib/components/tool/`)

| File | Purpose | Key Imports / Dependencies | Props | Hardcoded Governance Patterns |
|------|---------|---------------------------|-------|-------------------------------|
| `ToolApprovalDialog.svelte` | Card prompting user to approve/deny a tool call — shows tool label, formatted JSON input, approve/deny buttons | `@orqastudio/svelte-components/pure`, `tool-display.ts` | `approval: PendingApproval`, `onApprove`, `onDeny` | Uses `TOOL_LABELS`/`TOOL_ICONS` from tool-display.ts |
| `ToolCallCard.svelte` | Individual tool call display — collapsible input/output sections. Detects enforcement blocks (parses "Rule 'name' blocked..." pattern). Shows ViolationBadge for blocked calls. Truncates output at 10K chars with expand toggle | `ViolationBadge`, `@orqastudio/svelte-components/pure`, `tool-display.ts` | `toolName`, `toolInput`, `toolOutput`, `isError`, `isComplete` | **HARDCODED**: Regex pattern to detect enforcement block messages in tool output |
| `ToolCallGroup.svelte` | Collapsible group of tool calls by tool name — shows count badge, error count badge, expands to show individual ToolCallCards | `ToolCallCard`, `@orqastudio/svelte-components/pure`, `tool-display.ts` | `toolName: string`, `toolCalls: ToolCallInfo[]` | None |
| `ToolCallSummary.svelte` | Groups tool_use/tool_result message pairs — shows summary label, expandable detail with per-tool badges. Collapses multiple tool calls into a compact summary | `ToolCallGroup`, `@orqastudio/svelte-components/pure`, `tool-display.ts` | `messages: Message[]` | None |

---

## Summary: Hardcoded Governance Patterns

All instances where governance patterns, artifact types, status values, or workflow logic are hardcoded in the frontend rather than driven by plugins or the engine:

| File | What's Hardcoded | Severity |
|------|-----------------|----------|
| `category-colors.ts` | Lesson category -> Tailwind color map (process, technical, team, governance, delivery, general) | Low — cosmetic |
| `tool-display.ts` | TOOL_ICONS, TOOL_LABELS, CAPABILITY_LABELS maps for tool call display | Low — cosmetic |
| `model-options.ts` | CLAUDE_MODELS array (auto, opus, sonnet, haiku) | Medium — should come from sidecar/plugin |
| `ArtifactLink.svelte` | Fallback to DEFAULT_ARTIFACT_LINK_COLORS (imported from types) | Low — has override mechanism via settings |
| `FrontmatterHeader.svelte` | SKIP_FIELDS, CHIP_FIELDS, LINK_FIELDS, BOOLEAN_FIELDS, FIELD_ORDER, priority color classes | Medium — field classification should come from schema |
| `TraceabilityPanel.svelte` | iconForType mapping (epic->flag, task->check-square, etc.) | Low — cosmetic |
| `ArtifactViewer.svelte` | Fallback pipeline stages array [draft, in_progress, review, done] | Medium — should always come from project config |
| `ArtifactLanding.svelte` | categoryConfig with hardcoded icons/labels/descriptions per category | Medium — should come from navigation config |
| `DynamicArtifactTable.svelte` | PRIORITY_ORDER (P0-P3), STATUS_ORDER (draft..blocked) | Medium — should come from project config |
| `MarkdownLink.svelte` | ARTIFACT_ID_RE with specific prefixes (EPIC, TASK, MS, RES, etc.) | High — artifact type prefixes should come from schema |
| `StatusBar.svelte` | sidecarPluginName = "@orqastudio/plugin-claude" | High — should come from plugin registry |
| `SettingsCategoryNav.svelte` | appCategories and projectCategories arrays | Low — settings structure is core, not plugin |
| `ModelSettings.svelte` | modelOptions array (same as model-options.ts) | Medium — duplicated, should share source |
| `ProjectScanningSettings.svelte` | modelOptions array (duplicated again) | Medium — duplicated, should share source |
| `ShortcutsSettings.svelte` | shortcuts list (5 entries) | Low — shortcuts are core app feature |
| `ProjectSetupWizard.svelte` | Default excluded paths | Low — reasonable defaults |
| `GraphHealthPanel.svelte` | Health metric severity thresholds | Low — could be configurable but reasonable as defaults |
| `LessonVelocityWidget.svelte` | Stage definitions with colors (identified, active, promoted, resolved) | Medium — lesson stages should come from config |
| `DecisionQueueWidget.svelte` | actionLabel per artifact type (decision->Decide, task->Assign) | Low — cosmetic |
| `ImprovementTrendsWidget.svelte` | Hardcoded governance types: rule, lesson, decision | Medium — should come from schema |
| `EmbeddingModelStep.svelte` | Model name "all-MiniLM-L6-v2" | Low — display only |
| `SetupComplete.svelte` | Fixed 4-step checklist labels | Low — setup flow is core |
| `SetupWizard.svelte` | Fixed step IDs for routing | Low — setup flow is core |
| `ToolCallCard.svelte` | Regex for enforcement block detection | Low — pattern matching on known format |
| `ExplorerRouter.svelte` | CORE_VIEWS record mapping view keys to components | Low — core view routing is legitimate |

### High Severity (should be addressed)
1. **`MarkdownLink.svelte`** — Artifact ID regex has hardcoded type prefixes. Should derive from registered schemas.
2. **`StatusBar.svelte`** — Hardcoded sidecar plugin name. Should use `pluginRegistry.activeSidecarKey`.

### Medium Severity (worth refactoring)
3. **`model-options.ts` / `ModelSettings.svelte` / `ProjectScanningSettings.svelte`** — Model list duplicated in 3 places. Should come from sidecar/plugin.
4. **`FrontmatterHeader.svelte`** — Field classification (skip/chip/link/boolean/order) hardcoded. Should derive from artifact schema definitions.
5. **`ArtifactViewer.svelte`** — Fallback status stages. Should require project config.
6. **`ArtifactLanding.svelte`** — Category config. Should come from navigation tree config.
7. **`DynamicArtifactTable.svelte`** — Priority/status sort orders. Should come from project status config.
8. **`LessonVelocityWidget.svelte`** — Lesson stage definitions. Should come from lesson lifecycle config.
9. **`ImprovementTrendsWidget.svelte`** — Governance artifact types. Should come from schema registry.

---

## File Count Summary

| Directory | Count |
|-----------|-------|
| Root (`app.css`, `app.d.ts`) | 2 |
| `routes/` | 3 |
| `lib/assets/` | 2 |
| `lib/utils.ts` + `lib/utils/` | 6 |
| `lib/plugins/` | 2 |
| `lib/services/` | 1 |
| `lib/graph-viz.svelte.ts` | 1 |
| `lib/workers/` | 1 |
| `components/artifact/` | 17 |
| `components/content/` | 7 |
| `components/conversation/` | 13 |
| `components/dashboard/` | 11 |
| `components/enforcement/` | 1 |
| `components/governance/` | 1 |
| `components/graph/` | 2 |
| `components/layout/` | 15 |
| `components/lessons/` | 3 |
| `components/navigation/` | 5 |
| `components/plugin/` | 1 |
| `components/settings/` | 20 |
| `components/setup/` | 6 |
| `components/tool/` | 4 |
| **Total** | **123** |
