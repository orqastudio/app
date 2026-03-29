# Frontend Architecture Review

## Review Summary

8 areas reviewed. Results: 5 PASS, 2 ARCHITECTURE-GAP, 1 COVERAGE-GAP.

---

## Verdicts

### AC: StatusBar - sidecar handling uses plugin registry, not hardcoded names

**Verdict:** PASS

**Evidence:**
`app/src/lib/components/layout/StatusBar.svelte:41` reads `pluginRegistry.activeSidecar?.label ?? "No sidecar"` -- the provider name is derived from the plugin registry at runtime. The sidecar color and tooltip derivations are driven by the settings store state machine (connected/starting/error/stopped/not_started), which are platform-level states, not governance definitions.

---

### AC: ActivityBar - navigation derived from plugin registry, not hardcoded

**Verdict:** PASS

**Evidence:**
`app/src/lib/components/layout/ActivityBar.svelte:20` reads `navigationStore.topLevelNavItems`. That getter at `libs/sdk/src/stores/navigation.svelte.ts:192` gives priority to `projectStore.navigation` (from project.json), then falls back to `_buildDefaultNavTree()` which merges `PLATFORM_NAVIGATION` with `plugin.manifest.defaultNavigation` from the plugin registry. Plugin-contributed items are dynamically inserted before bottom fixed items.

One note: ActivityBar line 46 hard-codes the keys "artifact-graph", "plugins", and "settings" as bottom-section items. This is a layout constraint for platform-level features, not a governance definition -- not a P1 violation.

---

### AC: ArtifactViewer - artifact type handling uses plugin schemas, not hardcoded list

**Verdict:** PASS

**Evidence:**
`app/src/lib/components/artifact/ArtifactViewer.svelte:210-214` builds `ARTIFACT_ID_RE` from `pluginRegistry.allSchemas.map(s => s.idPrefix)`. No hardcoded artifact type lists in the viewer. The `pipelineStages` derivation at line 189-194 reads from `projectStore.projectSettings?.statuses` (project config), not hardcoded workflow definitions.

---

### AC: MarkdownLink - artifact ID detection uses plugin-derived prefixes, not hardcoded list

**Verdict:** PASS

**Evidence:**
`app/src/lib/components/content/MarkdownLink.svelte:24-31` builds `artifactIdRe` from `pluginRegistry.allSchemas.map(s => s.idPrefix).filter(Boolean)`. Falls back to `/^[A-Z]+-\d+$/` only when no schemas are registered yet. Correct P1-compliant behavior.

---

### AC: SettingsCategoryNav - settings sections

**Verdict:** COVERAGE-GAP

**Evidence:**
The file `app/src/lib/components/settings/SettingsCategoryNav.svelte` does not exist. IDEA-a3f7c912 references it as the location of the accepted-interim static settings nav config. The file has been deleted or renamed. The actual settings navigation implementation is in `app/src/lib/components/layout/SettingsDialog.svelte` (not reviewed). Cannot confirm ACCEPTED-INTERIM status without reading that file.

**Issue:** This review could not confirm the settings navigation implementation. Follow-up needed: read SettingsDialog.svelte and verify whether the settings nav is the accepted static config per IDEA-a3f7c912 or is a gap.

---

### AC: Config files in app/src/lib/config/ - static extraction accepted as ACCEPTED-INTERIM

**Verdict:** ARCHITECTURE-GAP

**Evidence:**
IDEA-a3f7c912 explicitly marks only the settings navigation as ACCEPTED-INTERIM. The following config files contain hardcoded governance vocabulary that is not covered by that interim acceptance:

- `governance-types.ts` hardcodes `ARTIFACT_TYPES` (task, epic, milestone, lesson, decision, rule, research, idea), `ARTIFACT_STATUSES`, and `RELATIONSHIP_TYPES`. Dashboard components import these directly -- e.g., `DecisionQueueWidget.svelte:10-11` imports `ARTIFACT_TYPES` and `ARTIFACT_STATUSES`.
- `action-labels.ts` hardcodes per-type review action labels -- new plugins cannot contribute entries without code changes.
- `relationship-icons.ts` hardcodes the icon mapping for artifact types -- new plugin-defined types get "file-text" fallback only.
- `category-colors.ts` hardcodes lesson category strings (process, coding, architecture).
- `sort-orders.ts` hardcodes `STATUS_ORDER` with specific status key strings.

Dashboard components consuming these constants (`DecisionQueueWidget`, `ImprovementTrendsWidget`, `IntegrityWidget`) cannot adapt when new workflow plugins are installed. This is a direct P1 violation -- governance patterns (artifact types, statuses) are hardcoded in the frontend rather than derived from the plugin registry.

**Issue:** `governance-types.ts` and `action-labels.ts` must be replaced with runtime queries against `pluginRegistry.allSchemas` and `pluginRegistry.allRelationships`. Dashboard components need to derive their artifact type lists from the registry, not static constants.

---

### AC: Plugin registry - provides runtime access to plugin-defined types, statuses, relationships; used by components

**Verdict:** PASS

**Evidence:**
`libs/sdk/src/plugins/plugin-registry.svelte.ts` provides: `allSchemas`, `allRelationships`, `getRelationship(key)`, `validateRelationship(key, from, to)`, `resolveNavigationItem(item)`, `sidecarProviders`. The registry is Svelte 5 reactive (`$state`) so components re-render when plugins register. Components that use it correctly: StatusBar, MarkdownLink, ArtifactViewer, ActivityBar (via navigationStore).

---

### AC: RULE-006 enforcement - no-restricted-syntax in eslint.config.js; no components call invoke() directly

**Verdict:** PASS

**Evidence:**
`app/eslint.config.js:21-27` defines `noInvokeInComponents` with selector `CallExpression[callee.name='invoke']` applied to `src/lib/components/**/*.svelte` and `src/lib/components/**/*.ts` at error severity.

Grep for `invoke(` in `app/src/lib/components/` found zero actual calls -- only a comment at `ConflictResolutionDialog.svelte:70` documenting the rule. All invoke() calls are correctly in services (`plugin-service.ts`, `plugins/loader.ts`) and stores.

---

### AC: Navigation pipeline - PLATFORM_NAVIGATION and plugin contributions, not hardcoded

**Verdict:** PASS

**Evidence:**
`libs/types/src/project.ts:244-283` defines `PLATFORM_NAVIGATION` as a typed const array. `NavigationStore._buildDefaultNavTree()` at `libs/sdk/src/stores/navigation.svelte.ts:211-233` merges PLATFORM_NAVIGATION with plugin manifest `defaultNavigation` contributions. Path resolution uses `pluginRegistry.allSchemas` at line 686. Navigation tree prioritizes explicit `project.json` config over the platform default.

---

## Blocking Issues

1. **P1 VIOLATION: governance-types.ts hardcodes artifact types and statuses** -- `ARTIFACT_TYPES`, `ARTIFACT_STATUSES`, and related constants are frontend definitions of governance vocabulary that should be derived from the plugin registry. Dashboard components importing these cannot adapt to new plugin-defined types or statuses without code changes. Fix: source these from `pluginRegistry.allSchemas` and `pluginRegistry.allRelationships` at runtime, and update dashboard components to use the registry queries.

2. **COVERAGE GAP: SettingsCategoryNav.svelte missing** -- The file referenced in IDEA-a3f7c912 as the accepted-interim settings nav location does not exist. The settings navigation implementation was not reviewed. If settings sections are hardcoded in SettingsDialog.svelte, the ACCEPTED-INTERIM status needs to be confirmed against that file before acceptance.

---

## Plugin View SDK and Component Library

### AC: libs/sdk/ -- shared SDK exists and is consumable by plugins

**Verdict:** PASS

**Evidence:**
`libs/sdk/package.json` exports `@orqastudio/sdk` with named entry points:

- `.` (index) -- `getStores`, `initializeStores`, store classes, `PluginRegistry`, `ArtifactGraphSDK`, `logger`, router helpers
- `./graph` -- `ArtifactGraphSDK`
- `./ipc` -- `invoke`, `createStreamChannel`
- `./utils` -- `parseFrontmatter`
- `./stores` -- all store classes

The package declares `@tauri-apps/api >= 2.0.0` and `svelte >= 5.0.0` as peer dependencies. It is published as `@orqastudio/sdk`. First-party plugins depend on it: `plugins/workflows/software-kanban/package.json` lists `"@orqastudio/sdk": "0.1.4-dev"` in peerDependencies.

The software-kanban plugin uses `getStores()` from `@orqastudio/sdk` directly in its view components (`RoadmapView.svelte:7`), getting access to `artifactGraphSDK`, `navigationStore`, and `projectStore` -- the same store instances the core app uses.

---

### AC: libs/svelte-components/ -- shared component library exists and is consumable by plugins

**Verdict:** PASS

**Evidence:**
`libs/svelte-components/package.json` exports `@orqastudio/svelte-components` with named entry points:

- `./pure` -- all UI primitives: Button, Badge, Card, Dialog, Tabs, Icon, Status, ScrollArea, EmptyState, LoadingSpinner, Sparkline, MetricCell, PipelineStages, Tooltip, Popover, DropdownMenu, Collapsible, Resizable, and ~20 more
- `./connected` -- store-connected components: StatusIndicator, ArtifactListItem, ArtifactLink, AppShell, ActivityBar, NavSubPanel, StatusBar, ToastContainer
- `./testing` -- test utilities
- `./tokens` -- CSS design tokens

First-party plugins consume this correctly. `KanbanCard.svelte` in software-kanban imports `StatusIndicator` from `@orqastudio/svelte-components/connected` and `SmallBadge` from `@orqastudio/svelte-components/pure`. `RoadmapView.svelte` imports `Icon`, `EmptyState`, `LoadingSpinner`, `ErrorDisplay` from the pure entry point.

---

### AC: libs/types/ -- shared types for plugins

**Verdict:** PASS

**Evidence:**
`libs/types/package.json` exports `@orqastudio/types` as a standalone package. It exposes all platform types: `ArtifactNode`, `ProjectSettings`, `PluginManifest`, `ArtifactSchema`, `RelationshipType`, `NavigationItem`, `WorkflowDefinition`, `EnforcementViolation`, etc. The full index at `libs/types/src/index.ts` re-exports 50+ types across all domains.

Plugins can declare `"@orqastudio/types": "0.1.4-dev"` as a direct dependency (not peerDep) since it is type-only. software-kanban uses it: `import type { ArtifactNode } from "@orqastudio/types"` in KanbanCard.svelte.

---

### AC: libs/graph-visualiser/ -- available to plugins

**Verdict:** ARCHITECTURE-GAP

**Evidence:**
`@orqastudio/graph-visualiser` exists as a published package (`libs/graph-visualiser/package.json`) and is used by the core app (`app/src/lib/graph-viz.svelte.ts:8`, `RelationshipGraphView.svelte:8`). It exports `GraphVisualiser`, `buildVisualizationElements`, `ARTIFACT_TYPE_COLORS`, and `GraphDataSource`.

However, `@orqastudio/graph-visualiser` is NOT exposed via `window.__orqa` in `app/src/lib/plugins/shared-modules.ts`. The shared modules object only exposes `sdk`, `components`, `componentsConnected`, and `svelte`. Plugin vite configs (e.g., `software-kanban/vite.config.ts`) only list the four exposed entries in `rollupOptions.external` -- `@orqastudio/graph-visualiser` is not among them.

A plugin that tries to use the graph visualiser would either:
(a) bundle a duplicate copy (bloating the plugin bundle and losing shared reactive state), or
(b) attempt to import it at runtime and fail (it is not in the globals map).

**Issue:** If plugins should be able to build graph visualisation views using the same library as the core app, `@orqastudio/graph-visualiser` must be added to `window.__orqa` in `shared-modules.ts` and added to the `external`/`globals` in plugin vite configs. Currently it is not pluggable.

---

### AC: PluginViewContainer -- plugin views load correctly with access to SDK and components

**Verdict:** PASS

**Evidence:**
`app/src/lib/components/plugin/PluginViewContainer.svelte`:

1. Gets the plugin install path via `getPluginPath(pluginName)` (correct -- goes through the service layer, no direct invoke in component)
2. Loads `dist/views/{viewKey}.js` from the plugin path via `convertFileSrc` + dynamic `import()`
3. Accepts either a `mount` function export (preferred) or a default Svelte component export
4. Calls `cleanup()` on `onDestroy` -- no memory leak

The `+layout.svelte` calls `exposeSharedModules()` before any plugin bundles are loaded (line 13), ensuring `window.__orqa` is populated before PluginViewContainer could ever be mounted. The timing is correct.

The software-kanban plugin demonstrates the full contract works: its `vite.config.ts` marks SDK and component library as externals that resolve from `window.__orqa`, and its views call `getStores()` to get the live store instances.

---

### AC: Documented API surface / exports for plugin developers

**Verdict:** ARCHITECTURE-GAP

**Evidence:**
There is no documented plugin development guide, no README in `libs/sdk/` or `libs/svelte-components/` describing what is available to plugin authors, and no example plugin template or scaffold. The contract between the app and plugin views (the `window.__orqa` global shape, the expected `dist/views/{viewKey}.js` output path, the `mount` function or default export convention) is implicit -- visible only by reading vite configs and `shared-modules.ts`.

Specifically missing:

- No `libs/sdk/README.md` or similar describing the plugin development API
- No documented list of which `window.__orqa` fields are stable vs internal
- The `PluginViewContainer` contract (mount function signature, cleanup return, path convention) is documented only in component comments, not in an accessible plugin developer reference
- The `vite.config.ts` pattern for marking externals is implied by the software-kanban example but not referenced from any documentation

**Issue:** The plugin view API surface exists and works (demonstrated by software-kanban) but is not documented. Plugin authors outside the core team have no reference for what to import, how to configure their build, or what the expected output structure is. This is a completeness gap rather than a functional gap -- the infrastructure works, the documentation doesn't exist.

---

### Summary: Plugin View Infrastructure

| Area | Verdict |
|------|---------|
| `@orqastudio/sdk` as installable package | PASS |
| `@orqastudio/svelte-components` as installable package | PASS |
| `@orqastudio/types` as installable package | PASS |
| `@orqastudio/graph-visualiser` accessible to plugins | ARCHITECTURE-GAP (not in `window.__orqa`) |
| PluginViewContainer loading and store injection | PASS |
| Plugin developer documentation | ARCHITECTURE-GAP (no docs exist) |

The core infrastructure is sound: plugins get access to the same store instances and component library as the core app via `window.__orqa`. The missing pieces are the graph-visualiser not being pluggable and the absence of any plugin developer documentation.

---

## Three-Pillar Frontend Architecture

### Pillar 1: Ecosystem Management (Plugins, Settings, Configuration)

#### Settings Navigation Implementation

The file previously referenced as SettingsCategoryNav.svelte was found at app/src/lib/components/navigation/SettingsCategoryNav.svelte (not settings/). This resolves the COVERAGE-GAP from the first review section.

The navigation sections are hardcoded as two static TypeScript constant arrays appCategories and projectGroups in SettingsCategoryNav.svelte:43-134. This is the ACCEPTED-INTERIM configuration referenced in IDEA-a3f7c912.

**Verdict:** ACCEPTED-INTERIM (confirmed)

- App-level sections (provider, model, appearance, shortcuts) are global platform features, not governance definitions. Hardcoding them is defensible.
- Project-level sections (Methodology, Sidecar, Connector, Plugins) are also static. New plugins cannot contribute settings pages to this nav without code changes to this file.
- IDEA-a3f7c912 accepts this explicitly as interim. No blocking issue at this time, but the resolution path (plugin manifest settings_pages declarations) is not yet implemented.

#### Plugin Browser

app/src/lib/components/settings/PluginBrowser.svelte provides a full ecosystem management UI:

- Tabs: Installed / Official / Community / Groups
- Install from GitHub registry or local path with conflict detection (ConflictResolutionDialog)
- Uninstall with confirmation
- Detail view renders manifest contents: schemas, relationships, views, widgets, hooks
- All operations go through pluginStore and pluginRegistry from @orqastudio/sdk -- no direct invoke() in the component

**Verdict:** PASS -- the plugin browser is a complete plugin lifecycle management UI driven by the plugin registry.

#### Settings Content (SettingsView)

app/src/lib/components/settings/SettingsView.svelte switches sections via a static if-chain over activeSection. Each section renders a purpose-built settings component (ProviderSettings, ModelSettings, AppearanceSettings, ProjectStatusSettings, RelationshipSettings, etc.).

The project-plugins section renders PluginBrowser -- plugins are managed within the settings flow. This is architecturally correct.

Section routing is static. No plugin can inject a new settings page via manifest declaration.

**Verdict:** ACCEPTED-INTERIM (same rationale as settings nav).

---

### Pillar 2: AI Assistance (Chat Panel and Daemon/Sidecar Integration)

#### Chat Panel Architecture

app/src/lib/components/conversation/ConversationView.svelte provides a full chat panel:

- Session management (create, restore, list via sessionStore)
- Message history loading (conversationStore.loadMessages(session.id))
- Streaming responses with streamingContent, streamingThinking (thinking blocks)
- Active tool call display (ToolCallSummary)
- Tool approval workflow (ToolApprovalDialog) -- user can approve/deny tool calls mid-stream
- Process violation display (processViolations)
- Context entry display (contextEntries for system_prompt_sent, context_injected)
- Auto-scroll with user scroll detection

All state access through conversationStore, sessionStore, projectStore, settingsStore from @orqastudio/sdk. No direct invoke() in the component -- RULE-006 compliant.

**Verdict:** PASS for component structure.

#### Sidecar Integration Path

libs/sdk/src/stores/conversation.svelte.ts communicates with the backend via Tauri IPC:

- invoke(stream_send_message) -- sends message and opens a stream channel for events
- invoke(stream_stop) -- cancels streaming
- invoke(message_list) -- loads persisted messages
- invoke(stream_tool_approval_respond) -- tool approval responses
- oneShotMessage() -- one-shot inference via sessionId: -1 convention

The connection path is Tauri IPC to a Rust command, not direct sidecar communication. The Rust backend manages the sidecar subprocess and pipes messages. The frontend never talks to the sidecar directly. This is architecturally correct per DOC-62969bc3 language boundary principle.

Architecture note: conversationStore is in @orqastudio/sdk, consumed by plugin views via getStores(). Plugin views can access sendMessage() and oneShotMessage(). Whether plugin views should initiate AI conversations is undocumented -- open question about plugin permissions, not a current defect.

**Verdict:** PASS

#### Context Injection Visibility

The conversation store surfaces contextEntries (system_prompt_sent and context_injected) as reactive state. ConversationView.svelte:37 derives contextEntries and renders them via ContextEntry.svelte, giving users visibility into what system prompt and knowledge context was injected before each turn.

**Verdict:** PASS

---

### Pillar 3: Artifact Viewer (Default Built-in Viewer, Plugin-Extendable)

#### Universal ArtifactViewer

app/src/lib/components/artifact/ArtifactViewer.svelte is the universal artifact viewer. It renders: breadcrumb, frontmatter header, pipeline stepper (derived from project status machine), acceptance criteria panel, hook viewer, markdown body, references panel, and traceability panel.

The viewer is invoked from ExplorerRouter.svelte whenever navigationStore.selectedArtifactPath is set, regardless of artifact type. No per-type dispatch exists -- every artifact goes to the same viewer.

#### Plugin Custom Viewer Registration

There is NO mechanism for plugins to register custom viewers for specific artifact types. Grep for registerViewer, getViewerForType, customViewer, and artifactType.*view patterns returned no results in app/src/, libs/sdk/src/, or plugin code.

Plugins contribute views via manifest.provides.views (navigation items opening in PluginViewContainer), but these are standalone views accessed via the activity bar. They cannot replace or override ArtifactViewer for specific artifact types.

ExplorerRouter.svelte routes all selectedArtifactPath to ArtifactViewer unconditionally:

```js
if (navigationStore.selectedArtifactPath) {
  return { type: core, component: ArtifactViewer };
}
```

The architecture documents (DOC-62969bc3, DOC-41ccf7c4) do not explicitly specify whether plugins should be able to register type-specific artifact viewers. The team-lead framing described ArtifactViewer as default built-in viewer that plugins can extend or replace. Under the current implementation, plugins can contribute standalone nav views but CANNOT replace or override ArtifactViewer for a specific artifact type path.

**Verdict:** ARCHITECTURE-GAP -- the plugin override mechanism for the artifact viewer does not exist. Architectural intent must be confirmed.

---

### Three-Pillar Summary

| Pillar | Area | Verdict |
|--------|------|---------|
| Ecosystem | Plugin Browser | PASS |
| Ecosystem | Settings Navigation | ACCEPTED-INTERIM |
| Ecosystem | Settings Content Routing | ACCEPTED-INTERIM |
| AI Assistance | Chat Panel Component | PASS |
| AI Assistance | Sidecar Integration Path | PASS |
| AI Assistance | Context Injection Visibility | PASS |
| Artifact Viewer | Universal Viewer (ArtifactViewer) | PASS |
| Artifact Viewer | Plugin type-specific viewer override | ARCHITECTURE-GAP |

### New Blocking Issue (Three-Pillar)

**ARCHITECTURE-GAP: No plugin artifact viewer override mechanism** -- ExplorerRouter.svelte sends all artifact paths to ArtifactViewer unconditionally. The plugin registry has no API for registering a custom viewer component for a specific artifact schema key. If workflow or domain plugins should render their artifact types with custom UIs when the user navigates to an artifact path, that pathway does not exist. Requires: (a) plugin manifest `provides.artifact_viewers` block mapping schema keys to view components; (b) `PluginRegistry.getViewerForArtifactType(schemaKey)` method; (c) ExplorerRouter checking the registry before falling back to ArtifactViewer. Architectural intent must be confirmed before implementing.

---

## Shared SDK / Dog-Food Principle Review

**Question:** Does the core app eat its own dog food -- importing from @orqastudio/sdk and @orqastudio/svelte-components the same way plugins do? Or does the core app use internal APIs that plugins cannot access?

### Finding 1: The core app primarily uses shared packages -- PASS on the main principle

Across app/src/lib/components/ there are 336 @orqastudio/* imports (98 files) vs 84 dollar-lib imports (46 files). The majority of the app is built on the shared packages. Critical components like ArtifactViewer, ConversationView, PluginBrowser, StatusBar, ActivityBar, MarkdownLink all use getStores() from @orqastudio/sdk and components from @orqastudio/svelte-components/pure and /connected.

The parseFrontmatter utility (used in 5+ components via /utils/frontmatter) is a thin re-export: app/src/lib/utils/frontmatter.ts re-exports directly from @orqastudio/sdk. No private implementation.

### Finding 2: ArtifactLink has a split implementation -- ARCHITECTURE-GAP

There are two ArtifactLink components:

**Shared (stub) -- libs/svelte-components/src/connected/artifact-link/ArtifactLink.svelte (12 lines)**

- Props: { artifactId: string }
- Uses mock-stores.js for Storybook
- Renders a plain button with title or artifactId
- No color, no tooltip, no status icon, no multi-project support, no display mode

**Private (full) -- app/src/lib/components/artifact/ArtifactLink.svelte (169 lines)**

- Props: `{ id?, path?, displayLabel? }`
- Uses `getStores()` from `@orqastudio/sdk` (correct)
- Full chip: color-mix border/bg from project settings, status icon with spin, tooltip with node metadata
- Multi-project qualified ID support (`sdk::EPIC-001`)
- Display mode: id vs title from project settings

The core app imports the full version from /components/artifact/ArtifactLink.svelte. The shared package exposes only the stub. Plugins receive the stub via window.__orqa.componentsConnected.ArtifactLink.

A plugin that builds a view displaying artifact relationships (like a kanban card that links to epics) cannot use the same artifact link chip that the core app uses. This violates the extensibility-by-design principle.

**Verdict:** ARCHITECTURE-GAP -- the real ArtifactLink must be moved to libs/svelte-components/src/connected/ and the stub replaced. The app should import ArtifactLink from @orqastudio/svelte-components/connected, not from /.

### Finding 3: MarkdownRenderer is private -- ARCHITECTURE-GAP

app/src/lib/components/content/MarkdownRenderer.svelte is the only markdown renderer available. It handles:

- Frontmatter stripping (via parseFrontmatter from `@orqastudio/sdk`)
- Custom `:::artifacts{}` directive preprocessing
- MarkdownLink injection (auto-links artifact IDs to the artifact chip)
- Diagram rendering via DiagramCodeBlock

This component is not in `@orqastudio/svelte-components`. Plugins that want to render artifact body content (e.g., a custom document viewer, a knowledge viewer) cannot use the same renderer that the core app uses. They would need to re-implement frontmatter stripping and the artifact ID linking themselves.

**Verdict:** ARCHITECTURE-GAP -- MarkdownRenderer should be promoted to libs/svelte-components and exported from @orqastudio/svelte-components/pure or a new /content entry point.

### Finding 4: frontmatter-config.ts constants are private -- MINOR GAP

app/src/lib/config/frontmatter-config.ts defines SKIP_FIELDS, DATE_FIELDS, LINK_FIELDS, CHIP_FIELDS, BOOLEAN_FIELDS, FIELD_ORDER, priorityClass(), priorityLabel(). These are the display-routing decisions for FrontmatterHeader rendering.

A plugin author building a custom artifact viewer would need to know which fields to skip, which to render as dates, which as chips, etc. These constants are not available in any shared package.

**Verdict:** MINOR -- only matters if a plugin wants to replicate the FrontmatterHeader rendering. Lower priority than ArtifactLink and MarkdownRenderer.

### Finding 5: plugin-service.ts getPluginPath() is private -- by design

app/src/lib/services/plugin-service.ts exports getPluginPath(name) which wraps invoke(plugin_get_path). This is used only by PluginViewContainer to load plugin bundles. Plugins do not need to query their own install path -- the app provides this when mounting them. This is correct by design.

**Verdict:** PASS -- not a gap.

### Finding 6: tool-display utilities are private -- minor gap for AI conversation plugin authors

app/src/lib/utils/tool-display.ts provides getToolDisplay(), groupLabel(), getEphemeralLabel(), getActivityPhase() -- utility functions for rendering tool call names, icons, and labels in the conversation UI. These are not in @orqastudio/sdk.

A plugin that wants to render tool call history (e.g., a custom conversation view or an activity log plugin) cannot use these utilities.

**Verdict:** MINOR -- niche use case, but a gap in the dog-food principle.

### Summary Table

| Component / API | Available to plugins? | Gap type |
|---|---|---|
| getStores(), ArtifactGraphSDK, navigateToArtifact | YES -- @orqastudio/sdk | PASS |
| parseFrontmatter | YES -- @orqastudio/sdk (app re-exports) | PASS |
| svelte-components/pure (Button, Icon, Card, etc.) | YES -- window.__orqa.components | PASS |
| svelte-components/connected (StatusIndicator, etc.) | YES -- window.__orqa.componentsConnected | PASS |
| ArtifactLink (full chip with color, tooltip, status) | NO -- private to app | ARCHITECTURE-GAP |
| MarkdownRenderer (with artifact linking, diagrams) | NO -- private to app | ARCHITECTURE-GAP |
| frontmatter-config.ts display constants | NO -- private to app | MINOR |
| tool-display.ts utilities | NO -- private to app | MINOR |
| graph-visualiser | NO -- not in window.__orqa | ARCHITECTURE-GAP (reported earlier) |

### Blocking Issues (dog-food principle)

1. **ARCHITECTURE-GAP: ArtifactLink full implementation is private to app** -- The real artifact link chip (color, tooltip, status, multi-project) exists only in app/src/lib/components/artifact/ArtifactLink.svelte. @orqastudio/svelte-components/connected exports a stub. Plugins cannot build views that display artifact links with the same fidelity as core app components. Fix: move full implementation to libs/svelte-components/src/connected/artifact-link/, update the export, update app imports to use @orqastudio/svelte-components/connected.

2\. **ARCHITECTURE-GAP: MarkdownRenderer is private to app** -- Plugins cannot render artifact markdown content (with artifact ID auto-linking, diagram blocks, frontmatter stripping) using the same component the app uses. Fix: promote to libs/svelte-components and export from @orqastudio/svelte-components.
