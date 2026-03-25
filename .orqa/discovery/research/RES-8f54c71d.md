---
id: RES-8f54c71d
title: Frontend App Audit — Post Plugin-Composed Architecture
created: 2026-03-25
type: research
status: completed
---

# Frontend App Audit — Post Plugin-Composed Architecture

Conducted 2026-03-25 against commit 9171faea on the `main` branch.

---

## 1. Route/Page Inventory

The app uses a single-page architecture. There is only one SvelteKit route:

| Route | File | Purpose |
|-------|------|---------|
| `/` | `app/src/routes/+page.svelte` | Renders `AppLayout` (the entire app) |
| (layout) | `app/src/routes/+layout.svelte` | Initialises SDK stores, plugin registry, hash-based router, key bindings |
| (layout config) | `app/src/routes/+layout.ts` | Disables SSR and prerendering |

All navigation is handled client-side via the SDK's `NavigationStore` with hash-based routing. There are no `/workflows`, `/gates`, `/agents`, or other sub-routes in SvelteKit.

**Verdict**: The routing layer is clean. No deprecated references. The single-route architecture is deliberate for a Tauri desktop app.

---

## 2. Component Inventory

### 2.1 Core View Components (ExplorerRouter)

Only 3 core views are registered in `ExplorerRouter.svelte` (line 23-27):

```typescript
const CORE_VIEWS: Record<string, Component> = {
    "project": ProjectDashboard,
    "artifact-graph": FullGraphView,
    "welcome": WelcomeScreen,
};
```

Everything else is either an `ArtifactViewer` (for selected artifacts) or a `PluginViewContainer` (for plugin-contributed views). This is correct and extensible.

### 2.2 Component Status by Area

#### Dashboard (`app/src/lib/components/dashboard/`) — 10 components

| Component | Status | Notes |
|-----------|--------|-------|
| `ProjectDashboard.svelte` | OK | Orchestrates widgets, delegates to `artifactGraphSDK` |
| `GraphHealthWidget.svelte` | **STALE** | Shows "Bidirectionality Ratio" metric (lines 85-89, 268-279). With forward-only storage, bidirectionality_ratio will always be < 100% by design. The tooltip text at line 275 says "Missing inverses create navigation asymmetry" which is misleading — inverses are now computed, not stored. The threshold alert at line 129 flags bidirectionality < 80% which will fire permanently. |
| `IntegrityWidget.svelte` | **STALE** | `categoryLabels` map (lines 36-49) is missing `SchemaViolation` and `TypePrefixMismatch` categories that the Rust validation crate now emits. Findings with these categories will render with `undefined` labels. |
| `PipelineWidget.svelte` | OK | Uses plugin relationships dynamically from `pluginRegistry.allRelationships`. No hardcoded types. |
| `DecisionQueueWidget.svelte` | OK | Queries graph by status, no deprecated references. |
| `MilestoneContextCard.svelte` | OK | Uses graph SDK, no deprecated references. |
| `ImprovementTrendsWidget.svelte` | OK | Displays health snapshot trends. Clean. |
| `HealthTrendWidget.svelte` | OK | Sparkline trends from snapshots. Clean. |
| `LessonVelocityWidget.svelte` | OK | Lesson pipeline by status. Clean. |
| `ToolStatusWidget.svelte` | OK | CLI tool runner. Clean. |
| `DashboardCard.svelte` | OK | Generic card wrapper. Clean. |

#### Artifact (`app/src/lib/components/artifact/`) — 17 components

| Component | Status | Notes |
|-----------|--------|-------|
| `HookViewer.svelte` | **POTENTIALLY STALE** | Renders `.sh` files as bash code blocks. Still valid for `.claude/hooks/` but the git-hooks system was part of the `githooks` plugin and may not be the primary hook mechanism going forward. |
| `ArtifactViewer.svelte` | **IMPORTS HOOK** | Lines 5, 288: imports and renders `HookViewer` for `.sh` file extensions. This is harmless but couples artifact viewing to hook rendering. |
| `ArtifactLanding.svelte` | **STALE REFERENCES** | Lines 48-55: Has a `hooks` category configuration pointing to `.claude/hooks/` with description "Hooks run automated actions at lifecycle events". The gate engine now handles lifecycle events via YAML workflows, not shell hooks. |
| `GateQuestions.svelte` | OK | Generic gate question renderer. Clean and reusable. |
| `TraceabilityPanel.svelte` | OK (minor) | Line 42: maps `hook` type to `webhook` icon. Harmless. |
| `PipelineStepper.svelte` | OK | Status pipeline visualization. Clean. |
| All others | OK | `AcceptanceCriteria`, `ArtifactLink`, `ArtifactMasterDetail`, `Breadcrumb`, `FrontmatterHeader`, `ReferencesPanel`, `RelationshipGraphView`, `RelationshipsList`, `RuleViewer`, `SkillViewer`, `AgentViewer` are all clean. |

#### Graph (`app/src/lib/components/graph/`) — 2 components

| Component | Status | Notes |
|-----------|--------|-------|
| `FullGraphView.svelte` | OK | Cytoscape-based graph visualization. Clean. |
| `GraphHealthPanel.svelte` | **STALE** | Same bidirectionality issue as `GraphHealthWidget.svelte`. Lines 71-75, 154-158, 363-366 reference bidirectionality_ratio with thresholds that assume stored inverses. |

#### Settings (`app/src/lib/components/settings/`) — 16 components

| Component | Status | Notes |
|-----------|--------|-------|
| `ProjectStatusSettings.svelte` | **CONCEPTUALLY STALE** | Manages `statusTransitions` and `auto_rules` in `project.json`. The new architecture uses YAML workflow state machines in `.orqa/workflows/`. This UI edits the old `project.json` status definitions which still drive the Tauri `status_transitions` engine, but those are being superseded by the workflow engine. Not broken — but represents the OLD way of doing status management. |
| `PluginBrowser.svelte` | **MINOR** | Lines 48, 436-453: Displays plugin hooks from manifests. Still valid for `githooks` plugin. |
| `ProjectSetupWizard.svelte` | **MINOR** | Lines 153-154: Shows hook count in scan results. Still valid. |
| All others | OK | `AppearanceSettings`, `CliStatusCard`, `ModelSettings`, `NavigationSettings`, `PluginInstallDialog`, `RelationshipSettings`, `ProjectGeneralSettings`, `ProjectScanningSettings`, `ProjectDeliverySettings`, `ProjectArtifactLinksSettings`, `SettingsView`, `ShortcutsSettings`, `SidecarStatusCard`, `ConflictResolutionDialog`, `ProviderSettings`, `ProviderSwitcher` are all clean. |

#### Other Areas

| Area | Status |
|------|--------|
| `layout/` (13 components) | OK — clean, no deprecated references |
| `content/` (6 components) | OK — markdown rendering, diagrams |
| `conversation/` (12 components) | OK — chat/streaming interface |
| `navigation/` (5 components) | OK |
| `plugin/` (1 component) | OK — `PluginViewContainer` is well-designed |
| `lessons/` (3 components) | OK |
| `enforcement/` (1 component) | OK — `ViolationBadge` |
| `governance/` (1 component) | OK — `ViolationsPanel` |
| `tool/` (4 components) | OK — tool approval/display |
| `setup/` (5 components) | OK |

---

## 3. Store/State Audit

There is NO `app/src/lib/stores/` directory. All state management lives in `libs/sdk/src/stores/`:

| Store | File | Purpose | Status |
|-------|------|---------|--------|
| `artifact` | `artifact.svelte.ts` | Nav tree, content loading | OK |
| `conversation` | `conversation.svelte.ts` | Chat sessions | OK |
| `enforcement` | `enforcement.svelte.ts` | Rules, violations | OK |
| `errors` | `errors.svelte.ts` | Error collection | OK |
| `lessons` | `lessons.svelte.ts` | Lesson management | OK |
| `navigation` | `navigation.svelte.ts` | View routing, activity bar | OK |
| `project` | `project.svelte.ts` | Active project, settings | OK |
| `session` | `session.svelte.ts` | Chat sessions | OK |
| `settings` | `settings.svelte.ts` | App-level settings | OK |
| `setup` | `setup.svelte.ts` | First-run wizard state | OK |
| `toast` | `toast.svelte.ts` | Toast notifications | OK |

Additionally, the `ArtifactGraphSDK` in `libs/sdk/src/graph/artifact-graph.svelte.ts` acts as the primary graph state manager. It is the central data hub.

**What's Missing from Stores**:
- No `WorkflowStore` — the 17 resolved YAML workflow files have no UI representation
- No `GateStore` — gate reviews have no UI state
- No `AgentTeamStore` — agent team metrics/status have no UI representation
- No `TokenBudgetStore` — token tracking has no UI representation
- No `KnowledgeStore` — knowledge browsing has no dedicated UI state

---

## 4. Backend Integration

### 4.1 Communication Pattern

Frontend communicates with the Rust backend via Tauri IPC (`invoke()` calls). The SDK wraps these in typed methods. The `ArtifactGraphSDK` additionally listens for Tauri events (`artifact-graph-updated`, `status-transitions-available`).

### 4.2 Registered Tauri Commands (from `app/src-tauri/src/lib.rs` lines 193-258)

| Command Group | Commands | Status |
|--------------|----------|--------|
| **Daemon** | `daemon_health` | OK |
| **Sidecar** | `sidecar_status`, `sidecar_restart` | OK |
| **Streaming** | `stream_send_message`, `stream_stop`, `stream_tool_approval_respond` | OK |
| **Project** | `project_open`, `project_get_active`, `project_list` | OK |
| **Sessions** | CRUD operations | OK |
| **Messages** | `message_list` | OK |
| **Artifacts** | `artifact_scan_tree`, `artifact_watch_start` | OK |
| **Project Settings** | CRUD + scan + icon | OK |
| **Settings** | `settings_set`, `settings_get_all` | OK |
| **Search** | `get_startup_status` | OK |
| **Setup** | CLI checks, auth, embedding model | OK |
| **Lessons** | CRUD + recurrence | OK |
| **Enforcement** | Rules list/reload, violations list | OK |
| **Graph** | Full graph lifecycle + integrity | OK |
| **Status Transitions** | `evaluate_status_transitions`, `apply_status_transition` | **DUAL SYSTEM** |
| **CLI Tools** | `get_registered_cli_tools`, `run_cli_tool`, `cli_tool_status` | OK |
| **Plugins** | Full lifecycle (list, install, uninstall, manifest) | OK |
| **Hooks** | `get_registered_hooks`, `generate_hook_dispatchers` | **POTENTIALLY STALE** |

### 4.3 Alignment Issues

1. **Dual status transition system**: `status_transition_commands.rs` (standalone) AND `graph_commands.rs` (lines 151-238) BOTH implement status transition evaluation. The standalone version reads from `project.json` `statusTransitions`. The graph_commands version does the same but also handles auto-apply. Neither uses the new YAML workflow state machines.

2. **Hook commands still registered**: `hook_commands.rs` provides `get_registered_hooks` and `generate_hook_dispatchers`. These are still valid for the `githooks` plugin but are orthogonal to the new gate engine.

3. **No workflow commands**: No Tauri commands exist for reading/executing YAML workflow state machines.

4. **No gate review commands**: No Tauri commands for the five-phase gate pipeline.

5. **No agent team commands**: No Tauri commands for agent team management/metrics.

### 4.4 Tauri Domain Modules (`app/src-tauri/src/domain/`)

Notable modules:
- `status_transitions.rs` — Old `project.json`-based transition engine. Still functional but superseded by YAML workflows.
- `process_gates.rs` — Exists but NOT exposed as Tauri commands to the frontend.
- `process_state.rs` — Session process state. Present in `AppState` but no commands expose it.
- `workflow_tracker.rs` — Present in `AppState` (`session.workflow_tracker`) but no commands expose it.
- `knowledge_injector.rs` — Present in `AppState` (`artifacts.knowledge_injector`) but no commands expose it.

**Key finding**: The Rust backend has `process_gates`, `workflow_tracker`, and `knowledge_injector` domain modules registered in `AppState` but NONE of them are exposed to the frontend via Tauri commands.

---

## 5. Plugin View System

### 5.1 View Declarations in Plugin Manifests

| Plugin | Views Declared |
|--------|---------------|
| `software` | 1: `roadmap` (key: "roadmap", label: "Roadmap", icon: "kanban") |
| `core` | 0 |
| `agile-governance` | 0 |
| `cli` | 0 |
| `coding-standards` | 0 |
| `plugin-dev` | 0 |
| `systems-thinking` | 0 |
| `githooks` | 0 |
| `svelte` | 0 |

### 5.2 View Loading Mechanism

`PluginViewContainer.svelte` (lines 16-56) loads plugin views by:
1. Getting the plugin's install path via `invoke("plugin_get_path", { name })`
2. Loading `{pluginPath}/dist/views/{viewKey}.js` as a dynamic ES module
3. Looking for `module.mount()` or `module.default` (Svelte component)
4. Mounting into a container div

### 5.3 Gap Analysis

- The `software` plugin declares a `roadmap` view but there is **no `dist/views/roadmap.js`** bundle in the plugin. The view declaration exists in the manifest but the actual view component has not been built.
- `PluginViewContainer` will show "Failed to load plugin view" when the roadmap route is activated.
- The `ExplorerRouter.svelte` correctly routes plugin views to `PluginViewContainer` via `navItem.type === "plugin"`.
- The `PluginRegistry` correctly registers plugin views and the `NavigationStore` creates navigation items for them.

**The plumbing works, but no plugin has actually shipped a compiled view bundle.**

---

## 6. Dashboard Health — CLI vs App Discrepancy

### 6.1 How Health Metrics Are Computed

The dashboard calls `artifactGraphSDK.getGraphHealth()` which invokes the Tauri command `get_graph_health` (line 125 of `graph_commands.rs`). This calls `compute_graph_health()` from `orqa_validation::compute_health`.

The integrity scan calls `artifactGraphSDK.runIntegrityScan()` which invokes `run_integrity_scan` (line 313 of `graph_commands.rs`). This calls `check_integrity()` which delegates to `orqa_validation::validate`.

**Both the frontend and CLI use the same Rust validation crate.** The numbers should match.

### 6.2 Possible Sources of Discrepancy

1. **Graph construction divergence**: The Tauri app builds its graph via `build_artifact_graph()` in `app/src-tauri/src/domain/artifact_graph.rs`. The CLI builds via `orqa_validation::graph::build_graph()`. These may scan different directories or use different artifact discovery logic.

2. **Stale graph cache**: The Tauri app caches the graph in `AppState.artifacts.graph`. It's refreshed by `refresh_artifact_graph` or when the file watcher fires. If the watcher misses changes (e.g., bulk file operations during migration), the cached graph is stale.

3. **Plugin relationship loading**: The app's `run_integrity_scan` loads plugin relationships by scanning `plugins/` directory and reading manifests (lines 287-309 of `graph_commands.rs`). The CLI daemon loads relationships differently. If they disagree on which relationships exist, constraint checks differ.

4. **`bidirectionality_ratio`**: The app reports this from `compute_graph_health()`. With forward-only storage, this ratio depends on whether the graph includes computed inverses. If the Tauri graph builder doesn't compute inverses but the CLI does, the ratio differs.

### 6.3 Specific Metric: "104 clusters vs 29"

The cluster count comes from `component_count` in `GraphHealth`. This suggests the Tauri graph builder sees many more disconnected subgraphs than the CLI. The most likely cause: the Tauri graph builder does NOT compute inverse edges (forward-only), so artifacts connected only via inverse traversal appear disconnected. The CLI's validation engine may compute inverses before counting components.

### 6.4 Specific Metric: "0% vs 90.5% traceability"

`pillar_traceability` measures what percentage of rules have a path to a pillar via `grounded-by` relationships. If the Tauri graph doesn't see these relationships (different discovery path, missing plugin), traceability drops to 0%.

---

## 7. What's Missing

### 7.1 No UI for Workflows / State Machines

- 17 resolved YAML workflow files exist in `.orqa/workflows/`
- The `ProjectStatusSettings.svelte` manages the OLD `project.json` status definitions
- No UI to view, edit, or visualize YAML workflow state machines
- No Tauri commands to read/parse workflow YAML files
- The `workflow_tracker` exists in AppState but is not exposed to the frontend

### 7.2 No UI for Gate Reviews

- `process_gates.rs` exists in the Tauri domain layer
- The five-phase gate pipeline (propose, review, decide, execute, verify) has no frontend representation
- No "pending gate reviews" widget on the dashboard
- No Tauri commands to list/approve/reject gates
- `GateQuestions.svelte` exists but only renders static gate question text in artifact frontmatter — it's not an interactive gate review component

### 7.3 No UI for Agent Teams / Metrics

- Token tracker, budget enforcer, and agent spawner were built in `libs/cli`
- No frontend visibility into active agent teams, token usage, or session cost
- No "agent activity" dashboard widget
- No Tauri commands for agent team state

### 7.4 No UI for Knowledge Browsing

- `knowledge_injector.rs` exists in AppState
- 110 knowledge artifacts were migrated with metadata
- No dedicated knowledge browser component
- Knowledge appears in the artifact nav tree (via `ArtifactLanding` with `knowledge` category) but there's no semantic search or tag-based browsing
- No "knowledge graph" visualization (separate from the full artifact graph)

### 7.5 No UI for Prompt Generation Pipeline

- The prompt pipeline (context injection, knowledge retrieval, agent persona selection) was built in `libs/cli`
- No visibility into how prompts are composed
- No "prompt preview" or "context budget" visualization

### 7.6 No Plugin Widget System

- The `software` plugin manifest declares a `widgets` array with a "Delivery Pipeline" widget
- No mechanism exists in the dashboard to load or render plugin-contributed widgets
- The dashboard widgets are all hardcoded in `ProjectDashboard.svelte`

### 7.7 No Artifact Creation UI

- The app can VIEW artifacts but cannot CREATE them
- No "New Epic", "New Task", "New Decision" buttons
- All artifact creation happens via the CLI or manual file editing

### 7.8 No Inline Editing

- Artifacts are read-only in the viewer
- Status changes require editing frontmatter directly (except for auto-transitions and the PipelineStepper which can update via `update_artifact_field`)

---

## 8. What's Dead / Stale

### 8.1 Type Mismatch: IntegrityCategory

**File**: `libs/types/src/artifact-graph.ts` (lines 109-122) and `libs/types/src/generated/validation.generated.ts` (lines 13-26)

The TypeScript types are missing two categories that the Rust validation crate emits:
- `SchemaViolation` — used by enforcement checks, schema checks, and structural checks
- `TypePrefixMismatch` — used by structural checks

The frontend `IntegrityWidget.svelte` `categoryLabels` map (lines 36-49) does not include these. When the backend returns findings with these categories, they render with `undefined` labels in the table.

**Impact**: Data loss in the UI — some integrity findings show broken labels.

### 8.2 Bidirectionality Metric is Misleading

**Files**:
- `app/src/lib/components/dashboard/GraphHealthWidget.svelte` (lines 85-89, 128-133, 268-279)
- `app/src/lib/components/graph/GraphHealthPanel.svelte` (lines 71-75, 154-158, 363-366)

With forward-only relationship storage, the `bidirectionality_ratio` measures what percentage of edges have their inverse STORED. By design, inverses are now computed at query time, not stored. The metric will always report low values and the threshold alerts will permanently fire.

**Impact**: False alarm — the dashboard permanently shows an amber/red alert for bidirectionality being "below 80% target" even though the architecture intentionally doesn't store inverses.

### 8.3 Duplicate Status Transition Engine

**Files**:
- `app/src-tauri/src/commands/status_transition_commands.rs` — standalone transition commands
- `app/src-tauri/src/commands/graph_commands.rs` (lines 151-238) — transition logic duplicated in `refresh_artifact_graph`

Both read from `project.json` status definitions. The standalone version provides `evaluate_status_transitions` and `apply_status_transition`. The graph version does auto-apply within `refresh_artifact_graph`. Both are registered Tauri commands. This is a maintenance risk.

### 8.4 ProjectStatusSettings.svelte — Old Paradigm

**File**: `app/src/lib/components/settings/ProjectStatusSettings.svelte`

This component manages status definitions with `transitions` (allowed next statuses) and `auto_rules` in `project.json`. The new architecture uses YAML workflow state machines that define states, transitions, and guards per artifact type. The settings UI edits the wrong data source.

### 8.5 Hook References (Low Priority)

**Files**:
- `app/src/lib/components/artifact/ArtifactLanding.svelte` (lines 48-55) — hooks category config
- `app/src/lib/components/artifact/ArtifactViewer.svelte` (lines 5, 288) — HookViewer import
- `app/src-tauri/src/commands/hook_commands.rs` — hook Tauri commands

These are not broken — the `githooks` plugin still uses them. But they represent the OLD lifecycle mechanism. The new gate engine handles lifecycle gates. Low priority but creates conceptual confusion about which system drives what.

### 8.6 No Stale Imports of Deleted Types

A search for `MissingInverse` across the entire `app/src` directory returned zero results. The cleanup epic successfully removed all references to the deleted enum variant from the frontend codebase.

### 8.7 No `tmp/` Path References

A search for `tmp/` across `app/src` returned zero results. The `.state/` rename was successful in the frontend.

### 8.8 No `statusTransitions` References in Frontend

A search for `statusTransitions` across `app/src` returned zero results. However, the Tauri backend still has `status_transitions` in 4 files (domain/mod.rs, commands/graph_commands.rs, lib.rs, commands/status_transition_commands.rs).

---

## 9. Recommendations

### Critical (blocks correct behavior)

1. **Sync IntegrityCategory types**: Add `SchemaViolation` and `TypePrefixMismatch` to the JSON schema (`libs/types/src/platform/validation.schema.json`), regenerate TypeScript types, and add labels to `IntegrityWidget.svelte`'s `categoryLabels` map.

2. **Fix bidirectionality metric**: Either (a) remove the bidirectionality_ratio from the dashboard/graph health panel since forward-only storage makes it meaningless as currently computed, or (b) have the backend compute it using inferred inverses so it reflects actual graph navigability.

3. **Investigate graph construction divergence**: The CLI and Tauri app may build different graphs from the same project root. Compare `build_artifact_graph()` in `app/src-tauri/src/domain/artifact_graph.rs` with the CLI's graph builder to find why cluster count and traceability differ.

### High Priority (architectural gaps)

4. **Expose workflow state machines to frontend**: Add Tauri commands to read `.orqa/workflows/*.resolved.yaml`, return current state per artifact, and provide a UI to visualize/interact with workflows.

5. **Expose gate engine to frontend**: Add Tauri commands for gate lifecycle (list pending, propose, approve/reject). Add a dashboard widget for pending gates.

6. **Retire or reconcile status transition systems**: Either remove the old `project.json`-based transition engine and `ProjectStatusSettings.svelte`, or document their relationship to YAML workflows.

### Medium Priority (feature gaps)

7. **Build plugin widget system**: Allow plugins to contribute dashboard widgets (the `software` plugin already declares them in its manifest).

8. **Build knowledge browser**: A dedicated UI for browsing knowledge artifacts with metadata, tags, and semantic search.

9. **Add agent team visibility**: Dashboard widget showing active teams, token usage, session cost.

10. **Ship plugin view bundles**: The `software` plugin's `roadmap` view is declared but has no compiled bundle.

### Low Priority (cleanup)

11. **Deduplicate status transition code**: `status_transition_commands.rs` and the transition logic in `graph_commands.rs` should be consolidated.

12. **Review hooks UI**: Decide whether hook display in `ArtifactLanding` and `ArtifactViewer` should be kept, renamed, or merged with the gate concept.

---

## 10. Summary Statistics

| Metric | Count |
|--------|-------|
| Total Svelte components | ~98 |
| Components with stale/deprecated references | 5 |
| Components with dead imports | 0 |
| Tauri command groups | 18 |
| Tauri commands registered | 46 |
| Tauri commands with no frontend consumer | ~5 (gates, workflow tracker, knowledge injector are in AppState but unexposed) |
| Missing Tauri commands for new features | ~10 (workflows, gates, agent teams, knowledge search) |
| Plugin views declared | 1 |
| Plugin views actually built | 0 |
| New UI areas needed | 5 (workflows, gates, agent teams, knowledge browser, prompt preview) |
