# Phase 2 Gap Analysis: Root Config, Libs, Infrastructure, and Misc

Compares current inventory (Phase 1 files `01-root-config.md` and `10-app-libs-infra-misc.md`) against `ARCHITECTURE.md`.

---

## 1. `.state/team/` Accumulation (214 files, 36 directories)

### Current State

`.state/team/` contains 36 team directories with 214 findings files, all from sessions on 2025-03-25 and 2025-03-26. These are agent team outputs: task findings, migration scripts, audit data, research reports. Some contain large data files (328 KB JSON audit data, 79 KB migration manifest). One directory (`fix-sot/`) is empty.

### Gap

There is **no cleanup policy**. The `.state/` directory is gitignored and treated as operational data (per AD-8727f99a), but nothing prunes old team findings. These files serve as historical evidence of what work was done and what agents found, but after the orchestrator reads them and the work is committed, their primary value is forensic.

### Recommendation

1. **Immediate:** Delete the empty `fix-sot/` directory.
2. **Policy:** `TeamDelete` should archive or remove team findings after the orchestrator commits. The CLI should provide `orqa dev clean-teams [--age <days>]` to prune team directories older than a threshold.
3. **Guideline:** Findings files are ephemeral working state. They should NOT accumulate indefinitely. If a finding contains something worth preserving, it should be promoted to a governance artifact (lesson, decision, knowledge) before the team is deleted.
4. **Large data files:** Migration manifests and audit JSON in `.state/team/` are one-shot outputs. Consider writing them to `.state/migrations/` with explicit naming rather than burying them in team directories where they are hard to find later.

---

## 2. `.claude/agents/` vs Methodology Plugin Base Roles

### Current State

**`.claude/agents/`** contains 6 role definitions (Claude Code agent format):
- `implementer.md` -- code implementation
- `reviewer.md` -- quality verification
- `researcher.md` -- investigation
- `writer.md` -- documentation
- `governance-steward.md` -- `.orqa/` artifact maintenance
- `planner.md` -- approach design

**`plugins/agile-workflow/agents/`** contains 2 agent artifacts (governance artifact format):
- `AGENT-7a06d10e.md` -- "Governance Enforcer" (designs/implements enforcement mechanisms)
- `AGENT-ae63c406.md` -- "Governance Steward" (`.orqa/` artifact maintenance specialist)

### Gap: Dual Representation

The `.claude/agents/` files are Claude Code's native subagent definitions. The `plugins/agile-workflow/agents/` files are OrqaStudio governance artifacts with YAML frontmatter, relationship links, and model assignments. These serve different purposes but overlap in content.

**Architecture says:** Base roles should live in the methodology plugin. Task-specific agents are generated at runtime by the engine's prompt pipeline. Fixed agent definitions in `.claude/agents/` are legacy -- they should be the **output** of the connector generating from the methodology plugin, not hand-maintained.

### Specific Gaps

| Issue | Detail |
|-------|--------|
| **Role count mismatch** | `.claude/agents/` has 6 roles. ARCHITECTURE.md defines 5 base roles (Orchestrator, Implementer, Reviewer, Researcher, Writer). CLAUDE.md lists 9 universal roles (adds Designer, Installer, Plugin Developer, Governance Steward). The methodology plugin has 2 specialist agents. No single authoritative list. |
| **Governance Steward appears 3 times** | As `.claude/agents/governance-steward.md`, as `AGENT-ae63c406.md` in the methodology plugin, and as a row in CLAUDE.md's role table. Content differs across all three. |
| **Planner absent from ARCHITECTURE.md** | `.claude/agents/planner.md` exists but Planner is not one of the 5 base roles in ARCHITECTURE.md section 6.1. It IS listed in CLAUDE.md's role table. |
| **Enforcer in plugin but not in .claude/agents/** | `AGENT-7a06d10e.md` (Governance Enforcer) exists in the methodology plugin but has no corresponding `.claude/agents/` definition. |
| **No generation pipeline** | The `.claude/agents/` files are hand-maintained. The connector should be generating them from base role definitions in the methodology plugin + workflow context. This pipeline does not exist yet. |

### Recommendation

1. **Consolidate the authoritative role list** in ARCHITECTURE.md. Currently there are 3 conflicting lists (ARCHITECTURE.md: 5 roles, CLAUDE.md: 9 roles, `.claude/agents/`: 6 roles).
2. **The methodology plugin should own base role definitions.** The `provides.agents` field in `agile-workflow/orqa-plugin.json` should contain the base roles, not specialist agents.
3. **`.claude/agents/` should become a generated output** of the connector, not a hand-maintained source. Until the generation pipeline exists, these files are the operational truth, but they should be treated as temporary.
4. **Specialist agents** (like the Enforcer) are not base roles -- they are task-specific compositions that the engine should generate from Base Role + Domain Knowledge at delegation time.

---

## 3. Root Config Files (Cargo.toml, package.json, Makefile)

### Cargo.toml

**Current:** 5 workspace members: `libs/validation`, `libs/search`, `libs/mcp-server`, `libs/lsp-server`, `app/src-tauri`.

**Architecture says:** The engine is a "standalone Rust crate -- an independent process, not embedded in any consumer." Currently, the engine capabilities (graph, workflow, validation, enforcement) are split across `libs/validation` (30 source files, the largest crate) and consumed by `app/src-tauri`, `libs/mcp-server`, and `libs/lsp-server`.

### Gap

| Issue | Detail |
|-------|--------|
| **No standalone engine crate** | ARCHITECTURE.md describes a single "engine" crate that provides graph, workflow, state machine, prompt pipeline, search, enforcement, plugin system, and agent generation. In reality, these capabilities are distributed: validation/graph/enforcement in `libs/validation` (Rust), workflow/prompt-pipeline/agent-generation in `libs/cli` (TypeScript), search in `libs/search` (Rust). There is no unified "engine" crate. |
| **Business logic in TypeScript CLI** | 32 CLI source files reference graph, workflow, state machine, prompt pipeline, or enforcement. The CLI contains `workflow-engine.ts`, `workflow-resolver.ts`, `prompt-pipeline.ts`, `prompt-registry.ts`, `knowledge-retrieval.ts`, `gate-engine.ts`, `agent-spawner.ts`, `budget-enforcer.ts`, `graph.ts`, `validation-engine.ts`, `enforcement-log.ts`. Architecture says business logic belongs in the engine, not in consumers. The CLI is a consumer, yet it implements core engine functionality. |
| **Rust/TypeScript split** | The "engine" is half Rust (`libs/validation` for parsing, schema validation, graph structure, daemon) and half TypeScript (`libs/cli` for workflow resolution, prompt generation, knowledge retrieval, agent generation). This is a pragmatic split but contradicts the "standalone Rust crate" architecture. |

### package.json

**Current:** 25 npm workspaces. Single devDependency (`yaml`).

**Gap:** The workspace list is accurate for the current structure but includes packages that may not belong at the monorepo level (e.g., `libs/graph-visualiser` is an app-internal lib, not a shared infrastructure lib).

### Makefile

**Current:** Bootstrap-only with single `install` target pointing to `scripts/install.sh`.

**Gap:** None. The Makefile correctly defers to the CLI for all commands. This is well-positioned.

### Recommendation

1. **Acknowledge the Rust/TypeScript engine split** in ARCHITECTURE.md or create a plan to consolidate. The current state works but the architecture document describes a target that does not exist.
2. **The CLI contains engine-level business logic.** Either: (a) the architecture should be updated to recognize the CLI as an engine co-host rather than a consumer, or (b) the TypeScript business logic should migrate into Rust (major effort).
3. **Cargo workspace is correctly scoped** for the current reality.

---

## 4. libs/cli -- Engine Consumer or Engine Co-Host?

### Current State

57 source files, 16 primary commands. The CLI exports a substantial library API (via `src/index.ts`) covering plugin management, graph browsing, daemon communication, version management, prompt pipeline, knowledge retrieval, token tracking, budget enforcement, prompt registry, gate engine, agent spawner.

### Gap: Boundary Violation

**Architecture says** (section 3.1): "The engine is consumed by multiple frontends: the app (via Tauri), the CLI, the MCP server, and the LSP server. None of these consumers ARE the engine -- they all consume it."

**Reality:** The CLI IS the engine for TypeScript-domain concerns. The prompt pipeline, workflow engine, knowledge retrieval, gate engine, and agent spawner are all implemented in the CLI, not in a standalone engine that the CLI consumes. Other consumers (the connector, app scripts) import from `@orqastudio/cli` to access these capabilities.

### Specific Concerns

| Concern | Detail |
|---------|--------|
| **CLI version hardcoded** | `cli.ts:28` prints `0.1.0-dev` but `package.json` says `0.1.4-dev`. Version drift. |
| **Circular dependency risk** | The connector (`@orqastudio/claude-code`) depends on the CLI. The CLI is also the tool the user runs. If the CLI has a bug, both the user and the connector are affected. |
| **Library exports are engine exports** | `src/index.ts` exports 80+ functions covering graph, prompt, workflow, agent, and enforcement concerns. This is an engine API surface, not a CLI utility surface. |
| **Legacy aliases still present** | 8 legacy command aliases (`setup`, `link`, `verify`, `audit`, `enforce`, `repo`, `hosting`, `index`, `log`) route to new locations. Per architecture: "No backwards compatibility -- pre-release, breaking changes expected." These can be removed. |

### Recommendation

1. **Split `libs/cli` into `libs/engine` (TypeScript) + `libs/cli`** -- extract the library functions (graph, prompt pipeline, workflow engine, knowledge retrieval, agent spawner, gate engine) into a separate `@orqastudio/engine` package. The CLI becomes a thin consumer of this engine package.
2. **Remove legacy aliases** -- the architecture says no backwards compatibility during pre-release.
3. **Fix version string** in `cli.ts:28` to read from `package.json` dynamically or match the canonical version.

---

## 5. libs/brand

### Current State

Design token library (`tokens.json`) + icon generation script. Dependencies: `sharp`. Private package. Contains color definitions (oklch), typography (Inter, JetBrains Mono), spacing, and radius tokens.

### Gap

**None.** `libs/brand` is correctly positioned as a shared design infrastructure library. It:
- Provides design tokens consumed by the app
- Generates icons deployed to `app/src-tauri/icons/` and `app/static/`
- Has no business logic
- Is private (not published)

### Minor Issues

| Issue | Detail |
|-------|--------|
| **Script path mismatch** | `package.json` references `scripts/generate-icons.mjs` but the actual file is at `libs/brand/generate-icons.mjs` (no `scripts/` subdirectory). Needs verification. |
| **Not listed in Cargo workspace** | Correct -- it is TypeScript-only, so no Cargo entry needed. |

---

## 6. infrastructure/

### 6.1 infrastructure/orqastudio-git/

**Current:** Docker Compose for a self-hosted Forgejo instance + setup script. Ports 10030 (HTTP), 10222 (SSH).

**Gap:** Actively used. The CI workflows in `.forgejo/workflows/` target this Forgejo instance. The setup script handles first-time configuration and GitHub push mirror setup. **No gap** -- this is correctly positioned as local development infrastructure.

### 6.2 infrastructure/sync-bridge/

**Current:** Bidirectional PR/issue/CI-status sync between Forgejo and GitHub. HTTP server (3 webhook endpoints + health check). 8 source files, ~1,200 lines.

**Gap:**

| Issue | Detail |
|-------|--------|
| **Version is 0.1.0, not 0.1.4-dev** | Out of sync with the rest of the monorepo. |
| **Port conflict documented but unresolved** | TASK-088e20b7 documents that sync-bridge defaults to port 3001, conflicting with the dev dashboard. The target port is 10402 (per EPIC-9e3d320b), but this fix may not have been applied. |
| **No evidence of active deployment** | The sync-bridge has a Dockerfile and source code but no reference in the Makefile, `orqa dev`, or any startup script. It appears to be written but not yet deployed. |
| **Not in npm workspace** | `infrastructure/sync-bridge` is not listed in the root `package.json` workspaces. This is likely intentional (it is infrastructure, not a library) but means `npm install` at root does not install its dependencies. |

### Recommendation

1. **Sync version** to `0.1.4-dev` with the rest of the monorepo.
2. **Resolve port conflict** (apply TASK-088e20b7 if not already done).
3. **Decide deployment strategy** -- either integrate into `docker-compose.yml` alongside Forgejo or document as "available but optional" infrastructure.

---

## 7. integrations/claude-agent-sdk/

### Current State

Contains a sidecar implementation for the Claude Agent SDK: 6 source files implementing a provider interface for LLM inference via the Claude Agent SDK. Has vendored `node_modules` containing the full Agent SDK (including ripgrep binaries, tree-sitter WASM, resvg WASM, esbuild, sharp).

Also contains a `.claude-plugin/plugin.json` (OrqaStudio plugin metadata) and empty `hooks/hooks.json`.

### Gap

| Issue | Detail |
|-------|--------|
| **Version mismatch** | `0.1.0-dev` vs monorepo `0.1.4-dev`. |
| **Vendored node_modules in source** | The sidecar has vendored `node_modules/` with large binary dependencies (ripgrep platform binaries, WASM files, sharp native modules). This bloats the repository. These should be installed at build time, not committed. |
| **Empty hooks** | `hooks/hooks.json` has `{ "hooks": {} }` -- placeholder with no implementation. |
| **Naming confusion** | Located at `integrations/claude-agent-sdk/` but the `plugin.json` calls itself `orqastudio-claude-integration`. The `package.json` calls itself `@orqastudio/plugin-claude-integration`. Three different names for one thing. |
| **Relationship to connector** | Architecture says sidecars provide LLM inference TO the app. This integration provides inference via the Claude Agent SDK, which maps to the sidecar concept. However, the connector (`connectors/claude-code/`) generates config FOR Claude Code. These are parallel concepts (per ARCHITECTURE.md section 3.3). The distinction is correct but the naming (`integrations/` vs `connectors/`) obscures it. |

### Recommendation

1. **Move to `sidecars/claude-agent-sdk/`** to match the architecture's vocabulary. The `integrations/` category adds no clarity.
2. **Remove vendored `node_modules`** -- these should be installed via `npm install`, not committed. Add to `.gitignore`.
3. **Align version** to `0.1.4-dev`.
4. **Standardize naming** -- pick one name and use it everywhere.

---

## 8. scripts/ -- Classification

### Ongoing Operations

| Script | Status | Recommendation |
|--------|--------|----------------|
| `install.sh` | **Active** -- the only Makefile target runs this | Keep. Entry point for `make install`. |
| `sync-versions.sh` | **Active** -- synchronizes VERSION across all submodules | Should become `orqa version sync` (the CLI command may already exist). Check for duplication with `libs/cli/src/lib/version-sync.ts`. |
| `link-all.sh` | **Active** -- builds and links everything in dependency order | Should become `orqa install link` or already is. Check for duplication with the CLI's install command. |

### One-Time Migration (Completed)

| Script | Status | Recommendation |
|--------|--------|----------------|
| `monorepo-merge.sh` | **Completed** -- merged submodules into monorepo | **Archive or delete.** The merge is done. Keeping it as documentation of how the monorepo was created is reasonable but it is no longer executable (the submodule repos would need to exist). |
| `migrate-artifact-ids.mjs` | **Completed** -- migrated sequential IDs to hex IDs | **Delete.** Migration is done. `id-migration-manifest.json` documents the mappings if needed forensically. |
| `standardise-ids.mjs` | **Completed** -- removed plugin intermediary prefixes | **Delete.** Migration is done. `id-standardise-manifest.json` documents the mappings. |
| `fix-duplicate-frontmatter-keys.mjs` | **Completed** -- fixed duplicate YAML keys | **Delete.** One-time fix. |
| `fix-missing-inverses.mjs` | **Likely obsolete** -- adds inverse relationships, but architecture uses forward-only storage | **Delete.** Contradicts the forward-only relationship storage design decision. |
| `link-skills-to-docs.mjs` | **Completed** -- added synchronised-with relationships | **Delete** if the relationships are now in place. |

### Manifest Files (One-Time Migration Output)

| File | Recommendation |
|------|----------------|
| `id-migration-manifest.json` | Move to `.state/migrations/` or delete. Not needed at repo root. |
| `id-standardise-manifest.json` | Move to `.state/migrations/` or delete. Not needed at repo root. |

### Summary

Of 11 files in `scripts/`:
- **3 are active** (install.sh, sync-versions.sh, link-all.sh) -- 2 may be duplicated by CLI commands
- **6 are completed one-time migrations** -- should be deleted
- **2 are manifest outputs** -- should be moved or deleted

---

## 9. tools/ -- Classification

### tools/debug/

| File | Status | Recommendation |
|------|--------|----------------|
| `dev.mjs` | **Active** -- debug dashboard server, referenced by `orqa dev` | Should become a CLI subcommand (`orqa dev dashboard`) if not already. |
| `dev-dashboard.html` | **Active** -- the dashboard UI | Stays with `dev.mjs`. |
| `.gitignore` | Ignores `.claude/` | Fine. |

### tools/ root

| File | Status | Recommendation |
|------|--------|----------------|
| `remove-inverse-relationships.mjs` | **One-time migration** -- removes stored inverses for forward-only model | **Delete.** Migration completed. |
| `migrate-types.mjs` | **One-time migration** -- converts bare types to stage-scoped | **Delete.** Migration completed. |

### Summary

Of 5 files in `tools/`:
- **3 are active** (debug dashboard + gitignore)
- **2 are completed one-time migrations** -- should be deleted

---

## 10. templates/ -- Plugin Architecture Alignment

### Current State

4 templates: `frontend`, `sidecar`, `cli-tool`, `full`. Each provides an `orqa-plugin.json`, source scaffolding, and CI workflows.

### Gaps

| Issue | Detail |
|-------|--------|
| **Content paths use `process/` prefix** | `full/orqa-plugin.json` line 48-53 uses `target: ".orqa/process/rules"` and `target: ".orqa/process/knowledge"`. ARCHITECTURE.md section 5.1 says: "No `process/` nesting -- artifact categories are top-level within `.orqa/`." Target paths should be `.orqa/rules/` and `.orqa/knowledge/<category>/`. |
| **Missing template types** | No templates for: methodology plugin, workflow plugin, knowledge-only plugin, infrastructure plugin, connector plugin. The registry only covers 4 of the 7 plugin purposes defined in ARCHITECTURE.md section 4.1. |
| **CI workflows assume submodule repos** | Templates include `.github/workflows/` with `publish-dev.yml` and `release.yml` that assume GitHub Packages publication from independent repos. After the monorepo merge, these are obsolete for first-party plugins. Still relevant for third-party plugin developers. |
| **`CHANGE-LICENSE` at templates root** | `templates/CHANGE-LICENSE` (Apache 2.0 + Ethical Use Addendum) is the license template for generated plugins. Correctly positioned. |

### Recommendation

1. **Update content paths** in all templates to remove the `process/` prefix.
2. **Add templates for methodology, workflow, knowledge, infrastructure, and connector plugins** -- or document that these template types are planned.
3. **Keep CI workflows** -- they are correct for third-party plugin development, which is the primary use case for templates.

---

## 11. app/.githooks/ -- Relationship to githooks Plugin

### Current State

`app/.githooks/` contains 15 validation scripts (1 orchestrator + 14 validators), all `.mjs` files directly in the app submodule. The `githooks` plugin (`plugins/githooks/`) contains 4 hooks: `install.sh`, `pre-commit`, `post-commit`, `validate-relationships.mjs`.

### Gap: Two Parallel Hook Systems

| Aspect | app/.githooks/ | plugins/githooks/ |
|--------|---------------|-------------------|
| **Files** | 15 (orchestrator + 14 validators) | 4 (installer + 3 hooks) |
| **Format** | Direct `.mjs` scripts | Plugin-declared hooks in `orqa-plugin.json` |
| **Scope** | Full enforcement suite (schema, links, status transitions, pillar alignment, relationships, history, config consistency, stubs, lint suppression, task deps, epic readiness, core graph protection, plugin source protection) | Schema validation, relationship validation, post-commit auto-push |
| **Architecture alignment** | **Legacy** -- hardcoded enforcement scripts in the app | **Target** -- plugin-provided hooks generated from engine rules |

### Analysis

The `app/.githooks/` scripts are the **operational enforcement suite** -- they are what actually runs on pre-commit. The `plugins/githooks/` hooks are a **partial reimplementation** moving toward the target architecture. The plugin has only 3 of the 15 enforcement checks.

**Architecture says** (section 10.1): "Git hooks -- generated by plugins from engine rules." The current `app/.githooks/` scripts are hand-written, not generated. The githooks plugin should eventually subsume all 15 checks, generating them from engine rule definitions.

### Recommendation

1. **app/.githooks/ is the operational truth.** Do not delete it until the githooks plugin can generate equivalent enforcement.
2. **The githooks plugin needs to grow** to cover all 15 validation checks. Each check should correspond to an engine rule that the plugin translates into a hook script.
3. **Document this as a known legacy/target gap** -- the transition path is: engine rules -> githooks plugin generates hooks -> replaces hand-written app/.githooks/.

---

## 12. app/scripts/ and app/tools/

### app/scripts/

| File | Status | Recommendation |
|------|--------|----------------|
| `rebuild-artifacts.mjs` (715 lines) | **Completed one-time migration** -- reads from `.orqa-backup/`, transforms frontmatter, writes to new `.orqa/` structure | **Delete.** Migration is done. |
| `migration-manifest.json` (~79k tokens) | **Output of above migration** | **Delete or move to `.state/migrations/`.** |
| `link-all-plugins.mjs` | **Possibly active** -- symlinks plugin directories for local dev | Check if superseded by `orqa plugin link`. If so, **delete**. |

### app/tools/

16 files. Key concern: several of these implement validation and enforcement logic that the architecture says belongs in the engine.

| File | Architecture Alignment |
|------|----------------------|
| `lib/parse-artifact.mjs` | Delegates to Rust binary -- **correct**. |
| `verify-pipeline-integrity.mjs` | Implements governance enforcement logic -- **should be in engine**. Currently called by pre-commit hook. |
| `check-orientation.mjs` | Session utility -- **fine as a tool**. Could become `orqa dev orientation`. |
| `summarize-artifact.mjs` | Knowledge summarization -- may duplicate `orqa summarize`. |
| `verify-installed-content.mjs` | Plugin content verification -- **should be in engine** (`orqa check verify`). |
| `lint-relationships.mjs` | Relationship validation -- **should be in engine** or githooks plugin. |

### Recommendation

1. **Delete completed migration scripts** from `app/scripts/`.
2. **Migrate enforcement logic** from `app/tools/` into the engine (or CLI as interim engine co-host). `verify-pipeline-integrity.mjs`, `verify-installed-content.mjs`, and `lint-relationships.mjs` implement engine-level business logic.
3. **Check for CLI duplicates** -- `summarize-artifact.mjs` may duplicate `orqa summarize`, `link-all-plugins.mjs` may duplicate `orqa plugin link`.

---

## 13. VERSION and VERSIONS Files

### Current State

| File | Content | Purpose |
|------|---------|---------|
| `VERSION` (root) | `0.1.4-dev` | Canonical version source |
| `VERSIONS` (root) | BSL-1.1 release history template | Change-date schedule for BSL license conversions |
| `VERSIONS` (app) | Lists version-bearing files + change-date table | App submodule's version file list + BSL schedule |

### Gap

| Issue | Detail |
|-------|--------|
| **Naming confusion** | `VERSION` (singular) is the current version. `VERSIONS` (plural) is the BSL change-date schedule. These serve completely different purposes but their names suggest they are related. |
| **Duplication between root and app** | Both `VERSIONS` files contain BSL change-date schedules. Root `VERSIONS` has no releases listed. App `VERSIONS` has `0.1.0-pre` with dates. These should be a single source of truth. |
| **Root VERSIONS vs app VERSIONS content differs** | Root `VERSIONS` is a comment-only template with no actual entries. App `VERSIONS` has a markdown table with one entry (`0.1.0-pre, 2026-03-16, 2030-03-16`). |

### Recommendation

1. **Keep `VERSION` at root** -- canonical version source, used by `scripts/sync-versions.sh` and `orqa version sync`.
2. **Merge `VERSIONS` into one** -- the BSL change-date schedule should be at root only (the app is a submodule of the monorepo, and the license is monorepo-scoped). Delete `app/VERSIONS` or convert it to just list version-bearing files (its other purpose).
3. **Consider renaming** `VERSIONS` to `RELEASE-SCHEDULE` or `CHANGE-DATE-SCHEDULE` to eliminate naming confusion.

---

## 14. validation_stderr.txt (Root)

### Current State

19 KB of JSON-formatted validation output at repo root. Contains `BrokenLink` errors for artifact references that don't resolve.

### Gap

This is debug output that should not be at the repo root. It is gitignored (not tracked), but it clutters the working directory.

### Recommendation

Move to `.state/validation/` or configure the validation daemon to write to `.state/` by default.

---

## Summary of All Gaps

### Critical (Architecture Violations)

| # | Gap | Impact |
|---|-----|--------|
| 1 | **No standalone engine crate** -- engine functionality split between `libs/validation` (Rust) and `libs/cli` (TypeScript) | Core architecture claim does not match reality |
| 2 | **CLI contains engine business logic** -- workflow engine, prompt pipeline, knowledge retrieval, agent spawner all in CLI | CLI is an engine co-host, not a consumer |
| 3 | **Three conflicting role lists** -- ARCHITECTURE.md (5), CLAUDE.md (9), .claude/agents/ (6) | No authoritative role definition |
| 4 | **app/.githooks/ is hand-written, not generated** -- 15 enforcement scripts that architecture says should come from plugins | Enforcement is hardcoded, not plugin-composed |

### Significant (Structural Issues)

| # | Gap | Impact |
|---|-----|--------|
| 5 | **.claude/agents/ are hand-maintained** -- should be connector output | Agent definitions not generated from engine |
| 6 | **Template content paths use obsolete `process/` prefix** | New plugins would install to wrong locations |
| 7 | **214 team findings with no cleanup policy** | `.state/team/` grows unboundedly |
| 8 | **sync-bridge version drift + unresolved port conflict** | Infrastructure out of sync |
| 9 | **integrations/claude-agent-sdk has vendored node_modules** | Repository bloat |

### Minor (Cleanup Needed)

| # | Gap | Impact |
|---|-----|--------|
| 10 | **8 completed migration scripts** in `scripts/` and `tools/` | Dead code |
| 11 | **app/scripts/rebuild-artifacts.mjs** -- 715-line completed migration | Dead code |
| 12 | **Dual VERSIONS files** with different content | Confusion about BSL schedule source |
| 13 | **validation_stderr.txt at repo root** | Clutter |
| 14 | **CLI version hardcoded as `0.1.0-dev`** in cli.ts | Version drift |
| 15 | **5 missing template types** | Cannot scaffold methodology, workflow, knowledge, infrastructure, or connector plugins |
| 16 | **Legacy CLI aliases** still present despite no-backwards-compatibility policy | Unnecessary code |

---

## Open Questions

| # | Question | Why Unresolved |
|---|----------|---------------|
| 1 | **Should the TypeScript engine logic migrate to Rust?** | Major architectural decision. The current Rust/TypeScript split is pragmatic but contradicts ARCHITECTURE.md. Migrating would be a large effort. Updating the architecture to reflect the dual-language reality may be more appropriate for pre-release. |
| 2 | **Should `libs/cli` be split into `libs/engine` + `libs/cli`?** | Depends on answer to Q1. If the TypeScript engine logic stays, extracting it into a separate package would better match the architecture's consumer/engine separation. |
| 3 | **What is the timeline for the githooks plugin to subsume app/.githooks/?** | The gap between hand-written hooks and plugin-generated hooks is large (15 checks vs 3). This is a known target-state gap that requires planning. |
| 4 | **Is sync-bridge actively deployed or aspirational?** | No references in startup scripts or development workflow. Could be deployed infrastructure that is managed outside the codebase, or could be unfinished work. |
