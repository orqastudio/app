# Migration Tasks: Phases 9-11

Exhaustive, atomic task lists for every unit of work in Phases 9-11. Each task fits one agent context window with specific acceptance criteria and reviewer checks.

---

## Phase 9: Frontend Alignment

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every hardcoding fix and navigation change must be validated against ARCHITECTURE.md to confirm the schema-driven approach is correct.

### 9.1 HIGH Severity Hardcoding Fixes

#### TASK 9.1.1: Replace hardcoded artifact ID regex in MarkdownLink.svelte

**What:** `app/src/lib/components/content/MarkdownLink.svelte` line 16 has `ARTIFACT_ID_RE` with hardcoded prefixes `(EPIC | TASK | AD | MS | IDEA | IMPL | RES | PILLAR | RULE | DOC | KNOW | AGENT)`. Replace with a dynamic regex built from the composed schema's artifact type definitions (each type has an `id_prefix` field). The composed schema is available via the `projectStore` or `artifactGraphSDK` from `@orqastudio/sdk`.

Also fix the second instance: `app/src/lib/components/artifact/ArtifactViewer.svelte` line 221 has a generic `ARTIFACT_ID_RE = /^[A-Z]+-\d+$/` — this is acceptable as a loose pattern but should share the same source.

**Files:**

- `app/src/lib/components/content/MarkdownLink.svelte`
- `app/src/lib/components/artifact/ArtifactViewer.svelte`

**Acceptance Criteria:**

- [ ] `MarkdownLink.svelte` no longer contains hardcoded prefix list
- [ ] Artifact ID prefixes are derived from the composed schema (via SDK store or Tauri IPC)
- [ ] New artifact types added by plugins are automatically detected as artifact links
- [ ] Existing artifact link rendering behavior is preserved (visual regression: links still render as colored chips)
- [ ] `ArtifactViewer.svelte` uses a shared utility or the same dynamic source

**Reviewer Checks:**

- Grep for hardcoded artifact type prefixes in both files — none should remain
- Verify the composed schema query returns the correct prefix list
- Confirm that a hypothetical new prefix (e.g., `FEAT`) would be picked up without code changes

---

#### TASK 9.1.2: Replace hardcoded sidecar plugin name in StatusBar.svelte

**What:** `app/src/lib/components/layout/StatusBar.svelte` has `sidecarPluginName = "@orqastudio/plugin-claude"` hardcoded. Replace with the active sidecar key from the plugin registry (`pluginRegistry.activeSidecarKey` or equivalent from `@orqastudio/sdk`).

**Files:**

- `app/src/lib/components/layout/StatusBar.svelte`

**Acceptance Criteria:**

- [ ] No hardcoded sidecar plugin name string in `StatusBar.svelte`
- [ ] Sidecar plugin name is read from the plugin registry's active sidecar
- [ ] If no sidecar is active, status bar shows appropriate fallback (e.g., "No sidecar")
- [ ] If a different sidecar plugin is installed (e.g., OpenAI), the status bar tracks it correctly

**Reviewer Checks:**

- Grep for `@orqastudio/plugin-claude` in entire `app/src/` — should only appear in plugin-specific code, not in generic UI
- Verify the plugin registry exposes the active sidecar key

---

### 9.2 MEDIUM Severity Hardcoding Fixes

#### TASK 9.2.1: Deduplicate and externalize model options

**What:** The model list `[auto, claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5]` is duplicated in THREE files:

1. `app/src/lib/components/conversation/model-options.ts` — `CLAUDE_MODELS` array
2. `app/src/lib/components/settings/ModelSettings.svelte` — `modelOptions` array
3. `app/src/lib/components/settings/ProjectScanningSettings.svelte` — `modelOptions` array

Replace all three with a single source. The sidecar plugin should advertise available models. Until the sidecar plugin API supports this, consolidate into `model-options.ts` and import from there in both settings components.

**Files:**

- `app/src/lib/components/conversation/model-options.ts`
- `app/src/lib/components/settings/ModelSettings.svelte`
- `app/src/lib/components/settings/ProjectScanningSettings.svelte`
- `app/src/lib/components/conversation/ModelSelector.svelte` (verify import still works)

**Acceptance Criteria:**

- [ ] Model options defined in exactly ONE place (`model-options.ts`)
- [ ] `ModelSettings.svelte` imports from `model-options.ts` instead of defining its own array
- [ ] `ProjectScanningSettings.svelte` imports from `model-options.ts` instead of defining its own array
- [ ] `ModelSelector.svelte` still works (already imports from `model-options.ts`)
- [ ] All three UIs show the same models in the same order

**Reviewer Checks:**

- Grep for `claude-opus` or `claude-sonnet` across `app/src/` — should appear only in `model-options.ts`
- Verify all three components render correctly with the shared source

---

#### TASK 9.2.2: Make FrontmatterHeader field classification schema-driven

**What:** `app/src/lib/components/artifact/FrontmatterHeader.svelte` hardcodes:

- `SKIP_FIELDS` set — fields not rendered
- `CHIP_FIELDS` set — fields rendered as tag chips
- `LINK_FIELDS` set — fields rendered as artifact links
- `BOOLEAN_FIELDS` set — fields rendered as toggles
- `FIELD_ORDER` array — display order
- Priority color classes (P0-P3 mapped to specific Tailwind classes)

These should derive from the composed schema's artifact type definitions, which should include field rendering hints. Until the schema supports rendering hints, move these to a shared configuration module that can be replaced by schema data later.

**Files:**

- `app/src/lib/components/artifact/FrontmatterHeader.svelte`

**Acceptance Criteria:**

- [ ] Field classification constants are extracted to a configuration source (shared module or schema query)
- [ ] `FrontmatterHeader.svelte` imports field classification instead of defining inline
- [ ] Adding a new field classification for a plugin-defined artifact type does not require editing `FrontmatterHeader.svelte`
- [ ] Priority color classes are extracted to the same configuration source
- [ ] Existing frontmatter rendering is visually identical

**Reviewer Checks:**

- Verify `SKIP_FIELDS`, `CHIP_FIELDS`, `LINK_FIELDS`, `BOOLEAN_FIELDS`, `FIELD_ORDER` are no longer defined inline
- Verify existing artifacts render frontmatter correctly (priority badges, chips, links, booleans all display)

---

#### TASK 9.2.3: Remove fallback stages from ArtifactViewer.svelte

**What:** `app/src/lib/components/artifact/ArtifactViewer.svelte` has a fallback `stages` array `[{key:"draft",label:"Draft"}, {key:"in_progress",label:"In Progress"}, {key:"review",label:"Review"}, {key:"done",label:"Done"}]` used when project settings have no statuses. The project config should ALWAYS provide statuses — if none are configured, the pipeline stepper should not render rather than falling back to hardcoded stages.

**Files:**

- `app/src/lib/components/artifact/ArtifactViewer.svelte`

**Acceptance Criteria:**

- [ ] Hardcoded fallback stages array is removed
- [ ] When project settings have statuses: pipeline stepper renders from config (no change)
- [ ] When project settings have NO statuses: pipeline stepper is not rendered (no fallback)
- [ ] No console errors when statuses are missing

**Reviewer Checks:**

- Grep for `draft.*in_progress.*review.*done` pattern in the file — should be gone
- Verify the component handles missing statuses gracefully (hidden, not broken)

---

#### TASK 9.2.4: Make ArtifactLanding category config data-driven

**What:** `app/src/lib/components/artifact/ArtifactLanding.svelte` has `categoryConfig` record mapping category keys (`process`, `delivery`, `discovery`, `governance`, `principles`) to `{icon, label, description}`. This should come from the navigation tree config provided by plugin composition.

**Files:**

- `app/src/lib/components/artifact/ArtifactLanding.svelte`

**Acceptance Criteria:**

- [ ] `categoryConfig` is no longer hardcoded inline
- [ ] Category metadata (icon, label, description) comes from the navigation tree or plugin composition
- [ ] Plugin-contributed categories appear correctly
- [ ] Existing categories render with their current icons, labels, and descriptions

**Reviewer Checks:**

- Grep for `categoryConfig` — should be sourced from store/config, not inline definition
- Verify all existing category pages still render correctly

---

#### TASK 9.2.5: Make DynamicArtifactTable sort orders config-driven

**What:** `app/src/lib/components/content/DynamicArtifactTable.svelte` has:

- `PRIORITY_ORDER` map (P0=0, P1=1, P2=2, P3=3)
- `STATUS_ORDER` map (draft=0, identified=1, in_progress=2, active=3, review=4, done=5, blocked=6)

Priority order should come from the composed schema's priority definitions. Status order should come from the project's status machine (which already defines status ordering).

**Files:**

- `app/src/lib/components/content/DynamicArtifactTable.svelte`

**Acceptance Criteria:**

- [ ] `PRIORITY_ORDER` is derived from composed schema or project config
- [ ] `STATUS_ORDER` is derived from the project's status definitions (which define display order)
- [ ] Sorting behavior is preserved for all current statuses and priorities
- [ ] New statuses added by workflow plugins sort correctly without code changes

**Reviewer Checks:**

- Grep for `PRIORITY_ORDER` and `STATUS_ORDER` — should not be hardcoded maps
- Verify table sorting works correctly with current data

---

#### TASK 9.2.6: Make LessonVelocityWidget stages config-driven

**What:** `app/src/lib/components/dashboard/LessonVelocityWidget.svelte` has a hardcoded `stages` array with fixed stage keys, labels, and Tailwind color classes: `identified` (amber), `active` (blue), `promoted` (green), `resolved` (slate). These should come from the lesson lifecycle config provided by the learning workflow plugin.

**Files:**

- `app/src/lib/components/dashboard/LessonVelocityWidget.svelte`

**Acceptance Criteria:**

- [ ] Stage definitions (keys, labels, colors) are no longer hardcoded
- [ ] Stages come from the lesson lifecycle workflow plugin configuration
- [ ] If the learning plugin defines different stages, the widget adapts
- [ ] Current visual appearance is preserved when the standard stages are active

**Reviewer Checks:**

- Grep for `identified.*active.*promoted.*resolved` pattern — should be gone from inline code
- Verify the widget still renders with correct colors and counts

---

#### TASK 9.2.7: Make ImprovementTrendsWidget governance types schema-driven

**What:** `app/src/lib/components/dashboard/ImprovementTrendsWidget.svelte` queries specific governance artifact types: `artifactGraphSDK.byType("rule")`, `.byType("lesson")`, `.byType("decision")`. These types should come from the composed schema's definition of which artifact types are "governance" types.

**Files:**

- `app/src/lib/components/dashboard/ImprovementTrendsWidget.svelte`

**Acceptance Criteria:**

- [ ] Governance artifact types are not hardcoded strings
- [ ] Types are derived from the composed schema (e.g., types with category "governance")
- [ ] A plugin adding a new governance artifact type would be included automatically
- [ ] Current trend lines are preserved for existing governance types

**Reviewer Checks:**

- Grep for `.byType("rule")`, `.byType("lesson")`, `.byType("decision")` — should use dynamic type list
- Verify the chart renders correctly with current data

---

#### TASK 9.2.8: Make DecisionQueueWidget action labels data-driven

**What:** `app/src/lib/components/dashboard/DecisionQueueWidget.svelte` has an `actionLabel` function that maps artifact type to action label (decision -> "Decide", task -> "Assign", etc.). This should come from the workflow plugin that defines action labels per artifact type per status.

**Files:**

- `app/src/lib/components/dashboard/DecisionQueueWidget.svelte`

**Acceptance Criteria:**

- [ ] `actionLabel` function does not contain hardcoded type-to-label mapping
- [ ] Action labels come from workflow plugin configuration or artifact type schema
- [ ] Fallback to a generic label (e.g., "Review") for types without configured action labels
- [ ] Current button labels are preserved for existing types

**Reviewer Checks:**

- Grep for `"Decide"`, `"Assign"` in the file — should not be inline constants
- Verify the decision queue still shows correct action buttons

---

#### TASK 9.2.9: Make EmbeddingModelStep model name config-driven

**What:** `app/src/lib/components/setup/EmbeddingModelStep.svelte` displays the hardcoded model name "all-MiniLM-L6-v2". The search engine should advertise its model name via config or IPC.

**Files:**

- `app/src/lib/components/setup/EmbeddingModelStep.svelte`

**Acceptance Criteria:**

- [ ] Model name string is not hardcoded in the component
- [ ] Model name comes from search engine config or daemon status
- [ ] If the embedding model changes (e.g., to BGE-small-en-v1.5), the display updates automatically

**Reviewer Checks:**

- Grep for `MiniLM` in the file — should be gone
- Verify setup wizard still shows the correct model name

---

### 9.3 Navigation Restructure

#### TASK 9.3.1: Implement methodology-stage-based main navigation

**What:** Replace the current hardcoded/legacy activity bar navigation with methodology-stage-driven navigation. The main nav items should be:

1. **Dashboard** (top-level landing)
2. **One item per methodology stage** (Discovery, Planning, Documentation, Implementation, Review, Learning) — derived from the methodology plugin and its stage plugins
3. **Plugins** (above Settings)
4. **Settings** (bottom)

The navigation structure must be generated from the methodology plugin and its stage plugins, not hardcoded.

**Files:**

- `app/src/lib/components/layout/ActivityBar.svelte`
- `app/src/lib/components/layout/AppLayout.svelte` (may need routing updates)
- `app/src/lib/components/layout/NavSubPanel.svelte` (sub-category routing)

**Acceptance Criteria:**

- [ ] Activity bar shows Dashboard + methodology stages + Plugins + Settings
- [ ] Methodology stages come from the installed methodology plugin, not hardcoded
- [ ] If no methodology plugin is installed, only Dashboard + Plugins + Settings show
- [ ] Each methodology stage shows its artifacts when clicked (via sub-nav panel)
- [ ] Existing navigation still works for all artifact categories

**Reviewer Checks:**

- Verify the activity bar renders methodology stages from plugin data
- Grep for hardcoded stage names in `ActivityBar.svelte` — none should exist
- Test navigation flow: click stage -> see artifacts for that stage

---

#### TASK 9.3.2: Add Plugins top-level navigation item

**What:** Add a "Plugins" entry to the activity bar, positioned above Settings. This surfaces the plugin browser as a top-level navigation destination rather than a settings sub-page.

**Files:**

- `app/src/lib/components/layout/ActivityBar.svelte`
- `app/src/lib/components/layout/ExplorerRouter.svelte` (add routing for plugin view)
- `app/src/lib/components/layout/NavSubPanel.svelte` (plugin sub-nav if needed)

**Acceptance Criteria:**

- [ ] "Plugins" icon appears in activity bar above Settings
- [ ] Clicking "Plugins" shows the plugin browser (installed/official/community tabs)
- [ ] Plugin browser in settings still works (can be reached from both locations)
- [ ] Plugin browser shows category filters (knowledge, methodology, workflow, sidecar, connector, infrastructure)

**Reviewer Checks:**

- Verify Plugins nav item is present and positioned correctly
- Verify click opens the plugin browser view
- Verify plugin browser is still accessible from Settings

---

#### TASK 9.3.3: Implement plugin group bundling in plugin browser

**What:** Surface **plugin groups** in the plugin browser that bundle a methodology + all its stage plugins together (e.g., "Agile Software Development" installs methodology + all workflow stages). The `core` plugin is NOT surfaced to users.

**Files:**

- `app/src/lib/components/settings/PluginBrowser.svelte`

**Acceptance Criteria:**

- [ ] Plugin browser has a "Groups" or "Bundles" view/tab showing plugin groups
- [ ] Each group shows which plugins it includes (methodology + workflow plugins)
- [ ] Installing a group installs all contained plugins
- [ ] The `core` plugin is hidden from the plugin browser (filtered out)
- [ ] Individual plugins within a group can still be browsed/installed separately

**Reviewer Checks:**

- Verify the core plugin is not visible in any plugin browser tab
- Verify plugin groups display their constituent plugins
- Verify group install triggers installation of all contained plugins

---

### 9.4 Settings Reorganization

#### TASK 9.4.1: Reorganize settings navigation to reflect architecture

**What:** `app/src/lib/components/navigation/SettingsCategoryNav.svelte` has hardcoded `appCategories` and `projectCategories` arrays. Reorganize to reflect the architecture:

**Project Settings sections:**

- **Methodology** — dedicated section for installed methodology plugin. Workflow plugins nested underneath.
- **Sidecar** — dedicated section for installed sidecar(s)
- **Connector** — dedicated section for installed connector(s)
- **Plugins** — generic section for all other installed plugins (knowledge, infrastructure, etc.)
- Settings pages generated by plugins appear in their appropriate section AND are reachable from the plugin list

**App Settings:** Provider, Model, Appearance, Shortcuts remain (these are app-level, not plugin-driven).

**Remove:** The "Navigation" settings page (navigation is now plugin-driven).

**Files:**

- `app/src/lib/components/navigation/SettingsCategoryNav.svelte`
- `app/src/lib/components/settings/SettingsView.svelte` (add routing for new sections)
- `app/src/lib/components/settings/NavigationSettings.svelte` (remove or repurpose)

**Acceptance Criteria:**

- [ ] Project settings sidebar shows: Methodology, Sidecar, Connector, Plugins sections (driven by installed plugins)
- [ ] Each section shows the appropriate installed plugin(s) with their settings pages
- [ ] App settings sidebar retains: Provider, Model, Appearance, Shortcuts
- [ ] Navigation settings page is removed
- [ ] Plugin-generated settings pages render in the correct section
- [ ] Settings categories adapt when plugins are installed/uninstalled

**Reviewer Checks:**

- Grep for hardcoded `appCategories` and `projectCategories` in `SettingsCategoryNav.svelte` — app categories may remain static, but project categories must be plugin-driven
- Verify NavigationSettings.svelte is removed or repurposed
- Verify each plugin type appears in its correct settings section

---

### 9.5 LOW Severity Hardcoding (Task per item)

These are lower priority but should still be addressed for architectural consistency.

#### TASK 9.5.1: Extract TraceabilityPanel icon mapping to config

**What:** `app/src/lib/components/artifact/TraceabilityPanel.svelte` has `iconForType` mapping (epic->flag, task->check-square, milestone->target, etc.). Extract to a shared configuration that can be extended by plugins defining new artifact types.

**Files:**

- `app/src/lib/components/artifact/TraceabilityPanel.svelte`

**Acceptance Criteria:**

- [ ] `iconForType` is not defined inline
- [ ] Icon mapping comes from a shared config or the artifact type schema
- [ ] Plugin-defined artifact types can specify their own icons
- [ ] Existing icons are preserved for current types

**Reviewer Checks:**

- Verify `iconForType` is imported, not defined inline
- Verify all existing artifact types still show correct icons

---

#### TASK 9.5.2: Extract category colors to config

**What:** `app/src/lib/utils/category-colors.ts` has `CATEGORY_COLORS` mapping lesson categories (process, technical, team, governance, delivery, general) to Tailwind color classes. Extract to a configurable source.

**Files:**

- `app/src/lib/utils/category-colors.ts`
- `app/src/lib/components/lessons/LessonList.svelte` (consumer)
- `app/src/lib/components/lessons/LessonViewer.svelte` (consumer)

**Acceptance Criteria:**

- [ ] Color mappings can be extended by plugins defining new lesson categories
- [ ] Existing lesson categories retain their current colors
- [ ] A fallback color exists for unknown categories

**Reviewer Checks:**

- Verify existing lesson views render with correct colors
- Verify unknown categories get a fallback color

---

#### TASK 9.5.3: Extract tool display config

**What:** `app/src/lib/utils/tool-display.ts` has `TOOL_ICONS`, `TOOL_LABELS`, and `CAPABILITY_LABELS` maps. These are used by `ToolApprovalDialog.svelte`, `ToolCallCard.svelte`, `ToolCallGroup.svelte`, `ToolCallSummary.svelte`. Extract to a configurable source that can be extended when new tools are registered.

**Files:**

- `app/src/lib/utils/tool-display.ts`

**Acceptance Criteria:**

- [ ] Tool display config can be extended by plugins registering new tools
- [ ] Existing tool display labels and icons are preserved
- [ ] Unknown tools get a sensible default display

**Reviewer Checks:**

- Verify tool call cards render correctly for all existing tools
- Verify unknown tools show a default icon and label

---

#### TASK 9.5.4: Make ExplorerRouter core views extensible

**What:** `app/src/lib/components/layout/ExplorerRouter.svelte` has a `CORE_VIEWS` record mapping view keys to components. While core views are legitimate, the pattern should be extensible for plugin-contributed views.

**Files:**

- `app/src/lib/components/layout/ExplorerRouter.svelte`

**Acceptance Criteria:**

- [ ] Core views (Dashboard, Graph, Welcome) remain functional
- [ ] Plugin views are routed via the existing `PluginViewContainer` mechanism
- [ ] The router can handle both core and plugin views without hardcoded assumptions

**Reviewer Checks:**

- Verify core views still render
- Verify plugin views still load via `PluginViewContainer`

---

#### TASK 9.5.5: Review and clean ToolCallCard enforcement regex

**What:** `app/src/lib/components/tool/ToolCallCard.svelte` has a regex pattern to detect enforcement block messages (`"Rule 'name' blocked..."`) in tool output. This pattern matches a known engine output format. Verify it matches the current engine output format and document the coupling.

**Files:**

- `app/src/lib/components/tool/ToolCallCard.svelte`

**Acceptance Criteria:**

- [ ] Regex matches the actual enforcement engine output format
- [ ] If the format has changed, update the regex
- [ ] Add a code comment documenting the coupling to the engine output format

**Reviewer Checks:**

- Compare the regex against actual enforcement block messages from the engine
- Verify enforcement block detection works in the conversation view

---

### 9.6 Custom Views

#### TASK 9.6.1: Verify plugin custom views render in new navigation

**What:** Ensure that custom views contributed by plugins (loaded via `PluginViewContainer.svelte`) render correctly in the new methodology-stage-based navigation structure. The view routing must work with both the old and new navigation approaches during transition.

**Files:**

- `app/src/lib/components/plugin/PluginViewContainer.svelte`
- `app/src/lib/components/layout/ExplorerRouter.svelte`

**Acceptance Criteria:**

- [ ] Plugin views registered in plugin manifests are accessible via navigation
- [ ] Views load correctly via `convertFileSrc` + dynamic import
- [ ] View cleanup works on navigation away (no memory leaks)
- [ ] Navigation to plugin views works from both activity bar and settings plugin list

**Reviewer Checks:**

- Install a plugin with a custom view and verify it renders
- Navigate away and back — verify cleanup and re-mount

---

#### TASK 9.6.2: Validate roadmap view works with milestone/epic hierarchy

**What:** ARCHITECTURE.md Phase 9 explicitly says: "Review the roadmap view to ensure it works with the milestone/epic hierarchy." Verify that the roadmap view correctly displays milestone -> epic relationships, respects status transitions, and handles the updated artifact types from the composed schema.

**Files:**

- `app/src/lib/components/views/` (roadmap view component)
- `app/src/lib/stores/` (relevant data stores)

**Acceptance Criteria:**

- [ ] Roadmap view renders milestones with their child epics
- [ ] Milestone/epic hierarchy uses graph relationships (not hardcoded structure)
- [ ] Status colors and transitions are correct per composed schema
- [ ] View works with the new methodology-stage-based navigation
- [ ] Empty states handled (no milestones, no epics under a milestone)

**Reviewer Checks:**

- Create test data with milestones and epics — verify hierarchical display
- Verify the view reads relationship data from the graph, not hardcoded lists
- Verify status rendering matches the workflow definitions

---

### 9.7 Frontend Build Verification

#### TASK 9.7.1: Frontend build and type-check after all Phase 9 changes

**What:** After all Phase 9 tasks are complete, run full frontend verification to ensure no regressions.

**Files:** All modified frontend files

**Acceptance Criteria:**

- [ ] `npx svelte-check` passes with zero errors
- [ ] `npx tsc --noEmit` passes
- [ ] `npm run build` succeeds in `app/`
- [ ] No new TypeScript errors introduced
- [ ] No new Svelte compiler warnings

**Reviewer Checks:**

- Run `svelte-check` independently and verify zero errors
- Verify the build output is valid

---

## Phase 10: Validate Against Targets

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every validation must compare generated output against hand-written targets per ARCHITECTURE.md Phase 10.

### 10.1 Target Schema Validation

#### TASK 10.1.1: Validate composed schema generation against target

**What:** Run the schema composition pipeline (from installed plugins) and compare the generated `schema.composed.json` against `targets/schema.composed.json`. The generated output must match or exceed the hand-written target.

**Files:**

- `targets/schema.composed.json` (target)
- Generated `schema.composed.json` (output of composition pipeline)

**Acceptance Criteria:**

- [ ] Schema composition pipeline runs without errors
- [ ] Generated schema contains ALL artifact types from target
- [ ] Generated schema contains ALL relationship types from target
- [ ] Generated schema contains ALL status definitions from target
- [ ] Field-by-field diff shows no meaningful gaps (ordering differences are acceptable)
- [ ] If match: replace `targets/schema.composed.json` with generated version

**Reviewer Checks:**

- Run a diff between generated and target schemas
- Verify all artifact type `id_prefix` values match
- Verify all relationship `from`/`to` constraints match

---

### 10.2 Claude Code Plugin Validation

#### TASK 10.2.1: Validate generated CLAUDE.md against target

**What:** Run the connector generation pipeline and compare generated `.claude/CLAUDE.md` against `targets/claude-code-plugin/.claude/CLAUDE.md`. The generated orchestrator prompt must contain all sections present in the target.

**Files:**

- `targets/claude-code-plugin/.claude/CLAUDE.md` (target)
- Generated `.claude/CLAUDE.md` (connector output)

**Acceptance Criteria:**

- [ ] Generated CLAUDE.md contains: project description, design principles table, team discipline section, agent delegation table, role-based tool constraints table, completion gate, autonomous execution, key design decisions, git workflow, architecture reference, session protocol
- [ ] Section ordering matches target
- [ ] Content accuracy verified against ARCHITECTURE.md
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- Section-by-section comparison against target
- Verify no stale or inaccurate content in generated version

---

#### TASK 10.2.2: Validate generated agent definitions against targets

**What:** Run the connector generation pipeline and compare each generated agent file against its target:

- `targets/claude-code-plugin/.claude/agents/designer.md`
- `targets/claude-code-plugin/.claude/agents/governance-steward.md`
- `targets/claude-code-plugin/.claude/agents/implementer.md`
- `targets/claude-code-plugin/.claude/agents/orchestrator.md`
- `targets/claude-code-plugin/.claude/agents/planner.md`
- `targets/claude-code-plugin/.claude/agents/researcher.md`
- `targets/claude-code-plugin/.claude/agents/reviewer.md`
- `targets/claude-code-plugin/.claude/agents/writer.md`

**Files:**

- 8 target agent files in `targets/claude-code-plugin/.claude/agents/`
- 8 generated agent files from connector output

**Acceptance Criteria:**

- [ ] All 8 agent files are generated (one per base role)
- [ ] Each generated agent has: role description, boundaries, tool access, knowledge references, completion standard
- [ ] Role-based tool constraints match the target (Implementer can edit+shell, Reviewer read-only, etc.)
- [ ] Knowledge references are correct for each role
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Per-agent diff against target
- Verify tool constraint tables match ARCHITECTURE.md Section 6
- Verify no agent has permissions exceeding its role

---

#### TASK 10.2.3: Validate generated architecture split files against targets

**What:** Run the connector generation pipeline and compare each generated architecture file against its target. There are 12 files:

1. `targets/claude-code-plugin/.claude/architecture/agents.md`
2. `targets/claude-code-plugin/.claude/architecture/audit.md`
3. `targets/claude-code-plugin/.claude/architecture/connector.md`
4. `targets/claude-code-plugin/.claude/architecture/core.md`
5. `targets/claude-code-plugin/.claude/architecture/decisions.md`
6. `targets/claude-code-plugin/.claude/architecture/enforcement.md`
7. `targets/claude-code-plugin/.claude/architecture/glossary.md`
8. `targets/claude-code-plugin/.claude/architecture/governance.md`
9. `targets/claude-code-plugin/.claude/architecture/migration.md`
10. `targets/claude-code-plugin/.claude/architecture/plugins.md`
11. `targets/claude-code-plugin/.claude/architecture/structure.md`
12. `targets/claude-code-plugin/.claude/architecture/targets.md`

**Files:**

- 12 target architecture files
- 12 generated architecture files from connector output

**Acceptance Criteria:**

- [ ] All 12 architecture files are generated
- [ ] Each file contains the relevant section(s) from ARCHITECTURE.md
- [ ] Content matches the current ARCHITECTURE.md (not stale)
- [ ] Cross-references between files are correct
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Per-file diff against target
- Verify content matches the current ARCHITECTURE.md sections
- Verify the reference back to ARCHITECTURE.md in each file header

---

#### TASK 10.2.4: Validate generated settings.json against target

**What:** Compare generated `.claude/settings.json` against `targets/claude-code-plugin/.claude/settings.json`.

**Files:**

- `targets/claude-code-plugin/.claude/settings.json` (target)
- Generated `.claude/settings.json` (connector output)

**Acceptance Criteria:**

- [ ] Generated settings.json has correct permission configuration
- [ ] All tool permissions match the target
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- JSON diff between generated and target
- Verify permission settings are correct and not overly permissive

---

#### TASK 10.2.5: Validate generated plugin.json against target

**What:** Compare generated `.claude-plugin/plugin.json` against `targets/claude-code-plugin/.claude-plugin/plugin.json`.

**Files:**

- `targets/claude-code-plugin/.claude-plugin/plugin.json` (target)
- Generated `.claude-plugin/plugin.json` (connector output)

**Acceptance Criteria:**

- [ ] Generated plugin.json has correct name, version, description
- [ ] Hook declarations match the target
- [ ] Skill declarations match the target
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- JSON diff between generated and target
- Verify all hooks are declared
- Verify all skills are declared

---

#### TASK 10.2.6: Validate generated hooks.json against target

**What:** Compare generated `.claude-plugin/hooks/hooks.json` against `targets/claude-code-plugin/.claude-plugin/hooks/hooks.json`.

**Files:**

- `targets/claude-code-plugin/.claude-plugin/hooks/hooks.json` (target)
- Generated `.claude-plugin/hooks/hooks.json` (connector output)

**Acceptance Criteria:**

- [ ] All hook events are present (pre-tool-use, post-tool-use, session-start, pre-compact, stop, subagent-stop, task-completed, teammate-idle, user-prompt-submit)
- [ ] Hook script paths are correct
- [ ] Matchers and filters match the target
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- JSON diff between generated and target
- Verify every hook event has the correct script path
- Verify matcher configurations are correct

---

#### TASK 10.2.7: Validate generated hook scripts against targets

**What:** Compare each generated hook script against its target. There are 9 scripts:

1. `targets/claude-code-plugin/.claude-plugin/scripts/post-tool-use.mjs`
2. `targets/claude-code-plugin/.claude-plugin/scripts/pre-compact.mjs`
3. `targets/claude-code-plugin/.claude-plugin/scripts/pre-tool-use.mjs`
4. `targets/claude-code-plugin/.claude-plugin/scripts/session-start.mjs`
5. `targets/claude-code-plugin/.claude-plugin/scripts/stop.mjs`
6. `targets/claude-code-plugin/.claude-plugin/scripts/subagent-stop.mjs`
7. `targets/claude-code-plugin/.claude-plugin/scripts/task-completed.mjs`
8. `targets/claude-code-plugin/.claude-plugin/scripts/teammate-idle.mjs`
9. `targets/claude-code-plugin/.claude-plugin/scripts/user-prompt-submit.mjs`

**Files:**

- 9 target script files
- 9 generated script files from connector output

**Acceptance Criteria:**

- [ ] All 9 scripts are generated
- [ ] Each script's logic matches or exceeds the target (enforcement calls, context injection, etc.)
- [ ] Scripts use correct IPC mechanisms (daemon calls, CLI calls)
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Per-script diff against target
- Verify enforcement integration points are correct
- Verify error handling in each script

---

#### TASK 10.2.8: Validate generated skill definitions against targets

**What:** Compare each generated skill SKILL.md against its target. There are 4 skills:

1. `targets/claude-code-plugin/.claude-plugin/skills/orqa/SKILL.md`
2. `targets/claude-code-plugin/.claude-plugin/skills/orqa-create/SKILL.md`
3. `targets/claude-code-plugin/.claude-plugin/skills/orqa-save/SKILL.md`
4. `targets/claude-code-plugin/.claude-plugin/skills/orqa-validate/SKILL.md`

**Files:**

- 4 target skill files
- 4 generated skill files from connector output

**Acceptance Criteria:**

- [ ] All 4 skills are generated
- [ ] Each skill has correct trigger pattern, description, and instructions
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Per-skill diff against target
- Verify trigger patterns work in Claude Code

---

### 10.2b Claude Code Migration Target Validation

#### TASK 10.2.9: Assess `targets/claude-code-migration/` artifacts

**What:** The `targets/claude-code-migration/` directory contains 22 migration-specific target files:

- 8 agent files (`.claude/agents/*.md`)
- 12 architecture files (`.claude/architecture/*.md`)
- 1 CLAUDE.md (`.claude/CLAUDE.md`)
- 1 settings.json (`.claude/settings.json`)

Determine whether these are:
(a) Generation targets that should be validated against the connector pipeline (like `claude-code-plugin`), or
(b) Migration-specific artifacts used only during the migration process that should be explicitly removed in 10.4.1

If (a): validate each file against generated output in tasks 10.2.10-10.2.12.
If (b): document that these are migration-specific artifacts, not generation targets, and ensure 10.4.1 explicitly acknowledges their removal.

**Files:**

- `targets/claude-code-migration/` (22 files)

**Acceptance Criteria:**

- [ ] Classification documented: generation target or migration artifact
- [ ] If generation target: subsequent validation tasks created/executed
- [ ] If migration artifact: explicit acknowledgment in task 10.4.1 that these are NOT generation targets and are removed as migration artifacts
- [ ] No files silently deleted without classification

**Reviewer Checks:**

- Verify classification rationale is sound
- Verify no files are unaccounted for

---

#### TASK 10.2.10: Validate migration agent definitions (if generation targets)

**What:** If task 10.2.9 classifies `targets/claude-code-migration/` as generation targets, validate each agent file against the connector output. Otherwise, this task is skipped.

**Files:**

- 8 agent files in `targets/claude-code-migration/.claude/agents/`
- Corresponding generated agent files

**Acceptance Criteria:**

- [ ] If applicable: each agent file validated against generated output
- [ ] If skipped: documented as migration-only artifacts per 10.2.9

**Reviewer Checks:**

- Verify skip/execute decision matches 10.2.9 classification

---

#### TASK 10.2.11: Validate migration architecture files (if generation targets)

**What:** If task 10.2.9 classifies them as generation targets, validate the 12 architecture files against generated output. Otherwise, skip.

**Files:**

- 12 architecture files in `targets/claude-code-migration/.claude/architecture/`

**Acceptance Criteria:**

- [ ] If applicable: each architecture file validated
- [ ] If skipped: documented as migration-only per 10.2.9

---

#### TASK 10.2.12: Validate migration CLAUDE.md and settings.json (if generation targets)

**What:** If task 10.2.9 classifies them as generation targets, validate CLAUDE.md and settings.json against generated output. Otherwise, skip.

**Files:**

- `targets/claude-code-migration/.claude/CLAUDE.md`
- `targets/claude-code-migration/.claude/settings.json`

**Acceptance Criteria:**

- [ ] If applicable: both files validated
- [ ] If skipped: documented as migration-only per 10.2.9

---

### 10.3 Enforcement Config Validation

#### TASK 10.3.1: Validate generated ESLint config against target

**What:** Compare generated `eslint.config.js` against `targets/enforcement/eslint/eslint.config.js`.

**Files:**

- `targets/enforcement/eslint/eslint.config.js` (target)
- Generated `app/eslint.config.js` (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] Generated config includes all rules from target
- [ ] Plugin configuration matches
- [ ] Ignore patterns match
- [ ] Running `eslint` with generated config produces same results as target
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- Diff configs side by side
- Run linting with both configs and compare results

---

#### TASK 10.3.2: Validate generated Clippy config against target

**What:** Compare generated clippy configuration against targets:

- `targets/enforcement/clippy/clippy.toml`
- `targets/enforcement/clippy/workspace-lints.toml`

**Files:**

- 2 target clippy files
- Generated clippy config (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] Generated `clippy.toml` matches target lint settings
- [ ] Generated workspace lints match target
- [ ] Running `cargo clippy` with generated config produces same results as target
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Diff configs side by side
- Run clippy with both configs and compare results

---

#### TASK 10.3.3: Validate generated markdownlint config against target

**What:** Compare generated `.markdownlint.json` against `targets/enforcement/markdownlint/.markdownlint.json`.

**Files:**

- `targets/enforcement/markdownlint/.markdownlint.json` (target)
- Generated `.markdownlint.json` (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] All rule configurations match
- [ ] If match: target is replaced with generated version

**Reviewer Checks:**

- JSON diff between generated and target

---

#### TASK 10.3.4: Validate generated Prettier config against target

**What:** Compare generated prettier configs against targets:

- `targets/enforcement/prettier/.prettierrc`
- `targets/enforcement/prettier/.prettierignore`

**Files:**

- 2 target prettier files
- Generated prettier config (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] Generated `.prettierrc` matches target formatting rules
- [ ] Generated `.prettierignore` matches target ignore patterns
- [ ] Running `prettier --check` with generated config produces same results as target
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Diff configs side by side
- Run prettier check with both configs and compare results

---

#### TASK 10.3.5: Validate generated git hooks against targets

**What:** Compare generated git hooks against targets:

- `targets/enforcement/githooks/pre-commit`
- `targets/enforcement/githooks/post-commit`

**Files:**

- 2 target githook files
- Generated git hooks (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] Generated `pre-commit` hook runs all enforcement checks from target
- [ ] Generated `post-commit` hook performs all target actions
- [ ] Hook scripts are executable
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- Diff hooks side by side
- Verify enforcement check coverage matches

---

#### TASK 10.3.6: Validate generated tsconfig files against targets

**What:** Compare generated TypeScript configs against targets:

- `targets/enforcement/tsconfig/base.json`
- `targets/enforcement/tsconfig/app.json`
- `targets/enforcement/tsconfig/library.json`

**Files:**

- 3 target tsconfig files
- Generated tsconfig files (enforcement pipeline output)

**Acceptance Criteria:**

- [ ] Generated configs have all compiler options from targets
- [ ] Path mappings match
- [ ] Include/exclude patterns match
- [ ] If match: targets are replaced with generated versions

**Reviewer Checks:**

- JSON diff between generated and target configs
- Verify TypeScript compilation works with generated configs

---

### 10.4 Final Validation

#### TASK 10.4.1: Remove targets/ directory after all validation passes

**What:** Once ALL target validations in Tasks 10.1.1 through 10.3.6 are complete and every target has been replaced by a generated version, delete the `targets/` directory entirely.

**Files:**

- `targets/` (entire directory)

**Acceptance Criteria:**

- [ ] Every task in Phase 10 (10.1.1 through 10.3.6) is marked DONE
- [ ] Every target file has been validated and replaced by generated output
- [ ] `targets/` directory is deleted
- [ ] No references to `targets/` remain in ARCHITECTURE.md, CLAUDE.md, or other configs
- [ ] Generation pipelines produce correct output without target files as input

**Reviewer Checks:**

- Verify `targets/` directory does not exist
- Grep for `targets/` in all configuration files — should be removed or updated
- Run generation pipelines and verify output is correct

---

#### TASK 10.4.2: Run full completion test

**What:** Execute the completion test from ARCHITECTURE.md Section 13:

1. Every target from Phase 1 is produced by a generation pipeline
2. The same methodology and principles apply whether working via the app or via Claude Code
3. All enforcement is mechanical (generated hooks, linting, validation, permissions)
4. The `.orqa/` directory looks like something the finished app would have created
5. Agents work without bypass permissions, scoped to their role

**Files:** Entire project

**Acceptance Criteria:**

- [ ] All generation pipelines produce valid output
- [ ] Connector generates a complete `.claude/` directory
- [ ] Enforcement generates all linting/hook configs
- [ ] `.orqa/` directory passes artifact schema validation
- [ ] Agent role constraints are enforced (tool access restrictions work)
- [ ] No manual intervention required for any generation pipeline

**Reviewer Checks:**

- Run every generation pipeline from scratch
- Verify `.orqa/` passes the validation script
- Test an agent with restricted permissions to verify constraints hold

---

## Phase 11: Post-Migration Documentation

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Every DOC/KNOW conversion must be validated against ARCHITECTURE.md to confirm artifact structure, injection metadata, and relationship integrity.

### 11.1 Architecture File Conversion: Documentation Artifacts

Each architecture split file becomes a DOC artifact in `.orqa/documentation/architecture/`.

#### TASK 11.1.1: Convert core.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/core.md` content into a proper `.orqa/documentation/architecture/` DOC artifact with correct frontmatter (id, type: doc, title, description, relationships).

Content covers: What OrqaStudio is, product pillars, core product principles, design principles (P1-P7), engine libraries, language boundary, access layers (app, CLI, daemon, MCP, LSP).

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/core.md`
- Output: `.orqa/documentation/architecture/core-architecture.md`

**Acceptance Criteria:**

- [ ] DOC artifact created with valid frontmatter (id: DOC-xxx, type: doc, title, description, status)
- [ ] Content accurately reflects ARCHITECTURE.md Sections 1-3
- [ ] Relationships link to related decision artifacts and knowledge artifacts
- [ ] Passes artifact schema validation

**Reviewer Checks:**

- Verify frontmatter validates against composed schema
- Verify content matches current ARCHITECTURE.md (not stale)

---

#### TASK 11.1.2: Convert plugins.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/plugins.md` into `.orqa/documentation/architecture/plugin-architecture.md`.

Content covers: Plugin purposes, taxonomy, composition, schema generation, content installation, manifest format.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/plugins.md`
- Output: `.orqa/documentation/architecture/plugin-architecture.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Section 4
- [ ] Relationships to core architecture doc and plugin knowledge artifacts

**Reviewer Checks:**

- Frontmatter validation
- Content accuracy against ARCHITECTURE.md

---

#### TASK 11.1.3: Convert governance.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/governance.md` into `.orqa/documentation/architecture/governance-artifacts.md`.

Content covers: `.orqa/` directory structure, artifact lifecycle, target structure, governance areas.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/governance.md`
- Output: `.orqa/documentation/architecture/governance-artifacts.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Section 5
- [ ] Relationships to core architecture doc

**Reviewer Checks:**

- Frontmatter validation
- Content accuracy against ARCHITECTURE.md

---

#### TASK 11.1.4: Convert agents.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/agents.md` into `.orqa/documentation/architecture/agent-architecture.md`.

Content covers: Base roles, prompt generation pipeline (plugin registry -> schema assembly -> section resolution -> token budgeting -> prompt output), knowledge injection tiers, agent lifecycle.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/agents.md`
- Output: `.orqa/documentation/architecture/agent-architecture.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Section 6
- [ ] Relationships to core and plugin architecture docs

**Reviewer Checks:**

- Frontmatter validation
- Content accuracy against ARCHITECTURE.md

---

#### TASK 11.1.5: Convert connector.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/connector.md` into `.orqa/documentation/architecture/connector-architecture.md`.

Content covers: What a connector is, generation pipeline, watcher, generated plugin structure, runtime flow.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/connector.md`
- Output: `.orqa/documentation/architecture/connector-architecture.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Section 8
- [ ] Relationships to plugin and core architecture docs

**Reviewer Checks:**

- Frontmatter validation
- Content accuracy against ARCHITECTURE.md

---

#### TASK 11.1.6: Convert enforcement.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/enforcement.md` into `.orqa/documentation/architecture/enforcement-design.md`.

Content covers: State machine design, enforcement tooling, validation timing, enforcement layers.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/enforcement.md`
- Output: `.orqa/documentation/architecture/enforcement-design.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Sections 9-10
- [ ] Relationships to core architecture doc

**Reviewer Checks:**

- Frontmatter validation
- Content accuracy against ARCHITECTURE.md

---

#### TASK 11.1.7: Convert decisions.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/decisions.md` into `.orqa/documentation/architecture/key-decisions.md`.

Content covers: All key architectural decisions table with resolutions and references.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/decisions.md`
- Output: `.orqa/documentation/architecture/key-decisions.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content accurately reflects ARCHITECTURE.md Section 11
- [ ] Each decision references its source document

**Reviewer Checks:**

- Frontmatter validation
- Verify every decision in the table is still accurate

---

#### TASK 11.1.8: Convert structure.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/structure.md` into `.orqa/documentation/architecture/codebase-structure.md`.

Content covers: Proposed directory layout, file organization principles.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/structure.md`
- Output: `.orqa/documentation/architecture/codebase-structure.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content reflects the ACTUAL post-migration directory structure (updated from proposed)
- [ ] Relationships to core architecture doc

**Reviewer Checks:**

- Verify the documented structure matches the actual codebase
- Frontmatter validation

---

#### TASK 11.1.9: Convert glossary.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/glossary.md` into `.orqa/documentation/architecture/glossary.md`.

Content covers: Precise definitions of all architectural terms.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/glossary.md`
- Output: `.orqa/documentation/architecture/glossary.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] All terms from ARCHITECTURE.md glossary are included
- [ ] Definitions are accurate and up-to-date

**Reviewer Checks:**

- Verify every term has a definition
- Verify no stale definitions (e.g., terms for removed concepts)

---

#### TASK 11.1.10: Convert targets.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/targets.md` into `.orqa/documentation/architecture/target-specifications.md`.

Content covers: Target state specifications for Phase 1, what each target defines, target locations.

Note: Post-migration, this becomes historical reference (targets have been validated and removed). The doc should be updated to reflect that targets were validated and are now produced by generation pipelines.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/targets.md`
- Output: `.orqa/documentation/architecture/target-specifications.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content updated to reflect post-migration state (targets validated, generation pipelines active)
- [ ] Historical context preserved for understanding the migration approach

**Reviewer Checks:**

- Verify content reflects post-migration reality
- Frontmatter validation

---

#### TASK 11.1.11: Convert migration.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/migration.md` into `.orqa/documentation/architecture/migration-plan.md`.

Content covers: Migration principles, all 11 phases, completion test. Post-migration, this becomes a historical record of how the migration was executed.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/migration.md`
- Output: `.orqa/documentation/architecture/migration-plan.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] Content updated to reflect completion status of each phase
- [ ] Historical context preserved

**Reviewer Checks:**

- Verify each phase has accurate completion status
- Frontmatter validation

---

#### TASK 11.1.12: Convert audit.md to documentation artifact

**What:** Convert `targets/claude-code-plugin/.claude/architecture/audit.md` into `.orqa/documentation/architecture/audit-criteria.md`.

Content covers: 9 audit criteria for reviewing files against the architecture.

**Files:**

- Source: `targets/claude-code-plugin/.claude/architecture/audit.md`
- Output: `.orqa/documentation/architecture/audit-criteria.md`

**Acceptance Criteria:**

- [ ] DOC artifact with valid frontmatter
- [ ] All 9 audit criteria included
- [ ] Content accurate and applicable post-migration

**Reviewer Checks:**

- Verify all 9 criteria are present
- Frontmatter validation

---

### 11.2 Architecture File Conversion: Knowledge Artifacts

Each documentation artifact gets derived knowledge artifacts for agent consumption. Knowledge artifacts are 500-2,000 tokens, atomic, self-contained, with injection metadata (tier, roles, paths, tags).

#### TASK 11.2.1: Create knowledge artifacts from core architecture

**What:** Create KNOW artifacts derived from the core architecture documentation. Break into atomic, agent-consumable chunks:

1. `KNOW-xxx: Engine Library Architecture` — what each engine crate provides, language boundary
2. `KNOW-xxx: Design Principles` — P1-P7 constraints as a quick reference
3. `KNOW-xxx: Access Layer Taxonomy` — app vs CLI vs daemon vs MCP vs LSP

Each knowledge artifact needs injection metadata: `tier` (always/stage-triggered/on-demand), `roles` (which agent roles need this), `paths` (file path triggers), `tags` (semantic tags).

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (3 files)

**Acceptance Criteria:**

- [ ] 3 KNOW artifacts created, each 500-2,000 tokens
- [ ] Each has valid frontmatter with injection metadata (tier, roles, paths, tags)
- [ ] Content is self-contained (no "see other doc" dependencies for core meaning)
- [ ] `relates-to` relationships link to the parent DOC artifact
- [ ] Passes artifact schema validation

**Reviewer Checks:**

- Verify each artifact is within token budget
- Verify injection metadata is correct (e.g., design principles should be tier: always)
- Verify content is self-contained

---

#### TASK 11.2.2: Create knowledge artifacts from plugin architecture

**What:** Create KNOW artifacts from plugin architecture:

1. `KNOW-xxx: Plugin Taxonomy` — purposes, categories, what each type provides
2. `KNOW-xxx: Plugin Composition Pipeline` — how plugins compose schemas and workflows
3. `KNOW-xxx: Plugin Manifest Format` — manifest fields, what each means

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (3 files)

**Acceptance Criteria:**

- [ ] 3 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Injection metadata accuracy

---

#### TASK 11.2.3: Create knowledge artifacts from governance artifacts doc

**What:** Create KNOW artifacts from governance artifacts documentation:

1. `KNOW-xxx: .orqa/ Directory Structure` — what goes where, artifact organization
2. `KNOW-xxx: Artifact Lifecycle` — how artifacts move through statuses, who creates/transitions them

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (2 files)

**Acceptance Criteria:**

- [ ] 2 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Injection metadata accuracy

---

#### TASK 11.2.4: Create knowledge artifacts from agent architecture

**What:** Create KNOW artifacts from agent architecture:

1. `KNOW-xxx: Base Agent Roles` — 8 roles, permissions, responsibilities
2. `KNOW-xxx: Prompt Generation Pipeline` — pipeline stages, how prompts are assembled
3. `KNOW-xxx: Knowledge Injection Tiers` — always/stage-triggered/on-demand, how each works

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (3 files)

**Acceptance Criteria:**

- [ ] 3 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Verify role permissions match ARCHITECTURE.md

---

#### TASK 11.2.5: Create knowledge artifacts from connector architecture

**What:** Create KNOW artifacts from connector architecture:

1. `KNOW-xxx: Connector Generation Pipeline` — what a connector generates, trigger conditions
2. `KNOW-xxx: Generated Plugin Structure` — what the connector output contains, file purposes

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (2 files)

**Acceptance Criteria:**

- [ ] 2 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Injection metadata accuracy

---

#### TASK 11.2.6: Create knowledge artifacts from enforcement design

**What:** Create KNOW artifacts from enforcement design:

1. `KNOW-xxx: State Machine Primitives` — states, guards, transitions, categories
2. `KNOW-xxx: Enforcement Layers` — which tools enforce what, validation timing

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (2 files)

**Acceptance Criteria:**

- [ ] 2 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Injection metadata accuracy

---

#### TASK 11.2.7: Create knowledge artifact from glossary

**What:** Create a single KNOW artifact from the glossary. The glossary is already concise and atomic — it may fit in one knowledge artifact. If too large, split into System Components and Plugin Ecosystem sections.

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (1-2 files)

**Acceptance Criteria:**

- [ ] KNOW artifact(s) created, each 500-2,000 tokens
- [ ] All terms from the glossary are included
- [ ] tier: always (glossary definitions should always be available)
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Verify all terms present
- Token budget compliance

---

#### TASK 11.2.8: Create knowledge artifacts from decisions architecture

**What:** Create KNOW artifacts from the decisions architecture documentation (DOC from 11.1.7):

1. `KNOW-xxx: Decision Framework` — how architectural decisions are recorded, reviewed, and superseded
2. `KNOW-xxx: Principle vs Planning Decisions` — the distinction between principle-level and tactical decisions, when to use each

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (2 files)

**Acceptance Criteria:**

- [ ] 2 KNOW artifacts, each 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Self-contained content
- [ ] Relationships to parent DOC (decisions architecture)

**Reviewer Checks:**

- Token budget compliance
- Injection metadata accuracy

---

#### TASK 11.2.9: Create knowledge artifact from codebase structure

**What:** Create KNOW artifact from the structure architecture documentation (DOC from 11.1.8):

1. `KNOW-xxx: Codebase Structure` — top-level directory layout, what each directory contains, where to find things

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (1 file)

**Acceptance Criteria:**

- [ ] 1 KNOW artifact, 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] tier: always (all agents need to know the codebase layout)
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Verify structure matches post-migration reality
- Token budget compliance

---

#### TASK 11.2.10: Create knowledge artifact from targets architecture

**What:** Create KNOW artifact from the targets architecture documentation (DOC from 11.1.10):

1. `KNOW-xxx: Target-Driven Development` — what targets are, how they are used for validation, the validate-then-replace workflow

**Note:** This may become historical reference post-migration. If targets are no longer relevant, the KNOW artifact should still be created to document the methodology, with `status: archived` and `tier: on-demand`.

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (1 file)

**Acceptance Criteria:**

- [ ] 1 KNOW artifact, 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] Appropriate tier based on post-migration relevance
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Verify tier assignment reflects post-migration relevance

---

#### TASK 11.2.11: Create knowledge artifact from migration architecture

**What:** Create KNOW artifact from the migration architecture documentation (DOC from 11.1.11):

1. `KNOW-xxx: Migration Methodology` — the 11-phase migration approach, lessons learned, how to plan large-scale codebase migrations

**Note:** This is primarily historical reference. Set `status: archived` and `tier: on-demand` since migration is complete.

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (1 file)

**Acceptance Criteria:**

- [ ] 1 KNOW artifact, 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] `tier: on-demand` (historical reference, not needed for routine work)
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Verify content captures key methodology insights, not just phase descriptions

---

#### TASK 11.2.12: Create knowledge artifact from audit criteria

**What:** Create KNOW artifact from the audit criteria documentation (DOC from 11.1.12):

1. `KNOW-xxx: Architecture Audit Criteria` — the 9 criteria for reviewing files against the architecture, how to conduct an audit

**Files:**

- Output: `.orqa/documentation/architecture/knowledge/` (1 file)

**Acceptance Criteria:**

- [ ] 1 KNOW artifact, 500-2,000 tokens
- [ ] Valid frontmatter with injection metadata
- [ ] tier: stage-triggered (relevant during review stage)
- [ ] roles: reviewer (primarily used by reviewers)
- [ ] Relationships to parent DOC

**Reviewer Checks:**

- Token budget compliance
- Verify all 9 audit criteria are represented
- Verify injection metadata targets the correct role

---

### 11.3 Cleanup

#### TASK 11.3.1: Remove file-audit/ directory

**What:** The `file-audit/` directory contains audit working files that are no longer needed post-migration. All findings have been acted upon. Delete the entire directory.

**Files:**

- `file-audit/` (entire directory)

**Acceptance Criteria:**

- [ ] `file-audit/` directory is deleted
- [ ] No references to `file-audit/` remain in CLAUDE.md or other config files
- [ ] Audit findings have been captured in appropriate documentation artifacts or are reflected in completed migration work

**Reviewer Checks:**

- Grep for `file-audit` in all files — no references should remain
- Verify the directory does not exist

---

#### TASK 11.3.2: Update ARCHITECTURE.md to reflect post-migration state

**What:** Update the root `ARCHITECTURE.md` to reflect that the migration is complete:

- Migration plan section updated with completion status
- Target state specifications section updated (targets validated and removed)
- Audit criteria section remains (still applicable for ongoing development)
- Remove references to `targets/` directory
- Remove references to `file-audit/` directory
- Update references to new documentation and knowledge artifact locations

**Files:**

- `ARCHITECTURE.md`

**Acceptance Criteria:**

- [ ] Migration plan section reflects completion
- [ ] No references to `targets/` or `file-audit/` directories
- [ ] References to architecture documentation point to `.orqa/documentation/architecture/`
- [ ] Document is accurate for the post-migration codebase

**Reviewer Checks:**

- Verify no stale references to removed directories
- Verify architecture documentation pointers are correct
- Verify migration phases have completion status

---

#### TASK 11.3.3: Update CLAUDE.md references

**What:** Update the root `.claude/CLAUDE.md` and any other CLAUDE.md files to reference the new documentation and knowledge artifact locations instead of removed directories.

**Files:**

- `.claude/CLAUDE.md`
- Any other CLAUDE.md files that reference targets/ or file-audit/

**Acceptance Criteria:**

- [ ] No references to `targets/` directory
- [ ] No references to `file-audit/` directory
- [ ] Architecture reference section points to `.orqa/documentation/architecture/`
- [ ] Reference documents section updated

**Reviewer Checks:**

- Grep for `targets/` and `file-audit/` in all CLAUDE.md files
- Verify architecture reference links are valid

---

#### TASK 11.3.4: Verify documentation/knowledge hierarchy completeness

**What:** Final verification that every architectural concept from ARCHITECTURE.md has:

1. A DOC artifact in `.orqa/documentation/architecture/`
2. Derived KNOW artifact(s) in `.orqa/documentation/architecture/knowledge/`
3. Correct relationships between DOC and KNOW artifacts
4. Correct injection metadata on KNOW artifacts

**Files:**

- `.orqa/documentation/architecture/` (all files)
- `.orqa/documentation/architecture/knowledge/` (all files)

**Acceptance Criteria:**

- [ ] 12 DOC artifacts exist (one per architecture split file)
- [ ] 16-18 KNOW artifacts exist (derived from DOC artifacts)
- [ ] Every DOC has at least one derived KNOW
- [ ] Every KNOW has a `relates-to` relationship to its parent DOC
- [ ] All artifacts pass schema validation
- [ ] Injection metadata coverage: at least one `tier: always` knowledge per major domain

**Reviewer Checks:**

- Count DOC and KNOW artifacts
- Verify relationship integrity (no broken links)
- Run artifact schema validation on all new artifacts
- Verify injection metadata tiers are appropriate

---

## Summary

| Phase | Task Count | Description |
| ------- | ----------- | ------------- |
| **Phase 9** | 23 tasks | Frontend alignment: 2 HIGH + 9 MEDIUM + 5 LOW hardcoding fixes, 3 navigation tasks, 1 settings reorg, 2 custom views verification (incl. roadmap), 1 build verification |
| **Phase 10** | 21 tasks | Target validation: 1 schema, 12 Claude Code plugin (8 original + 4 migration target), 6 enforcement config, 2 cleanup/completion |
| **Phase 11** | 28 tasks | Post-migration docs: 12 DOC conversions, 12 KNOW creation tasks (7 original + 5 added for decisions/structure/targets/migration/audit), 4 cleanup/verification |
| **Total** | **72 tasks** | |

### Dependency Chain

```text
Phase 9 (all tasks) --> TASK 9.7.1 (build verification)
                            |
                            v
Phase 10.1-10.3 (all validation tasks, can run in parallel per category)
                            |
                            v
TASK 10.4.1 (remove targets/) --> TASK 10.4.2 (completion test)
                                      |
                                      v
Phase 11.1 (DOC conversions, can run in parallel)
                            |
                            v
Phase 11.2 (KNOW creation, depends on corresponding DOC)
                            |
                            v
Phase 11.3 (cleanup tasks)
                            |
                            v
TASK 11.3.4 (final hierarchy verification)
```text

---

### Phase 10 Addendum: DOC/KNOW Sync Verification Pipeline

> Added 2026-03-28 based on design discussion. DOC/KNOW sync is part of the artifact integrity pipeline — content accuracy, state machines, relationship validation, and sync verification all run through the same enforcement system.

#### TASK 10.5.1: Implement DOC/KNOW sync_hash in frontmatter

**What:** Add `sync_hash:` field to KNOW frontmatter for each `synchronised-with` DOC relationship. The hash is a content hash of the corresponding DOC. Many-to-many: one KNOW can sync to multiple DOCs (storing a hash per DOC), one DOC can sync to multiple KNOWs.

**Acceptance Criteria:**

- [ ] All KNOW files with `synchronised-with` relationships have `sync_hash:` entries
- [ ] Hash is computed from DOC content (e.g., SHA-256 of body after frontmatter)
- [ ] Many-to-many: `sync_hashes:` is an object keyed by DOC ID

#### TASK 10.5.2: Implement sync verification command

**What:** Create `orqa sync --verify KNOW-xxx` command that:

1. Reads all DOCs linked via `synchronised-with`
2. Shows diff between current DOC content hash and stored sync_hash
3. Requires explicit acknowledgment flag to update sync state
4. For KNOW files synced to multiple DOCs: when one DOC changes, ALL related DOCs must be re-verified (consistency mesh)

**Acceptance Criteria:**

- [ ] Command shows which DOCs have changed since last sync
- [ ] Explicit flag required to acknowledge (not automatic)
- [ ] All sync_hashes recomputed on acknowledgment (forces review of all linked DOCs)
- [ ] Project setting controls who can verify: `sync_verification: human | orchestrator | auto`

#### TASK 10.5.3: Wire sync verification into git hook

**What:** Add DOC/KNOW sync check to pre-commit hook:

1. For each staged DOC-*.md, find KNOW files linked via `synchronised-with`
2. Check that sync_hashes are current (verification command was run)
3. Block commit if any sync is stale with descriptive message
4. Explicit verification flag required — hash recomputation alone is NOT sufficient

**Acceptance Criteria:**

- [ ] Staged DOC changes with stale KNOW sync block commit
- [ ] Error message names the affected KNOW files and DOCs
- [ ] Verification requires explicit command, not just hash update

#### TASK 10.5.4: Wire sync awareness into LSP

**What:** Add inline warnings via LSP when editing DOC files:

1. Show that content sections are synchronized to KNOW anchors
2. Show which other DOCs share the same KNOW anchor (consistency mesh)
3. On save, flag affected KNOWs as stale in daemon

**Acceptance Criteria:**

- [ ] LSP shows inline diagnostics for synchronized content sections
- [ ] Diagnostics identify the KNOW anchor and related DOCs
- [ ] File watcher integration flags stale KNOWs on DOC save

#### TASK 10.5.5: Implement consistency mesh detection

**What:** When a KNOW is synced to multiple DOCs and one DOC changes, detect potential inconsistency across all DOCs sharing that anchor. Even if both DOCs changed and hashes were updated, the content itself may still be inconsistent — require additional verification.

**Acceptance Criteria:**

- [ ] Changing DOC-abc triggers review of DOC-def if both sync to KNOW-xyz
- [ ] Verification is not just "did hashes update" but "has consistency been confirmed"
- [ ] Works through the artifact integrity pipeline (same system as state machines, relationship validation)

#### TASK 10.5.6: Complete artifact integrity pipeline audit

**What:** Conduct a comprehensive review of the entire artifact lifecycle to identify ALL processes that could be enforced via the verification system. This is not limited to known gap areas — it is a ground-up audit of every point where artifact integrity could be mechanically verified or consistency could be enforced.

The audit should examine every artifact type, every relationship type, every workflow transition, every content convention, and every cross-artifact dependency to determine what CAN be mechanically enforced. The output is a complete enforcement catalog — the definitive list of what the integrity pipeline should check.

For each identified enforcement opportunity:

- Describe what is being verified
- Classify as mechanical (automatable) or advisory (requires judgment)
- Specify the enforcement point (git hook, file watcher, LSP, CLI command, daemon endpoint)
- Specify the verification method (hash comparison, schema validation, graph query, content analysis, relationship check)
- Assess the cost/benefit (is this worth enforcing mechanically?)

**Acceptance Criteria:**

- [ ] Complete audit covering every artifact type and relationship in the schema
- [ ] Every identified enforcement opportunity documented with classification and method
- [ ] No pre-determined scope limitations — the audit discovers what's needed, not what's expected
- [ ] Findings written to a decision artifact (AD-*.md) for review before implementation
- [ ] Implementation priority assigned to each (critical/high/medium/low)
