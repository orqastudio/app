# Migration Plan

> This is part of the OrqaStudio Architecture Reference. See ARCHITECTURE.md for the complete document.

---

## 13. Migration Plan

### Principle: Target States First

Before building the generation pipelines, **hand-write the target outputs** as test fixtures. These represent what the finished system would produce. Development validates against these targets. The targets are only replaced by generated output once the generation code is complete and validated.

**Protection rule:** Target state files are protected during development. They must NOT be overwritten by work-in-progress code. A target is only replaced when the code that generates it produces output that matches or exceeds the hand-written version.

**Zero tech debt enforcement:** Every migration phase must leave zero legacy behind. No deprecated code, no stale artifacts, no commented-out blocks, no backwards-compatibility shims, no "follow-up" tasks that defer cleanup. If a file, function, artifact, or config doesn't serve the target architecture, it is deleted in the same phase that replaces it. Legacy code and artifacts left in the codebase WILL influence future agent behavior in the wrong direction — this is not theoretical, it is the exact problem this migration is solving.

**Regeneration safeguard:** Generation pipelines (connector, enforcement plugins) must support a **dry-run mode** controlled by an environment variable (`ORQA_DRY_RUN=true`). When dry-run is enabled, pipelines write their output to a comparison directory (e.g., `.state/dry-run/`) instead of overwriting live files. This allows generated output to be compared and validated against the hand-written targets without affecting the in-progress migration. The environment variable is set to `true` for the duration of the migration and switched to `false` only when the pipeline is validated. This applies to: connector generation (`.claude/`), enforcement config generation (git hooks, eslint, clippy, markdownlint, prettier), and resolved workflow generation.

### Phase 1: Establish Target States and Migration Enforcement

Hand-write the target outputs as test fixtures. These are the FIRST thing built — everything else validates against them. The enforcement tooling must be active before any other migration work begins.

#### Step 1: Target Schema + Validation Script (PREREQUISITE)

This is the foundation — without schema validation, nothing else can be verified:

1. **Write `targets/schema.composed.json`** — the full composed schema for all artifact types, relationships, valid statuses
2. **Write a temporary validation script** (`scripts/validate-artifacts.mjs` or similar) that checks governance artifacts against the target schema. This is a stopgap — it gets replaced by the engine's enforcement crate later. It must validate: required frontmatter fields per type, ID format, type-location consistency, relationship target existence, status validity, knowledge size constraints.
3. **Verify the script works** by running it against the current `.orqa/` directory — it should report the known issues from the audit.

#### Step 2: Install Enforcement Configs

Copy target enforcement configs from `targets/enforcement/` into the live project:

- `targets/enforcement/eslint/eslint.config.js` -> `app/eslint.config.js`
- `targets/enforcement/clippy/clippy.toml` -> workspace root `clippy.toml`
- `targets/enforcement/clippy/workspace-lints.toml` -> apply to `Cargo.toml` `[workspace.lints]`
- `targets/enforcement/markdownlint/.markdownlint.json` -> project root `.markdownlint.json`
- `targets/enforcement/prettier/.prettierrc` -> project root `.prettierrc`
- `targets/enforcement/prettier/.prettierignore` -> project root `.prettierignore`
- Install required dependencies (`prettier`, `prettier-plugin-svelte`, `prettier-plugin-tailwindcss`, `markdownlint-cli2`)

#### Step 3: Migration `.claude/` Instance

Write the migration-specific `.claude/` directory with hooks that call enforcement tooling **directly** (not thin daemon wrappers — the daemon doesn't exist yet):

- Hooks invoke eslint, clippy, markdownlint, the validation script, and scoped tests directly
- Artifact validation hook calls the temporary validation script against the target schema
- Hooks are more aggressive than the post-migration thin wrappers — they enforce everything at every step because the migration is critical
- Agent definitions include full migration context (architecture references, target state awareness, phase plan)
- **Artifact schema validation is DISABLED in hooks until Phase 6 (Content Cleanup) and Phase 7 (Governance Artifact Migration) are complete.** Enabling it before artifacts are fixed would block every commit. The validation script exists and can be run manually to track progress, but the pre-commit hook skips it via an `ORQA_SKIP_SCHEMA_VALIDATION=true` environment variable. This variable is removed when the artifacts are fixed and validation should be enforced.

#### Step 4: Remaining Targets

With enforcement in place, write the remaining targets:

| Target | What It Defines | Location |
|--------|----------------|----------|
| **Target Claude Code Plugin** | The ideal `.claude/` output the connector should generate | `targets/claude-code-plugin/` |
| **Target `.orqa/` structure** | Governance artifacts as the app would generate them | Applied directly to `.orqa/` |
| **Target resolved workflows** | One per stage, fully composed | `targets/workflows/` |
| **Target plugin manifests** | orqa-plugin.json for each plugin with correct taxonomy fields | `targets/plugin-manifests/` |

Additionally, the actual governance artifacts in `.orqa/` must be corrected to match the target state — correct artifact types, correct frontmatter, correct locations, correct relationships. The LSP/MCP then validates against the end-goal, not WIP, and can accurately identify issues.

These targets serve as:

1. **Test fixtures** — the generation pipeline must produce output matching these
2. **Validation benchmarks** — LSP/MCP validate against the target schema, not WIP
3. **Development reference** — developers see what the end state looks like
4. **Living data** — the `.orqa/` artifacts are correct and complete, ready for the app to display

See **Appendix A** for the detailed target state specifications.

### Phase 2: Engine Extraction

Extract business logic from Tauri backend and CLI into Rust library crates:

1. **Types and traits first** — shared type definitions, storage traits
2. **Graph crate** — artifact reader, relationship engine, traceability
3. **Enforcement crate** — rule evaluation, artifact validation (absorb app/.githooks/ logic)
4. **Search crate** — semantic search (already partially extracted)
5. **Workflow crate** — state machine evaluation, status transitions, guards
6. **Plugin crate** — installation, composition, schema generation
7. **Prompt crate** — prompt generation pipeline (absorb connector's prompt-injector and knowledge-injector)
8. **Agent crate** — base role definitions, task-specific agent generation
9. **Stream loop abstraction** — design proper traits for sidecar communication and event delivery
10. **Absorb app/tools/ engine logic** — `verify-pipeline-integrity.mjs`, `verify-installed-content.mjs`, `lint-relationships.mjs` implement engine-level business logic

### Phase 3: Daemon

Build the daemon as a standalone Rust process:

1. System tray icon with context menu (process status, app launch)
2. File watchers for plugin/rule/workflow changes
3. MCP server (consuming engine crates)
4. LSP server (consuming engine crates, validating against composed schema)
5. Unified logging (future: split into metrics + logger)

### Phase 4: Connector Cleanup

Refactor the connector to be pure generation + watching:

1. Move prompt classification logic to engine prompt crate
2. Move knowledge injection logic to engine prompt crate
3. Move context preservation logic to engine
4. Replace direct MCP IPC with daemon calls
5. Move connector knowledge artifacts to methodology plugin
6. Implement `generator.ts` (primary connector job) and `watcher.ts` (live regeneration)
7. Generate hooks.json from plugin declarations
8. Delete legacy `artifact-bridge.ts`
9. `.claude/` directory becomes connector output — stop hand-maintaining `.claude/agents/` and `.claude/CLAUDE.md`
10. Validate generated output against target Claude Code Plugin

### Phase 5: Plugin Manifest Standardization

Update all plugin manifests to support the architecture:

1. Add `purpose` field (methodology, workflow, knowledge, connector, infrastructure, sidecar)
2. Add `stage_slot` field for workflow plugins
3. Standardize category vocabulary to match taxonomy
4. Declare content installation targets (where in `.orqa/` hierarchy)
5. Implement installation constraint enforcement in `orqa install`
6. Standardize schema field naming — `title` not `name` across all artifact type schemas
7. Fix missing files referenced by manifests (KNOW-3f307edb, review-checklist.md) — either create in plugin source or remove references
8. Fix software-kanban contribution workflow missing from manifest
9. Fix agile-documentation inconsistent workflow declaration format (flat string -> structured object)
10. Rename plugins for clarity — names must make taxonomy self-evident (e.g., `agile-workflow` -> `agile-methodology`)

### Phase 6: Content Cleanup (Zero Dead Weight)

Review and clean ALL governance content. Nothing survives that isn't accurate, relevant, and forward-compatible.

#### Scripts

- Remove every script that isn't forward-compatible with the target architecture
- Dead migration scripts, obsolete tooling, one-time fixes that have run — all deleted
- Specifically: `migrate-artifact-ids.mjs`, `standardise-ids.mjs`, `fix-duplicate-frontmatter-keys.mjs`, `fix-missing-inverses.mjs`, `link-skills-to-docs.mjs`, `remove-inverse-relationships.mjs`, `migrate-types.mjs`, `rebuild-artifacts.mjs` — all completed migrations, delete
- Migration manifests (`id-migration-manifest.json`, `id-standardise-manifest.json`, `migration-manifest.json`) — delete or archive to `.state/migrations/`

#### Documentation and Knowledge

- Review all project documentation for accuracy — update or remove anything stale
- Review all knowledge artifacts for accuracy and duplication — merge duplicates, remove obsolete
- Ensure documentation and knowledge are sourced from the correct plugins (not orphaned project copies of plugin content)
- Organize into domain subdirectories for human navigation

#### Ideas

- Combine/group ideas that are thematically the same
- Archive ideas that are no longer relevant to the architecture

#### Decisions

- Review every decision for accuracy against the current architecture
- Archive decisions that were superseded or no longer apply
- Split into `principle-decision` and `planning-decision` types

#### Epics and Tasks

- Archive all epics/tasks that aren't about the path forward
- Ensure remaining epics/tasks align with this migration plan
- Clean status values (consistent quoting, valid statuses)

#### Lessons

- Review for ongoing relevance — archive lessons about superseded approaches
- Any valid lesson that can be guarded mechanically becomes a mechanical guard immediately — no recurrence threshold needed at this stage
- Convert applicable lessons into enforcement rules, validation checks, or workflow guards

#### Rules

- Review all rules for accuracy against the current architecture
- Classify every rule as **mechanical** (enforced by tooling) or **advisory** (guidance for agents/humans)
- Rules must support filtering/grouping by this classification
- Remove rules that contradict the plugin-composed architecture or are made redundant by it

#### .state/ Cleanup

- Delete empty team directories (e.g., `fix-sot/`)
- Establish cleanup policy: team findings are ephemeral — promote valuable content to governance artifacts, then delete
- CLI should provide `orqa dev clean-teams [--age <days>]` to prune stale team directories

#### Legacy File Removal

- Delete `validation_stderr.txt` (root)
- Delete `tmp/` directory (superseded by `.state/`)
- Delete `app/WORKING-DOCUMENT.md` (legacy)
- Remove vendored `node_modules` from `integrations/claude-agent-sdk/` (add to `.gitignore`)
- Remove legacy CLI aliases (no backwards compatibility per architecture)
- Fix CLI version hardcoding in `cli.ts` to read from `package.json` dynamically

### Phase 7: Governance Artifact Migration

Restructure `.orqa/` to match target structure:

1. Remove `process/` nesting — promote categories to top-level
2. Remove `agents/` directory — delete AGENT-*.md files
3. Remove `grounding/` directory — migrate to `tier: always` knowledge in plugins
4. Remove SKILL.md files from knowledge
5. Categorize knowledge and documentation into domain subdirectories
6. Fix wireframe artifact type (doc -> wireframe)
7. Fix personas directory (move DOC to documentation/)
8. Standardize frontmatter (title not name, required status, consistent quoting)
9. Regenerate resolved workflows as one-per-stage

### Phase 8: Codebase Restructure

Move directories to match the proposed structure:

1. Move engine crates to `libs/`
2. Create `daemon/` top-level directory
3. Restructure `plugins/` into taxonomy subdirectories
4. Move CLI to top-level `cli/`
5. Move claude-agent-sdk to top-level `sidecars/`
6. Remove sync-bridge (aspirational, not needed now)
7. Remove all dead scripts identified in Phase 5
8. Update all import paths, Cargo workspace, and package.json references
9. Update templates — remove `process/` prefix from content paths, add missing template types (methodology, workflow, knowledge-only, infrastructure, connector, sidecar)
10. Remove `file-audit/` directory (audit working files, not permanent)
11. Update `CLAUDE.md` to reflect the new architecture (or generate it via connector)
12. Reconcile relationship type count (41 in plugins vs 30 stated in CLAUDE.md)

### Phase 9: Frontend Alignment

Review and update the app frontend to work with the target architecture:

#### Navigation Structure

- **Dashboard** — top-level landing with insight widgets sourced from appropriate plugins. Ensure existing widgets work; detailed redesign is future scope.
- **Methodology stages** — one main nav item per methodology stage (Discovery, Planning, Documentation, Implementation, Review, Learning). Artifacts organized by where they fit in each stage's sub-workflow. Navigation structure generated from the methodology plugin and its stage plugins, not hardcoded.
- **Plugins** — top-level nav item (above Settings). Shows available/installed plugins with filters by category (knowledge, methodology, workflow, sidecar, connector, infrastructure). Surface **plugin groups** that bundle a methodology + all its stage plugins together (e.g., "Agile Software Development" installs methodology + all workflow stages). The `core` plugin is NOT surfaced to users — it's only a plugin for architectural composability, but it IS part of the core framework.
- **Settings** — bottom nav item. Reorganized to reflect the architecture:
  - **Methodology** — dedicated section for the installed methodology plugin. Workflow plugins nested underneath (they serve a specific required purpose within the methodology). Settings pages generated by the methodology and workflow plugins themselves.
  - **Sidecar** — dedicated section for the installed sidecar(s). Required plugin type, gets its own area.
  - **Connector** — dedicated section for installed connector(s). Settings pages generated by the connector plugin.
  - **Plugins** — generic section for all other installed plugins (knowledge, infrastructure, coding-standards, etc.). Grouped together since they're optional extensions.
  - Each plugin generates its own settings pages, which appear in the appropriate section AND are reachable via a link on the installed plugins list.
  - Global vs project settings remain separate but follow the same organizational structure.
  - **Remove the navigation settings page** — navigation is plugin-driven for now. Future: allow users to override the default navigation structure (methodology stages -> workflow artifacts -> custom views) with custom layouts, enabling plugins to create alternate artifact views.

#### Custom Views

- Review the roadmap view to ensure it works with the milestone/epic hierarchy
- Custom views contributed by plugins should render correctly in the new navigation structure

#### Hardcoded Pattern Removal

- Remove hardcoded artifact type prefixes, status values, stage names, and sort orders
- Replace with engine-provided data from composed schema and resolved workflows
- Fix hardcoded sidecar plugin name in StatusBar
- Deduplicate model options (currently in 3 files)

### Phase 10: Validate Against Targets

For each target artifact from Phase 1:

1. Run the generation pipeline
2. Compare generated output against hand-written target
3. If match: replace target with generated version
4. If gap: fix the generation pipeline, do not modify the target
5. Remove `targets/` directory once all generation is validated

### Phase 11: Post-Migration Documentation

Create proper project documentation and knowledge artifacts for all architecture content that remains relevant post-migration:

1. Convert each split architecture file (`targets/claude-code-plugin/.claude/architecture/*.md`) into proper `.orqa/` documentation and knowledge artifacts
2. Documentation files (DOC) for human-readable reference, organized by topic
3. Knowledge files (KNOW) for agent-consumable chunks with injection metadata (tier, roles, paths, tags)
4. Ensure the documentation/knowledge hierarchy is complete — every architectural concept has both a doc and derived knowledge
5. Remove the `targets/` directory — all targets are now produced by generation pipelines
6. Remove the `file-audit/` directory — audit is complete

### Completion Test

The system is complete when:

- Every target from Phase 1 is produced by a generation pipeline
- The same methodology and principles apply whether working via the app or via Claude Code
- All enforcement is mechanical (generated hooks, linting, validation, permissions)
- The `.orqa/` directory looks like something the finished app would have created
- Agents work without bypass permissions, scoped to their role
- Architecture documentation exists as proper governance artifacts, not just a root-level markdown file
