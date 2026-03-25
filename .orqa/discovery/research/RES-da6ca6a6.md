---
id: RES-da6ca6a6
title: "UX View Architecture Research: Core vs Plugin Views, Navigation, and Visualization"
type: research
status: active
category: ux-architecture
description: >
  Independent UX research investigating how OrqaStudio's plugin-composed architecture
  should express itself in the desktop app UI. Covers core vs plugin view boundaries,
  navigation model, dashboard structure, workflow visualization, agent/team visibility,
  and plugin management. Based on analysis of the current codebase, plugin manifests,
  type definitions, and the Team Design v2 research document.
created: 2026-03-25
relationships:
  - target: RES-d6e8ab11
    type: related
    rationale: "Architecture context for workflow, agent, and state machine design"
tags:
  - ux
  - views
  - navigation
  - plugin-architecture
  - dashboard
  - workflow-visualization
---

# UX View Architecture Research

## 1. Current State Analysis

### What Exists Today

The app currently has a VS Code-inspired layout with four structural zones:

1. **Toolbar** (top) -- app-level actions, window controls
2. **ActivityBar** (left, 48px) -- icon-based primary navigation
3. **NavSubPanel** (200px) -- secondary navigation for the active group/category
4. **Explorer + Chat** (resizable panes) -- main content area with embedded conversation panel
5. **StatusBar** (bottom) -- background status

The `ExplorerRouter` routes between three core views and a plugin view container:

| Key | Component | Description |
|-----|-----------|-------------|
| `project` | `ProjectDashboard` | Dashboard with 7 widgets |
| `artifact-graph` | `FullGraphView` | Force-directed graph visualization |
| `welcome` | `WelcomeScreen` | No-project landing page |
| (plugin) | `PluginViewContainer` | Runtime-loaded plugin view |

Artifact browsing is handled by `ArtifactNav` (list panel) + `ArtifactViewer` (detail panel), not as separate views.

### Current Dashboard Widgets

The `ProjectDashboard` has a fixed layout with these widgets, all hardcoded:

1. **MilestoneContextCard** -- active milestone, P1 epic progress, gate, deadline
2. **GraphHealthWidget** -- largest connected component ratio, integrity scan counts
3. **ImprovementTrendsWidget** -- "Learning" column
4. **DecisionQueueWidget** -- artifacts in "review" status, active epics
5. **PipelineWidget** -- governance learning loop (lesson -> decision -> rule -> lesson)
6. **LessonVelocityWidget** -- lesson creation/promotion rate
7. **IntegrityWidget** -- full integrity check results with filtering
8. **ToolStatusWidget** -- CLI tool run statuses from plugins

### Current Plugin View Declarations

Across all 12 plugins, only the **software plugin** declares views and widgets:

| Plugin | Views | Widgets |
|--------|-------|---------|
| `@orqastudio/plugin-software-project` | `roadmap` (Kanban icon) | `pipeline` (Delivery Pipeline), `milestone-context` |
| All other 11 plugins | `[]` | `[]` |

The software plugin also provides `defaultNavigation` with two groups: **Discovery** (research, wireframes) and **Delivery** (roadmap, milestones, epics, tasks).

### Navigation Model

The app supports two navigation modes:

1. **New model** -- `project.json` has a `navigation` array (NavigationItem tree). Items can be `builtin`, `plugin`, or `group` type.
2. **Legacy model** -- `project.json` has an `artifacts` array of groups/types with filesystem paths.

The current dev project uses the legacy `artifacts` model (no `navigation` key in project.json). The new navigation model is implemented but not yet activated for this project.

### View Type System

From `libs/types/src/plugin.ts`:

- **ViewRegistration**: `{ key, label, icon }` -- a full-page view a plugin contributes
- **WidgetRegistration**: `{ key, label, icon, defaultPosition, defaultSpan }` -- a dashboard widget with grid positioning
- **DefaultNavItem**: plugin-recommended navigation tree additions
- **NavigationItem**: project-level navigation config (stored in project.json)

Plugin views are loaded at runtime via `PluginViewContainer`, which dynamically imports `dist/views/{viewKey}.js` from the plugin directory and mounts the Svelte component.

---

## 2. Core vs Plugin Views

### Question: What should core provide vs what plugins contribute?

### Finding: Clear Separation Already Exists

The architecture already has a clean separation principle: **core provides engines, plugins provide definitions**. This should extend to views.

#### Core Views (independent of any plugin)

These views operate on core graph engine data and are meaningful regardless of which plugins are installed:

| View | Purpose | Data Source |
|------|---------|-------------|
| **Project Dashboard** | Health overview, action queue, trends | Graph engine (health, integrity, traceability) |
| **Artifact Graph** | Full relationship visualization | Graph engine (nodes, edges, components) |
| **Artifact Browser** | Master-detail for any artifact type | Graph engine + filesystem (artifact content) |
| **Artifact Viewer** | Single artifact with frontmatter, relationships, traceability | Graph engine + filesystem |
| **Search** | Semantic/text search across all artifacts | Search engine (ONNX embeddings) |
| **Settings** | Project configuration, plugin management | Project config (project.json) |
| **Conversation** | Chat with AI agent | Sidecar (Claude/other provider) |

These 7 core views handle the fundamental activities: orient (dashboard), explore (graph, browser), work (viewer, conversation), search, configure.

#### Plugin Views (contributed by domain plugins)

These views present domain-specific interpretations of artifact data:

| View Category | Example | Plugin Owner |
|---------------|---------|--------------|
| **Roadmap/Kanban** | Board view of epics/tasks by status | Delivery plugin (e.g., software) |
| **Timeline/Gantt** | Time-based milestone visualization | Delivery plugin |
| **Workflow Designer** | Visual state machine editor | Governance plugin |
| **Knowledge Map** | Semantic clustering of knowledge artifacts | Thinking plugin (e.g., systems-thinking) |
| **Gate Review** | Structured review interface for human gates | Governance plugin |
| **Learning Dashboard** | Lesson patterns, recurrence analysis | Governance plugin |
| **Code Coverage Map** | Test coverage mapped to artifacts | Coding standards plugin |
| **Architecture Map** | System component relationships | Software plugin |

### Recommendation: Formalize the Core View Registry

The current `CORE_VIEWS` object in `ExplorerRouter.svelte` is the right pattern. Formalize it:

1. Core views are registered in `ExplorerRouter` with static imports -- they ship with the app and are always available.
2. Plugin views are loaded dynamically via `PluginViewContainer` -- they appear only when the providing plugin is installed.
3. The dashboard should be composable: core provides the layout frame, plugins contribute widgets via their manifests (this is already the widget system design, but the dashboard currently hardcodes all widgets).

### How Plugin Views Should Be Discovered and Displayed

The existing infrastructure is sound:

1. Plugin declares views in `provides.views[]` with key, label, icon.
2. Plugin declares `defaultNavigation[]` recommending where the view appears in the nav tree.
3. At install time, `defaultNavigation` items are merged into the project's `navigation` config.
4. The `ActivityBar` renders the navigation tree. Plugin items appear alongside builtin items.
5. When a plugin nav item is activated, `ExplorerRouter` routes to `PluginViewContainer`.
6. `PluginViewContainer` dynamically imports the plugin's bundled view component.

What is missing:

- **Widget discovery for the dashboard** -- the dashboard hardcodes widgets. It should read from `pluginRegistry.allWidgets` and render them dynamically using `getWidgetComponent()`.
- **View capability declarations** -- plugins should declare what data their views need (e.g., "requires graph access", "requires artifact type: task") so the UI can show/hide gracefully.
- **Fallback for missing plugin views** -- if a plugin declares a view but hasn't built it yet, the `PluginViewContainer` error state handles this, but the nav should also indicate unbuildable items.

---

## 3. Navigation Model

### Finding: The Two-Model System Should Be Consolidated

The app supports both legacy `artifacts` config and the new `navigation` tree. The new model is strictly more capable:

- Supports `builtin`, `plugin`, and `group` types
- Plugin views can live alongside artifact lists in groups
- Groups can nest plugin and builtin items together

### Recommendation: Navigation Architecture

```
ActivityBar (left sidebar, icon-only, 48px)
  |
  +-- Dashboard (builtin, always first)
  +-- [Plugin groups from defaultNavigation]
  |     e.g., Discovery (research, wireframes)
  |     e.g., Delivery (roadmap, milestones, epics, tasks)
  +-- [Governance group from plugin]
  |     e.g., Process (rules, agents, knowledge, lessons, decisions)
  |     e.g., Principles (pillars, personas, vision)
  +-- --- separator ---
  +-- Artifact Graph (builtin, always at bottom)
  +-- Search (builtin, triggers overlay)
  +-- --- separator ---
  +-- Settings (builtin, always last)
```

Key principles:

1. **Dashboard is always first** -- it's the orientation point.
2. **Plugin-contributed groups appear in declared order** -- plugins use `defaultNavigation` to suggest grouping. The project owner can reorder in Settings > Navigation.
3. **Artifact Graph and Search are always at the bottom** -- they are cross-cutting tools, not domain-specific.
4. **Settings is always last** -- convention from VS Code, Figma, etc.
5. **The conversation panel is NOT a nav item** -- it's an always-available side panel (already the case). This is correct because the conversation is orthogonal to the current view.
6. **No command palette yet** -- the existing `Ctrl+Space` search overlay serves the quick-access role. A full command palette (like VS Code's `Ctrl+Shift+P`) is a future enhancement for executing actions, not just navigating.

### Group Navigation

When a group icon is clicked in the ActivityBar:

1. The group's children appear in the NavSubPanel (200px).
2. If the group has only one child, the NavSubPanel is hidden and the child is activated directly.
3. Clicking a child item either shows an artifact list (for `builtin` type) or loads a plugin view (for `plugin` type).

This is already implemented correctly in `NavigationStore.setGroup()` and `NavSubPanel`.

---

## 4. Default Views and Dashboard Architecture

### What the User Should See on First Launch

**Before a project is loaded:**
- WelcomeScreen with "Open Project" action (current implementation is correct).

**After a project is loaded (first time):**
- ProjectSetupWizard (current implementation is correct -- detects missing project.json settings).

**After setup is complete:**
- **Project Dashboard** -- the orientation view. This is the correct default (already implemented in the `$effect` in AppLayout that switches from "chat" to "project" when a project becomes active).

### Information Hierarchy on the Dashboard

The current dashboard layout follows a good narrative structure. Based on the governance practitioner's needs, the hierarchy should be:

| Priority | Information Need | Current Widget | Assessment |
|----------|-----------------|----------------|------------|
| **1. What needs my attention?** | Items requiring human decisions | DecisionQueueWidget | Good -- shows "review" status items and active epics |
| **2. Where am I relative to the goal?** | Milestone progress | MilestoneContextCard | Good -- shows active milestone, P1 progress, deadline |
| **3. Is the project healthy?** | Graph integrity, connected components | GraphHealthWidget | Good -- largest component ratio as health score |
| **4. What's the work pipeline look like?** | Artifact flow through stages | PipelineWidget | Good -- governance learning loop |
| **5. Are we learning?** | Lesson velocity, recurrence patterns | LessonVelocityWidget, ImprovementTrendsWidget | Good -- two widgets for learning signals |
| **6. Are the tools working?** | CLI tool status, integrity checks | ToolStatusWidget, IntegrityWidget | Good -- operational status |

### Recommendation: Make the Dashboard Plugin-Composable

The current dashboard hardcodes all 8 widgets. The widget system types (`WidgetRegistration` with `defaultPosition` and `defaultSpan`) already support a grid-based composable dashboard. The transition should be:

1. **Core dashboard frame** provides:
   - Project header (name, description, icon) -- always present
   - A CSS Grid container that reads widget registrations from the plugin registry
   - Default grid layout from `defaultPosition`/`defaultSpan` on each widget
   - User override of positions/spans stored in project.json

2. **Core widgets** (shipped with the app, not from plugins):
   - `decision-queue` -- items needing human action (depends only on graph engine "review" status)
   - `graph-health` -- connected component ratio, integrity summary (depends only on graph engine)
   - `integrity` -- full integrity check list (depends only on graph engine)
   - `health-trend` -- sparkline of health snapshots over time (depends only on graph engine)

3. **Plugin widgets** (contributed via `provides.widgets`):
   - `pipeline` (software plugin) -- governance learning loop visualization
   - `milestone-context` (software plugin) -- active milestone progress
   - `lesson-velocity` (governance plugin) -- lesson creation/promotion rates
   - `improvement-trends` (governance plugin) -- learning trend analysis
   - `tool-status` (from any plugin with CLI tools) -- tool run statuses

4. **Widget rendering**:
   - For core widgets, use static imports (like `CORE_VIEWS`).
   - For plugin widgets, use `pluginRegistry.getWidgetComponent(pluginSource, widgetKey)`.
   - Widgets that fail to load show a graceful error state within their grid cell.

### Recommendation: Dashboard Sections

Rather than a flat grid, organize the dashboard into semantic sections that make sense even if some widgets are absent:

```
Section 1: "Attention Required" (always visible)
  - Decision Queue (core)
  - Gate Reviews pending (plugin, if governance plugin installed)

Section 2: "Progress" (visible if delivery plugin installed)
  - Milestone Context (plugin)
  - Pipeline/Kanban summary (plugin)

Section 3: "Health" (always visible)
  - Graph Health (core)
  - Health Trend (core)
  - Integrity summary (core)

Section 4: "Learning" (visible if governance plugin installed)
  - Lesson Velocity (plugin)
  - Improvement Trends (plugin)

Section 5: "Tools" (visible if any plugin has CLI tools)
  - Tool Status (core, but populated by plugins)
```

Sections with no available widgets should not render (no empty placeholders).

### Artifact Browsing with Plugin-Defined Types

The current system handles this well:

1. Plugins declare `schemas[]` with `key`, `label`, `icon`, `defaultPath`, `idPrefix`.
2. The navigation tree groups artifact types into logical categories.
3. `ArtifactNav` lists artifacts from the filesystem path associated with the type.
4. `ArtifactViewer` renders any artifact generically: frontmatter header, pipeline stepper, markdown body, traceability panel, relationships list.

The generic rendering approach is correct for a plugin-composed system. Plugin-specific artifact rendering should be opt-in -- a plugin can register a custom viewer for its artifact types via a new `artifactViewers` registration in the manifest, but the default `ArtifactViewer` must work for everything.

---

## 5. Workflow Visualization

### Current State

There are 17 workflow YAML files across two plugins:

| Plugin | Workflows |
|--------|-----------|
| agile-governance | vision, pillar, persona, idea, decision, rule, lesson, knowledge, agent, doc, pivot, epic, task, delivery (14) |
| software | milestone, research, wireframe (3) |

Each workflow defines states with categories (planning, active, review, completed, terminal), transitions with guards and actions, gates (human approval sub-workflows), and variants.

The `PipelineStepper` component already renders workflow states as a horizontal stepper on each artifact, showing past/current/future states with clickable transitions. This reads from `projectStore.projectSettings?.statuses` -- a flat list of status definitions.

### Recommendation: State Machine Visualization on Artifacts

The `PipelineStepper` is good for the linear happy path, but it loses information for non-linear workflows. A task workflow, for example, has branching paths (hold, blocked) that the stepper cannot represent.

**Tier 1 (immediate, within ArtifactViewer):**
- Keep `PipelineStepper` as the primary visualization. It works for the common case.
- Add a "Workflow" info section below the stepper that shows:
  - Current state name and description (from the workflow YAML)
  - Available transitions (the events that can fire from the current state)
  - Guard status for each transition (which guards pass, which fail)
  - Gate information (if the transition has a gate, show the gate pattern)

**Tier 2 (plugin view):**
- A dedicated "Workflow Browser" view contributed by the governance plugin.
- Shows all registered workflows with their state diagrams (rendered as Mermaid statecharts).
- Allows inspecting states, transitions, guards, gates for any workflow.
- This is a reference view, not day-to-day.

**Tier 3 (future):**
- A "Workflow Designer" plugin view that allows visual editing of workflow YAML files.
- Out of scope for near-term.

### Gate Review Presentation

The existing `GateQuestions.svelte` renders gate questions as blockquoted text. For the five-phase gate pipeline (gather, present, collect, execute, learn), the UI needs:

1. **Gate Panel** (appears when an artifact is at a gate transition):
   - Shows the gather phase results (pre-check pass/fail, collected data)
   - Presents the structured review content (from the "present" phase definition)
   - Offers verdict buttons (from the "collect" phase definition)
   - Requires rationale text if `require_rationale: true`
   - On submission, executes the transition and records the audit entry

2. **Gate indicator on PipelineStepper**:
   - Transitions that have gates should show a gate icon (e.g., `shield-check`) on the connector line between states.
   - Clicking the gate icon opens the Gate Panel.

3. **Gate status on dashboard DecisionQueueWidget**:
   - Items with pending gates should appear in the "Attention Required" section.
   - The action label should mention the gate pattern (e.g., "Review required: simple approval").

### Recommendation: Category-Based Status Colors

The research document (RES-d6e8ab11, Section 7) defines state categories with UI treatments:

| Category | Color | Use |
|----------|-------|-----|
| planning | Blue | Status badges, pipeline stepper past/current indicators |
| active | Green | Active work indicators |
| review | Amber | Review/gate indicators |
| completed | Purple | Completed indicators |
| terminal | Gray | Archived/surpassed indicators |

These should be applied consistently across the `PipelineStepper`, status badges in `FrontmatterHeader`, the `DecisionQueueWidget`, and graph visualization node colors.

---

## 6. Agent/Team Visibility

### Current State

The app has NO agent team visibility currently. There are no components for showing:
- Active agent teams
- Task progress within teams
- Token metrics
- Agent findings
- Orchestration activity

The conversation view shows AI messages and tool calls, but does not surface the team/task/agent structure.

### Finding: Two Audiences, Two Visibility Levels

The research document describes a hub-spoke model where the orchestrator delegates to ephemeral agents. The UI audience splits into:

1. **Governance practitioner** (primary user): Cares about what decisions are needed, what work is happening, what the outcomes are. Does NOT care about token counts or agent internals.

2. **Power user / developer** (secondary user): Cares about token efficiency, agent performance, cost attribution, debugging orchestration. DOES care about internals.

### Recommendation: Layered Visibility

**Layer 1: Action-Oriented (Default)**

Surface agent activity as *work happening*, not *agents executing*:

- **Conversation panel** already shows tool calls. Add a "Background Work" indicator when agents are running:
  - A subtle badge on the conversation panel header showing "3 tasks in progress"
  - Clicking expands to show task descriptions (not agent details)
  - When a task completes, the findings summary appears as a system message

- **Dashboard widget** for active work:
  - "Active Teams" section showing team name, task count, completed count
  - Each task shows: title, status (pending/running/complete), elapsed time
  - This is a core widget (agent teams are a core concept, not plugin-specific)

**Layer 2: Metrics (Opt-In)**

A dedicated "Session Metrics" view (core view, not plugin) accessible from the StatusBar or Settings:

- **Session overview**: total tokens, estimated cost, cache hit rate
- **Agent breakdown**: per-agent token usage, context utilization ratio
- **Efficiency analysis**: overhead ratio, knowledge injection cost
- **Budget enforcement**: current usage vs session budget (if configured)

This view reads from `.state/token-metrics.jsonl` (as specified in the research doc).

**Layer 3: Debug (Developer Tools)**

A "Developer Console" panel (already partially implemented as `initDevConsole()` in AppLayout):

- Full orchestration log: team create/delete, agent spawn/complete, task state changes
- Raw token metrics per API call
- Prompt preview: what was actually sent to the model
- This is behind a developer mode flag in settings

### Token Metrics on the StatusBar

The StatusBar should show a single token efficiency indicator:

```
[Session: 45K tokens | $0.23 | Cache: 87%]
```

Clicking opens the Session Metrics view. This gives the power user constant visibility without cluttering the governance practitioner's experience.

### Recommendation: Do NOT Show Agent Internals by Default

The governance practitioner mental model is:
- "I asked for a code review" (not "I spawned a reviewer agent with 2,800 token budget")
- "The review found 3 issues" (not "Agent-7f2b completed in 4.2s with 1,847 output tokens")

Agent/team details are implementation details of the orchestration system. Surface them only in the metrics/debug layers.

---

## 7. Plugin Management

### Current State

The `PluginBrowser` component in Settings provides:

- **Installed tab**: lists locally installed plugins from `plugins/` directory
- **Official tab**: fetches from a GitHub registry catalog
- **Community tab**: fetches from a community registry catalog
- **Detail view**: shows plugin manifest (schemas, views, widgets, relationships, tools, hooks)
- **Install flow**: download, extract, validate, resolve conflicts
- **Conflict resolution**: dialog for schema/relationship key collisions with merge/rename options

### Finding: Plugin Management is Well-Developed

The plugin browser is functional. What is missing:

1. **Plugin health/contribution visibility** -- no way to see at a glance what a plugin contributes to the project (how many artifact types, how many relationships, how many knowledge artifacts, how many workflows).

2. **Plugin dependency graph** -- no visualization of which plugins require which others.

3. **Plugin enable/disable** -- the `PluginProjectConfig` type has `enabled: boolean` but the UI does not expose toggle controls (only install/uninstall).

### Recommendation: Enhanced Plugin Detail View

In the "Installed" tab detail view, add a contribution summary card:

```
@orqastudio/plugin-agile-governance v0.1.4-dev
Role: core:governance

Contributes:
  14 artifact types    (vision, pillar, persona, idea, decision, ...)
  14 workflows         (vision, pillar, persona, idea, decision, ...)
  12 relationships     (upholds, aligns-with, governs, ...)
   5 agents            (cleanup-reviewer, content-migrator, ...)
   4 knowledge items   (composability, diagnostic-methodology, ...)
   2 enforcement mechanisms (behavioral, json-schema)
   0 views, 0 widgets

Dependencies: requires @orqastudio/plugin-core-framework
```

This gives the user immediate understanding of what each plugin provides.

### Recommendation: Plugin Toggle

Add an enable/disable toggle to each installed plugin card. When disabled:
- The plugin's schemas, relationships, views, and widgets are unregistered from the `PluginRegistry`
- Existing artifacts using the plugin's types remain on disk but show validation warnings
- The toggle state is persisted in `project.json` under `plugins.<name>.enabled`

This allows non-destructive experimentation with plugin combinations.

---

## 8. Concrete Recommendations Summary

### Near-Term (next 1-2 epics)

1. **Activate the new navigation model** for the dev project by generating a `navigation` array in project.json from the existing `artifacts` config plus plugin `defaultNavigation` contributions.

2. **Make the dashboard widget-composable** by reading from `pluginRegistry.allWidgets` and rendering with `getWidgetComponent()`. Move current hardcoded widgets to their owning plugins' manifests.

3. **Add category-based colors** to `PipelineStepper`, status badges, and graph nodes using the state category vocabulary (planning=blue, active=green, review=amber, completed=purple, terminal=gray).

4. **Add gate indicators** to `PipelineStepper` for transitions with gates. Show the gate pattern name and a clickable icon.

5. **Add a "Workflow Info" section** to `ArtifactViewer` below the stepper showing current state description, available transitions, and guard status.

### Medium-Term (next 3-4 epics)

6. **Build a Gate Review Panel** component that implements the gather-present-collect flow for human gates.

7. **Add "Active Work" widget** to the dashboard showing running agent teams and task progress.

8. **Add session token summary** to the StatusBar.

9. **Build a Session Metrics core view** accessible from StatusBar click or a nav item.

10. **Enhance the Plugin Browser** with contribution summary cards and enable/disable toggles.

### Long-Term (future roadmap)

11. **Workflow Browser plugin view** from the governance plugin showing all state machines as Mermaid diagrams.

12. **Command palette** (Ctrl+Shift+P) for executing actions across views.

13. **Custom artifact viewers** -- plugins can register custom viewer components for their artifact types.

14. **Workflow Designer** -- visual editor for workflow YAML files.

15. **Token efficiency dashboard** with trend analysis, cost attribution, and cache optimization recommendations.

---

## 9. Open Questions

1. **Should the agile-governance plugin declare views/widgets?** Currently it declares none, but it owns the governance learning loop (PipelineWidget), lesson velocity (LessonVelocityWidget), and improvement trends (ImprovementTrendsWidget). These should move to the plugin.

2. **Should there be a "Workflows" core view or plugin view?** Workflows are registered by plugins but evaluated by the core engine. A read-only workflow browser could be a core view that reads all registered workflows generically. A workflow editor would be a plugin view.

3. **How should the conversation panel interact with gate reviews?** When a gate review is triggered by an agent, should the gate review UI appear in the conversation panel (inline with the chat) or as a modal/side panel over the main explorer area?

4. **Should plugin views have access to the full SDK store registry?** Currently they do (via `globalThis.__orqa_stores`), which means plugin views can access all stores. This is powerful but has no permission boundary. Should plugin views declare required capabilities?

5. **What is the right dashboard layout for projects with minimal plugins?** A project with only the core framework plugin would have very few widgets. The dashboard should gracefully collapse to just the health/integrity core widgets without looking empty.
