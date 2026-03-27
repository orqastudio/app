# File Audit: App Config, Libs, Infrastructure, and Misc

Phase 1 factual inventory. No recommendations or judgments.

---

## 1. App Root Files

### package.json
- **Path:** `app/package.json`
- **Name:** `orqa-studio`, version `0.1.4-dev`, private, type module
- **Dependencies:** @orqastudio/graph-visualiser, sdk, svelte-components, types (all `0.1.4-dev`); @tauri-apps/api `^2.3.0`, @tauri-apps/plugin-dialog `^2.2.0`, @tauri-apps/plugin-fs `^2.2.0`, @tauri-apps/plugin-notification `^2.2.0`, @tauri-apps/plugin-shell `^2.2.0`; cytoscape `^3.30.4`, mermaid `^11.4.1`, svelte-markdown `^0.4.1`, svelte-highlight `^7.7.0`
- **DevDependencies:** @sveltejs/adapter-static `^3.0.8`, @sveltejs/kit `^2.16.0`, @tailwindcss/vite `^4.0.7`, svelte `^5.19.7`, svelte-check `^4.1.4`, tailwindcss `^4.0.7`, typescript `^5.7.3`, vite `^6.2.4`, vitest `^3.0.0`, eslint `^9.0.0`

### svelte.config.js
- **Path:** `app/svelte.config.js`
- Uses static adapter, prerender entries `['*']`, strict mode disabled

### vite.config.ts
- **Path:** `app/vite.config.ts`
- Server port `10420`, strictPort true
- Excludes `@orqastudio/*` from Vite pre-bundling (HMR for linked libs)
- FS access allowed: `src`, `.svelte-kit`, `node_modules`, `../libs`, `../plugins`, `../connectors`
- Test config: jsdom environment, v8 coverage, setup file at `src/lib/components/shared/__tests__/setup.ts`

### tsconfig.json
- **Path:** `app/tsconfig.json`
- Extends `.svelte-kit/tsconfig.json`

### eslint.config.js
- **Path:** `app/eslint.config.js` (83 lines)
- Enforces RULE-006: no-explicit-any everywhere, no `invoke()` in `$lib/components/`
- Enforces RULE-033: no HTML `title` attribute (use shadcn Tooltip)
- Excludes `ui/` from component purity rules (vendored shadcn components)

### components.json
- **Path:** `app/components.json`
- shadcn/ui configuration: Svelte, TypeScript, `$lib/components/ui` base, Tailwind CSS v4

### WORKING-DOCUMENT.md
- **Path:** `app/WORKING-DOCUMENT.md` (446 lines)
- Architecture document covering three-layer artifact model, canonical relationships, state machine, enforcement layers
- References AD-049 through AD-054
- Documents artifact types, relationship vocabulary (30 types), status enums

### VERSIONS
- **Path:** `app/VERSIONS`
- Lists all version-bearing files in the app submodule (package.json, Cargo.toml)

### NOTICE
- **Path:** `app/NOTICE`
- BSL-1.1 license notice with Change License reference

### THIRD_PARTY_NOTICES.md
- **Path:** `app/THIRD_PARTY_NOTICES.md`
- Third-party dependency license notices

### CHANGE-LICENSE
- **Path:** `app/CHANGE-LICENSE` (93 lines)
- Apache 2.0 with Ethical Use Addendum
- Prohibits use by organisations restricting rights based on protected characteristics (gender identity explicitly listed)
- No hosted service (SaaS) without written agreement

### .gitignore
- **Path:** `app/.gitignore` (65 lines)
- Ignores: target/, node_modules/, dist/, .svelte-kit/, orqa.db*, src-tauri/target/, .vscode/, .idea/, .env*, .state/, *.log
- Ignores .claude/ junction directories (agents/, rules/, skills/, hooks/, plugins/, worktrees/)
- Ignores plugin-generated config files: sidecar-config.json, plugin-tools.json, plugin-hooks.json

---

## 2. App .githooks/

### pre-commit
- **Path:** `app/.githooks/pre-commit` (160 lines, bash)
- Master pre-commit orchestrator. Runs checks in order:
  1. Rust format (`cargo fmt --check`), lint (`cargo clippy`), test (`cargo test`)
  2. Frontend typecheck (`npx svelte-check`), lint (`npx eslint`), test (`npx vitest run`)
  3. Artifact schema validation (`node .githooks/validate-schema.mjs`)
  4. Dangling link validation (`node .githooks/validate-links.mjs`)
  5. Pipeline integrity (`node tools/verify-pipeline-integrity.mjs`)
  6. Status transition validation (`node .githooks/validate-status-transitions.mjs`)
  7. Pillar alignment check (`node .githooks/validate-pillar-alignment.mjs`)
  8. Relationship type validation (`node .githooks/validate-relationships.mjs`)
  9. Historical preservation check (`node .githooks/validate-history.mjs`)
  10. Config-disk consistency (`node .githooks/validate-config-disk.mjs`)
  11. Stub scanning (`node .githooks/scan-stubs.mjs`)
  12. Lint suppression audit (`node .githooks/audit-lint-suppressions.mjs`)
  13. Task dependency gate (`node .githooks/validate-task-deps.mjs`)
  14. Epic readiness gate (`node .githooks/validate-epic-readiness.mjs`)
  15. Core graph artifact protection (`node .githooks/validate-core-graph.mjs`)
  16. Plugin source protection (`node .githooks/validate-plugin-sources.mjs`)
- Skips Rust checks if no `.rs` files staged; skips frontend if no `.ts/.svelte/.css` files staged; skips artifact validation if no `.orqa/` files staged

### validate-schema.mjs
- **Path:** `app/.githooks/validate-schema.mjs` (149 lines)
- Validates YAML frontmatter of staged `.orqa/*.md` files against daemon's `/validate` endpoint
- Fallback to `orqa-validation validate` binary if daemon unavailable
- Port: `10200 + 58 = 10258` (configurable via `ORQA_PORT_BASE`)

### validate-status-transitions.mjs
- **Path:** `app/.githooks/validate-status-transitions.mjs` (224 lines)
- Loads transition rules from plugin manifests (`plugins/*/orqa-plugin.json` and `connectors/*/orqa-plugin.json`)
- Compares old status (from `git show HEAD:file`) vs new status (from daemon/binary parse)
- Validates transitions are legal per plugin-defined `statusTransitions` map

### validate-links.mjs
- **Path:** `app/.githooks/validate-links.mjs`
- Checks relationship targets in staged artifacts exist as valid artifact IDs

### validate-pillar-alignment.mjs
- **Path:** `app/.githooks/validate-pillar-alignment.mjs`
- Verifies new artifacts reference at least one pillar or are in an exempt type list

### validate-relationships.mjs
- **Path:** `app/.githooks/validate-relationships.mjs`
- Validates relationship `type` values are from the canonical vocabulary (loaded from plugin schemas)

### validate-history.mjs
- **Path:** `app/.githooks/validate-history.mjs`
- Blocks deletion of artifacts with status `completed` or `promoted` (historical preservation)

### validate-config-disk.mjs
- **Path:** `app/.githooks/validate-config-disk.mjs`
- Ensures config files on disk match expected content (prevents manual edits to generated configs)

### scan-stubs.mjs
- **Path:** `app/.githooks/scan-stubs.mjs`
- Scans for TODO/FIXME/STUB markers in staged files; warns or blocks depending on count

### audit-lint-suppressions.mjs
- **Path:** `app/.githooks/audit-lint-suppressions.mjs`
- Counts new `eslint-disable` / `@ts-ignore` additions in staged changes; warns if threshold exceeded

### validate-task-deps.mjs
- **Path:** `app/.githooks/validate-task-deps.mjs`
- Blocks moving a task to `in-progress` if its `depends-on` tasks are not completed

### validate-epic-readiness.mjs
- **Path:** `app/.githooks/validate-epic-readiness.mjs`
- Blocks moving an epic to `completed` unless all its tasks are completed

### validate-core-graph.mjs
- **Path:** `app/.githooks/validate-core-graph.mjs`
- Protects core graph artifacts (pillars, principles) from deletion or status regression

### validate-plugin-sources.mjs
- **Path:** `app/.githooks/validate-plugin-sources.mjs` (91 lines)
- Reads `.orqa/manifest.json` to find plugin-sourced files
- In dogfood mode: blocks commits to installed copies; otherwise: warns
- Prevents editing installed copies (should edit plugin source instead)

---

## 3. App scripts/

### rebuild-artifacts.mjs
- **Path:** `app/scripts/rebuild-artifacts.mjs` (715 lines)
- One-off migration script: reads from `.orqa-backup/`, transforms frontmatter (standalone fields to relationships, old vocabulary to canonical), writes to `.orqa/` new structure
- Maps: belongs-to to delivers, documents to informs, practices to grounded-by, verifies to enforces
- Handles path remapping: delivery/ideas to discovery/ideas, process/pillars to principles/pillars

### migration-manifest.json
- **Path:** `app/scripts/migration-manifest.json`
- Large generated JSON file (~79k tokens). Output of rebuild-artifacts.mjs recording old-to-new ID mappings

### link-all-plugins.mjs
- **Path:** `app/scripts/link-all-plugins.mjs`
- Symlinks plugin directories for local development

---

## 4. App tools/

### lib/parse-artifact.mjs
- **Path:** `app/tools/lib/parse-artifact.mjs` (98 lines)
- Shared artifact parser delegating to `orqa-validation` Rust binary
- Searches multiple binary locations: `libs/validation/target/{release,debug}/orqa-validation{,.exe}`
- Exports `parseFrontmatter()` and `parseArtifact()`

### verify-pipeline-integrity.mjs
- **Path:** `app/tools/verify-pipeline-integrity.mjs` (458 lines)
- Checks governance artifacts: relationships exist, no null targets, no deprecated fields
- Enforcement chain checks: ADs without enforcement, promoted lessons without grounded-by
- Pipeline health: recurring unpromoted lessons, stuck observations
- Epic reconciliation: checks for reconciliation tasks

### check-orientation.mjs
- **Path:** `app/tools/check-orientation.mjs` (104 lines)
- Mid-cycle orientation check (TASK-386, PILLAR-003)
- Reads active in-progress epics and tasks from `.orqa/delivery/`
- Shows session state from `.state/session-state.md`
- Shows last 5 git commits and orientation questions

### summarize-artifact.mjs
- **Path:** `app/tools/summarize-artifact.mjs`
- Generates compressed summaries of artifacts for knowledge injection

### verify-installed-content.mjs
- **Path:** `app/tools/verify-installed-content.mjs`
- Verifies installed plugin content matches expected state from manifest

### lint-relationships.mjs
- **Path:** `app/tools/lint-relationships.mjs`
- Standalone linter for relationship validation outside of git hooks

### (Several additional tool scripts in app/tools/ — total 16 files)

---

## 5. libs/brand/

### package.json
- **Path:** `libs/brand/package.json`
- Name: `@orqastudio/brand`, version `0.1.4-dev`, private
- Dependencies: sharp (for icon generation)

### tokens/tokens.json
- **Path:** `libs/brand/tokens/tokens.json`
- **Colours:** Deep Ocean `#0B132B`, Midnight Navy `#1C2541`, Signal Cyan `#00D1FF`
- Full light/dark theme tokens in oklch colour space
- Sidebar-specific tokens for light and dark themes
- **Fonts:** Inter (UI), JetBrains Mono (monospace)
- **Spacing:** xs 4px, sm 8px, md 16px, lg 24px, xl 32px
- **Radius:** base `0.625rem`

### icons.json
- **Path:** `libs/brand/icons.json`
- Maps SVG sources to outputs:
  - `Fin.svg` for small icons/favicons (various sizes)
  - `App Icon.svg` for full app icons
- Deploy targets: `tauri-app` (copies to `app/src-tauri/icons/`), `web-app` (copies to `app/static/`)

### generate-icons.mjs
- **Path:** `libs/brand/generate-icons.mjs`
- Uses sharp to generate PNG icons from SVG sources, deploying to configured targets

---

## 6. libs/cli/

### package.json
- **Path:** `libs/cli/package.json`
- Name: `@orqastudio/cli`, version `0.1.4-dev`
- Binary: `"orqa"` maps to `dist/cli.js`
- Dependencies: `@orqastudio/types`, `@types/node`, `yaml`
- Repository: `git@github.com:orqastudio/orqastudio-lib-cli.git`

### src/cli.ts (177 lines)
- **16 primary commands:** install, plugin, check, test, build, graph, daemon, mcp, metrics, summarize, lsp, version, id, migrate, git, dev
- **Legacy aliases:** setup to install link, link to plugin link, verify to check verify, audit to check audit, enforce to check enforce, repo to git, hosting to git hosting, index to mcp index, log enforcement-response to check enforce response

### src/index.ts (148 lines)
- Library exports for programmatic use by connectors/plugins
- **Exported modules:**
  - Symlink utilities (createSymlink, ensureSymlink, verifySymlink, removeSymlink)
  - Plugin management (installPlugin, uninstallPlugin, listInstalledPlugins, fetchRegistry, readLockfile, writeLockfile, readManifest, validateManifest)
  - Graph browsing (scanArtifactGraph, queryGraph, getGraphStats)
  - Daemon client (callDaemonGraph, isDaemonRunning)
  - Version management (readCanonicalVersion, writeCanonicalVersion, syncVersions, checkVersionDrift)
  - Repo maintenance (auditLicenses, auditReadmes, generateReadmeTemplate)
  - Prompt pipeline (generatePrompt, estimateTokens, DEFAULT_TOKEN_BUDGETS)
  - Knowledge retrieval (retrieveKnowledge, queryOnDemandEntries, countOnDemandEntries, generateOnDemandPreamble)
  - Token tracking (TokenTracker, recordRequest, recordAgentComplete, recordSessionSummary, readMetricEvents, filterEvents, computeTrends)
  - Budget enforcement (BudgetEnforcer, estimateCost, inferModelTier, suggestDowngrade, DEFAULT_BUDGETS, COST_PER_MTOK, MODEL_TIERS)
  - Prompt registry (buildPromptRegistry, readPromptRegistry, runPromptRegistryBuild, queryKnowledge, querySections)
  - Gate engine (startGate, submitVerdict, getOpenGates, getGateSession, clearGateSessions, setAiRecommendation, computeCycleTime)
  - Agent spawner (createAgentConfig, selectModelTier, isValidRole, modelTierLabel, serializeFindings, parseFindingsHeader, UNIVERSAL_ROLES, DEFAULT_MODEL_TIERS, ROLE_TOOL_CONSTRAINTS)

### Source files (57 total in src/)
- **Commands (24):** install, plugin, check, test, build, graph, daemon, mcp, lsp, version, id, migrate, git, dev, summarize, metrics, audit, enforce, link, repo, setup, hosting, validate-schema, index
- **Libraries (24):** config-generator, lockfile, registry, root, symlink, injector-config, version-sync, license, readme, installer, manifest, frontmatter, graph, ports, daemon-client, validation-engine, content-lifecycle, workflow-engine, prompt-registry, knowledge-retrieval, token-tracker, budget-enforcer, agent-spawner, gate-engine, prompt-pipeline, workflow-resolver, agent-file-generator, enforcement-log
- **Tests (1):** prompt-pipeline.integration.ts

### .github/workflows/ (3 files)
- **ci.yml:** Runs on push/PR to main. Steps: checkout, Node 22, npm install, tsc --noEmit, npm run build
- **publish-dev.yml:** Runs on push to main. Publishes to GitHub Packages with `--tag dev`, version set to `0.1.0-dev.{SHA}`
- **publish.yml:** Runs on GitHub release. Validates tag matches package.json version, publishes to GitHub Packages

### .state/
- **orchestrator-preamble.md:** Sample prompt pipeline output showing section counts, token budgets, and session state reminder

---

## 7. Infrastructure

### infrastructure/orqastudio-git/

#### docker-compose.yml
- **Path:** `infrastructure/orqastudio-git/docker-compose.yml` (43 lines)
- Runs Forgejo 10 (`codeberg.org/forgejo/forgejo:10`)
- Container name: `orqastudio`
- Ports: `10030:3000` (HTTP), `10222:22` (SSH)
- SQLite3 database, LFS enabled, Actions enabled, push-create enabled
- Volumes: `orqastudio-data`, `orqastudio-config`

#### setup.sh
- **Path:** `infrastructure/orqastudio-git/setup.sh` (149 lines)
- First-time git server setup:
  1. Creates admin user via `forgejo admin user create` in container
  2. Creates `orqastudio` organisation via API
  3. Creates `orqastudio/app` repo via API
  4. Adds `local` git remote pointing to Forgejo and pushes
  5. Instructions for configuring push mirror to GitHub
  6. Configures branch protection on `main` (PRs required, admin can push)

### infrastructure/sync-bridge/

#### package.json
- **Path:** `infrastructure/sync-bridge/package.json`
- Name: `@orqastudio/sync-bridge`, version `0.1.0`
- Description: "Bidirectional PR/issue/CI-status sync between Forgejo and GitHub"
- Type module, main: `dist/index.js`
- DevDependencies: typescript `^5.7.0`, @types/node `^22.0.0`

#### Dockerfile
- **Path:** `infrastructure/sync-bridge/Dockerfile` (6 lines)
- `node:22-slim`, copies dist/ and runs `node dist/index.js`

#### tsconfig.json
- **Path:** `infrastructure/sync-bridge/tsconfig.json`
- Target ES2022, module Node16, strict, declarations, sourceMaps

#### src/index.ts (216 lines)
- HTTP server with three endpoints:
  - `POST /webhook/github` — receives GitHub webhook events (X-GitHub-Event header)
  - `POST /webhook/forgejo` — receives Forgejo webhook events (X-Forgejo-Event / X-Gitea-Event header)
  - `GET /health` — health check with uptime
- Routes events to sync-pr, sync-issue, sync-status handlers

#### src/config.ts (36 lines)
- `BridgeConfig` interface: port, forgejo (url, token, webhookSecret, org, repo), github (token, webhookSecret, owner, repo)
- Default port: `10402`, default Forgejo URL: `http://localhost:10030`
- All config from environment variables

#### src/github.ts (189 lines)
- GitHub REST API client
- Functions: createIssue, closeIssue, closePullRequest, addComment, createCommitStatus, listPullRequests
- Uses `api.github.com` with `2022-11-28` API version

#### src/forgejo.ts (165 lines)
- Forgejo REST API client
- Functions: createPullRequest, createIssue, closeIssue, addComment, listPullRequests, listIssues
- Uses configurable Forgejo URL with `/api/v1` path

#### src/sync-pr.ts (212 lines)
- PR sync logic:
  - GitHub PR opened: creates corresponding Forgejo PR with `[GitHub PR #N]` marker in title
  - Forgejo PR merged: closes corresponding GitHub PR
  - Sync loop prevention via title markers

#### src/sync-issue.ts (353 lines)
- Issue sync logic:
  - GitHub issue opened: creates Forgejo issue with `[Synced from GitHub #N]` marker
  - GitHub issue closed: finds and closes matching Forgejo issue
  - Forgejo issue opened: creates GitHub issue with `[Synced from Forgejo #N]` marker
  - Forgejo issue closed: finds and closes matching GitHub issue
  - Sync loop prevention via title markers

#### src/sync-status.ts (86 lines)
- CI status sync: forwards Forgejo commit statuses to GitHub as commit statuses
- Maps Forgejo states (success, failure, error, pending, running) to GitHub states
- Context prefixed with `forgejo/`

---

## 8. Integrations

### integrations/claude-agent-sdk/

#### .claude-plugin/plugin.json
- Name: `orqastudio-claude-integration`, version `0.1.0-dev`
- Description: "OrqaStudio Claude integration -- Agent SDK sidecar + governance hooks from @orqastudio/claude-code-cli"

#### hooks/hooks.json
- Empty hooks object: `{ "hooks": {} }`

#### package.json
- Name: `@orqastudio/plugin-claude-integration`, version `0.1.0-dev`, private
- Dependencies: `@orqastudio/cli ^0.1.0-dev`, `@anthropic-ai/claude-agent-sdk latest`

#### .github/workflows/ci.yml
- Validates `orqa-plugin.json` parses as JSON
- Builds sidecar: `cd sidecar && npm install && npm run build`

#### sidecar/
- Contains `node_modules/` with vendored dependencies:
  - `@anthropic-ai/claude-agent-sdk` (SDK with ripgrep vendor binaries for all platforms, tree-sitter WASM files, resvg WASM)
  - `esbuild` (build tool)
  - `sharp` with `@img/sharp-win32-x64` (image processing)
  - `tsx` (TypeScript execution)
  - `get-tsconfig`, `resolve-pkg-maps` (TypeScript helpers)

---

## 9. Models

### models/all-MiniLM-L6-v2/

| File | Size |
|------|------|
| `model.onnx` | 87 MB |
| `tokenizer.json` | 456 KB |

- ONNX model for semantic search (sentence embeddings)
- all-MiniLM-L6-v2: 384-dimensional embeddings, 22M parameters
- Used by `libs/search/` for semantic search functionality

---

## 10. Scripts (repo root)

### Setup/Build Scripts

#### install.sh (111 lines)
- Bootstrap script: zero to `orqa` on PATH
- Ensures Node.js 22+ (via fnm, nvm, or platform package manager)
- Initialises git submodules
- Builds types and CLI libraries, npm links, then runs `orqa install`

#### sync-versions.sh (102 lines)
- Synchronises canonical VERSION across all submodules
- Updates `package.json`, `orqa-plugin.json`, `Cargo.toml`, `plugin.json` in all libs/, app/, plugins/
- Also updates `@orqastudio/*` dependency versions

#### link-all.sh (80 lines)
- Installs deps, builds libs, and npm links everything in dependency order:
  - Tier 1: types
  - Tier 2: cli (depends on types)
  - Tier 3: claude-code connector (depends on types + cli)
  - Tier 4: sdk (depends on types)
  - Tier 5: svelte-components (depends on types)
  - Tier 6: graph-visualiser (depends on types)
  - Tier 7: app (links all libs, runs svelte-kit sync, builds frontend)

#### monorepo-merge.sh (177 lines)
- Merges all 30 submodule repos into a monorepo via `git subtree add`
- Has `--dry-run` flag
- Repos listed in dependency order (7 tiers from leaf libs to meta)
- Steps: verify clean tree, remove all submodules, import each repo preserving history

### Migration/Maintenance Scripts

#### fix-duplicate-frontmatter-keys.mjs (219 lines)
- Fixes duplicate YAML frontmatter keys across `.orqa/` artifacts
- Merges duplicate `relationships:` blocks, deduplicates entries by target+type
- For non-array duplicate keys, keeps last occurrence
- Validates result with yaml.parse()
- `--apply` flag to execute (dry run by default)

#### fix-missing-inverses.mjs (178 lines)
- Adds missing inverse relationships across the artifact graph
- Runs `npx orqa verify` to find MissingInverse errors
- Builds ID-to-filepath index, then adds inverse relationships to target files
- Uses YAML parse/stringify for safe frontmatter manipulation
- Contains complete inverse relationship map (fulfils/fulfilled-by, delivers/delivered-by, etc.)

#### migrate-artifact-ids.mjs (404 lines)
- Bulk migration: sequential IDs (TYPE-NNN) to hex IDs (TYPE-XXXXXXXX)
- Generates random 4-byte hex suffixes
- Updates: id fields, relationship targets, body text references, plugin manifests
- Uses YAML-aware frontmatter parsing (not regex) for safe updates
- Writes migration manifest to `scripts/id-migration-manifest.json`
- `--apply` or `--manifest` flags

#### standardise-ids.mjs (226 lines)
- Removes plugin intermediary prefixes from artifact IDs
- Converts: `KNOW-CLI-3198c8fb` to `KNOW-990e4f85` (strip intermediary, keep hex); `KNOW-CC-decision-tree` to `KNOW-<new hex>` (non-hex suffix)
- Writes manifest to `scripts/id-standardise-manifest.json`

#### link-skills-to-docs.mjs (216 lines)
- Adds `synchronised-with` relationships from skill artifacts to their human-facing documentation
- Also adds inverse relationships on doc files
- Covers: core platform skills, plugin skills (svelte, tauri, rust, software, cli, typescript), project-level skills

### Manifest Files

#### id-migration-manifest.json
- Generated by `migrate-artifact-ids.mjs`, records old-to-new ID mappings
- Large JSON file

#### id-standardise-manifest.json
- Generated by `standardise-ids.mjs`, records intermediary-prefix-stripped ID mappings

---

## 11. Tools (repo root)

### debug/

#### dev.mjs (270 lines)
- OrqaStudio Debug Dashboard server
- Provides web dashboard for inspecting running dev environment
- Features: real-time process status via SSE (polls control file every 2s), log aggregation from frontend, command buttons triggering `orqa dev` CLI commands
- Default port: `10401` (PORT_BASE + 201)
- Endpoints: `GET /` (dashboard HTML), `GET /events` (SSE), `POST /log` (log forwarding), `POST /command/{cmd}` (restart, stop, kill, status), `GET /status` (JSON)
- Reads `tmp/dev-controller.json` control file written by `orqa dev`

#### dev-dashboard.html
- Single-file HTML dashboard (>10k tokens)
- Dark theme, monospace font, inline SVG logo
- Real-time process status display via SSE connection

#### .gitignore
- Ignores `.claude/`

### Migration Tools

#### remove-inverse-relationships.mjs (188 lines)
- Removes stored inverse relationship entries from `.orqa/` frontmatter
- Forward-only storage model: artifacts store forward direction only; graph engine computes inverses at query time
- Removes: grounded-by, documented-by, upheld-by, promoted-from, driven-by, realised-by
- Preserves: served-by (intentionally kept)

#### migrate-types.mjs (101 lines)
- Migration: bare types to stage-scoped types
- `type: idea` to `type: discovery-idea`, `type: research` to `type: discovery-research`, `type: decision` to `type: discovery-decision`
- Only modifies `type:` field in YAML frontmatter

---

## 12. Templates

### registry.json
- **Path:** `templates/registry.json`
- Version 1, defines 4 template types:
  1. **frontend** — "Views and widgets only -- no backend code"
  2. **sidecar** — "AI provider sidecar with streaming and tool execution"
  3. **cli-tool** — "One-shot CLI tools registered as plugin commands"
  4. **full** — "Complete plugin with views, sidecar, CLI tools, and content"

### Template: cli-tool/
- `orqa-plugin.json`: Defines `cliTools` array with runtime, entrypoint, args, category. Provides schemas, views, widgets (all empty), content (rules, knowledge), build command, lifecycle hook
- `src/cli-tool.ts`: TypeScript CLI tool entry point template
- CI workflows: standard CI pipeline

### Template: frontend/
- `orqa-plugin.json`: Defines views array with key, label, icon. Provides defaultNavigation entry. Content: knowledge only. Dependencies: `@orqastudio/sdk`
- `src/MyView.svelte`: Svelte component template
- CI workflows: standard CI pipeline

### Template: full/
- `orqa-plugin.json`: Full plugin manifest with all sections (schemas, views, widgets, relationships, enforcement, cliTools, content, sidecar)
- CI workflows: standard CI pipeline

### Template: sidecar/
- `orqa-plugin.json`: Sidecar plugin manifest with streaming and tool execution config
- `src/sidecar.ts`: TypeScript sidecar entry point template
- CI workflows: standard CI pipeline
