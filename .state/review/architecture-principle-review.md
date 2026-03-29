# Architecture-Principle Review

**Date:** 2026-03-29
**Reviewed against:** Architecture docs post-commit 65ab12b5b (enforcement plugin model, categories, orqa enforce)
**Commit reviewed:** e730f08ec (64 FAIL fixes)
**Method:** 5 parallel reviewer agents, each with fresh context, reviewing against architecture principles (not AC checklists)

---

## Executive Summary

| Area | PASS | ARCHITECTURE-GAP | ACCEPTED-INTERIM | PLANNED | SILENT |
|------|------|-------------------|------------------|---------|--------|
| Engine + Daemon | 13 | 2 blocking + 2 flags | — | — | — |
| Plugin System | 3 | 7 | — | — | — |
| Frontend | 7 | 3 | 1 | — | — |
| Connector + CLI | 4 | 4 | — | 1 | — |
| .orqa/ Structure | 7 | 4 | — | — | — |
| **Total** | **34** | **20 blocking + 2 flags** | **1** | **1** | **0** |

**Overall verdict:** The core engine architecture is sound (13/17 PASS). The main gaps cluster around the **enforcement plugin model** (not yet implemented), the **language boundary** (CLI + connector still TypeScript), and the **frontend extensibility** model (artifact viewer not plugin-overridable, governance types hardcoded).

---

## BLOCKING Architecture Gaps (Priority Order)

### 1. Enforcement Generator Model Not Implemented

**Affects:** Plugin system, daemon watcher, pre-commit hook, orqa enforce
**Principle violated:** P1 (Plugin-Composed Everything)
**References:** DOC-41ccf7c4 section 4.3, DOC-70063f55 section 10.2

No plugin implements the enforcement generator contract. The architecture requires:

- Plugin declares `enforcement` block with `role`, `engine`, `generator`, `actions`, `watch`, `file_types`

- `orqa plugin install` calls generator → produces composed config in `.orqa/configs/`

- Daemon watcher reads `enforcement.watch` from manifests → triggers regeneration on rule changes

- `orqa enforce --staged` dispatches to all installed plugin `actions.check`

**Current state:**

- Plugins use legacy `provides.enforcement_mechanisms` field (no generator)

- TypeScript/Svelte plugins export ESLint config as Node.js modules (P1 violation)

- Daemon watcher has hardcoded `WATCH_DIRS` and `RULES_DIR`

- Pre-commit hook hardcodes tool invocations (`npx eslint`, `npx markdownlint-cli2`)

- `orqa enforce` exists in code but is not wired as a top-level CLI command

**Resolution:** Implement the full enforcement plugin contract as documented in DOC-41ccf7c4.

### 2. `categories` Array Field Missing from Manifest Schema

**Affects:** Plugin system, manifest validation
**Principle violated:** P1
**Reference:** DOC-41ccf7c4 section 4.1

DOC-41ccf7c4 requires `categories: string[]` with enforcement sub-types (`enforcement-generator`, `enforcement-contributor`). No plugin manifest uses this field. The `PluginManifest` type only has singular `category`. The if/then category-to-config-block structural validation in the manifest schema is absent.

**Resolution:** Add `categories` array to PluginManifest type. Update all manifests. Implement category-to-block validation.

### 3. GateCondition Closed Enum (P1 + P4 Violation)

**Affects:** Engine workflow crate
**Principle violated:** P1 (hardcoded governance), P4 (imperative, not declarative)
**File:** `engine/workflow/src/gates.rs:32-47`

`GateCondition` is a Rust enum with 5 hardcoded variants encoding specific process gate logic. A plugin cannot declare a new condition type without changing Rust source. Should be string-based dispatch with params map.

**Resolution:** Replace enum with `String` condition identifier + `HashMap<String, Value>` params. Dispatch by string match with open fallthrough.

### 4. Tracker Hardcoded Verification Commands

**Affects:** Engine workflow crate
**Principle violated:** P1
**File:** `engine/workflow/src/tracker.rs:169-175`

`record_command()` hardcodes 6 verification command substrings. Should come from `TrackerConfig.verification_patterns`.

**Resolution:** Move to config with current values as defaults.

### 5. Connector Language Boundary Violation

**Affects:** Connector, language boundary
**Principle violated:** DOC-62969bc3 section 3.1 (Rust below, TypeScript frontend-only)
**Reference:** Plans at `.state/team/remediation-critical/plan-connector-rust.md`

Connector is TypeScript calling daemon HTTP. Architecture requires Rust calling engine crates directly. CLI is also TypeScript (planned Rust migration exists).

**Status:** PLANNED — migration plans exist for both CLI and connector.

### 6. Frontend Governance Types Hardcoded (P1 Violation)

**Affects:** Frontend dashboard components
**Principle violated:** P1
**File:** `app/src/lib/config/governance-types.ts`

`ARTIFACT_TYPES`, `ARTIFACT_STATUSES`, `RELATIONSHIP_TYPES` hardcoded as static constants. Dashboard components import directly. Plugin-contributed governance vocabulary not reflected.

**Resolution:** Derive from `pluginRegistry.allSchemas` at runtime, same pattern as MarkdownLink/ArtifactViewer.

### 7. Artifact Viewer Not Plugin-Extensible

**Affects:** Frontend artifact viewer (three-pillar architecture)
**Principle violated:** Plugin-extendable artifact viewing
**File:** `app/src/lib/components/content/ArtifactViewer.svelte`

No mechanism for plugins to register custom viewers for specific artifact types. ExplorerRouter sends all artifacts to ArtifactViewer unconditionally. Plugin registry has no `getViewerForArtifactType()` API.

**Resolution:** Add viewer registration to plugin manifest. Plugin registry provides type→viewer lookup. ExplorerRouter checks for custom viewer before falling back to default ArtifactViewer.

### 8. Hardcoded Watch Paths (Daemon + Connector)

**Affects:** Daemon watcher, connector watcher
**Principle violated:** P1 — watch paths should come from plugin manifests
**Files:** `daemon/src/watcher.rs` (WATCH_DIRS), `connectors/claude-code/src/watcher.ts` (WATCH_PATTERNS)

Both daemon and connector hardcode their watch directories instead of reading from installed plugin manifests.

**Resolution:** Plugin manifests declare watch paths. Daemon reads at startup. Connector inherits from daemon config.

### 9. Pre-commit Hook Hardcodes Tools

**Affects:** Git hooks, enforcement
**Principle violated:** P1 — enforcement dispatch should be `orqa enforce --staged`
**File:** `.githooks/pre-commit`

Hook directly invokes `npx eslint`, `npx markdownlint-cli2` instead of routing through `orqa enforce --staged`. When a new enforcement plugin is installed, the hook doesn't automatically dispatch to it.

**Resolution:** Replace hardcoded tool invocations with `orqa enforce --staged`. Wire `orqa enforce` as top-level CLI command.

### 10. Cross-Package ESLint Exports

**Affects:** TypeScript and Svelte plugins
**Principle violated:** P1 — plugins should provide generators, not package exports
**Files:** `plugins/knowledge/typescript/src/eslint/index.ts`, `plugins/knowledge/svelte/src/eslint/index.ts`

These export ESLint config as Node.js modules for cross-plugin import. Architecture requires generators that produce self-contained config files.

**Resolution:** Replace exports with generator tools. Remove src/eslint/ from plugins.

---

## Lower-Severity Gaps

### 11. `@orqastudio/graph-visualiser` Not Exposed to Plugins

Plugin `window.__orqa` globals map doesn't include the graph visualiser. Plugins building graph views would fail at runtime.

**Fix:** Add to `shared-modules.ts` and plugin vite config template.

### 12. No Plugin Developer Documentation

The plugin view contract (window.__orqa shape, mount function signature, vite externals config) is implicit. No README or developer guide exists.

**Fix:** Create developer guide in `libs/sdk/`.

### 13. BaseRole Enum in engine/agent/types.rs

8 hardcoded structural role variants. These appear to be infrastructure (structural coordination roles), not governance patterns. Should be documented explicitly as infrastructure vs governance.

### 14. Missing knowledge/ Subdirectories

`documentation/guides/`, `documentation/platform/`, `documentation/project/` (81 DOC files) have no `knowledge/` subdirectory. DOC-fd3edf48 requires one in every documentation topic directory. Not all DOCs need KNOW counterparts, but the directories should exist.

### 15. Missing implementation/ideas/ Directory

DOC-fd3edf48 section 5.1 explicitly requires this directory. Does not exist.

### 16. Workflow Naming: methodology.resolved.yaml

Spec says `methodology.resolved.yaml` but actual file is `agile-methodology.resolved.yaml`. The actual name is correct — it reflects the installed methodology. Spec needs updating.

### 17. Per-Type Resolved Workflows Not in Spec

`workflows/` contains 29 files including per-artifact-type workflows (epic.resolved.yaml, task.resolved.yaml, etc.). DOC-fd3edf48 only documents stage-level workflows. Per-type workflows are correct architecture — spec needs updating.

---

## Accepted Interim

### Static Frontend Config Files (IDEA-a3f7c912)

8 config files (`frontmatter-config.ts`, `category-config.ts`, `sort-orders.ts`, `lesson-stages.ts`, `governance-types.ts`, `action-labels.ts`, `search-config.ts`, `relationship-icons.ts`) are static TypeScript modules, not derived from plugin registry at runtime. Extraction from components was the correct intermediate step. Runtime derivation requires plugin manifest support for UI config declarations.

---

## PASS Areas (No Action Needed)

- Engine types: pure data shapes, no governance patterns

- Engine enforcement: scanner config-driven, rule evaluation generic

- Engine workflow transitions: string-based dispatch, open condition set

- Engine plugin: manifest read/validate, install constraints correct

- Engine prompt: knowledge injection config-driven

- Engine validation: graph checks generic

- Engine search: ONNX-based, no external dependencies

- Daemon main/config/health/tray: correct architecture

- Daemon MCP (client-managed, documented rationale)

- Plugin directory taxonomy matches DOC-41ccf7c4

- Plugin install constraints (one-methodology, one-per-stage-slot)

- Schema composition from plugin manifests

- Frontend StatusBar, ActivityBar, ArtifactViewer ID regex, MarkdownLink (all plugin-registry-driven)

- Frontend plugin registry: fully reactive, provides allSchemas/allRelationships

- Frontend RULE-006 enforcement

- Frontend shared SDK: `@orqastudio/sdk` and `@orqastudio/svelte-components` published, plugins consume correctly

- Frontend PluginViewContainer: correct shared module loading

- Frontend three-pillar: ecosystem management and AI assistance are first-class

- Connector hooks: all thin adapters, no business logic

- Connector generator: generation/file-writing only

- .orqa/ structure: top-level directories correct, no legacy directories

---

## Three-Pillar Frontend Architecture Assessment

| Pillar | Verdict | Notes |
|--------|---------|-------|
| Ecosystem Management | PASS | PluginBrowser + settings give full control. Install, configure, enable/disable plugins works. |
| AI Assistance | PASS | Chat panel is first-class. RULE-006 compliant, state via SDK stores, sidecar integration through Tauri IPC. |
| Artifact Viewer | ARCHITECTURE-GAP | Default viewer works but is NOT plugin-extendable. No type→viewer registration, no plugin override mechanism. |

The SDK/component library architecture is sound: `@orqastudio/sdk` and `@orqastudio/svelte-components` are consumed by both core app and plugins. The core app eats its own dog food. Gap: graph-visualiser not exposed to plugins.

---

## Items Requiring User Clarification

1. **`categories` vs `category`:** DOC-41ccf7c4 documents `categories: string[]` with enforcement sub-types. Current code uses singular `category: string` enum. Is the plural form the correct target, or should the doc be updated to match the simpler singular form?

2. **Artifact viewer extensibility:** Should plugins be able to register custom viewers for specific artifact types (e.g., a kanban board for tasks, a timeline for milestones)? This is implied by the three-pillar architecture but not explicitly documented.

3. **BaseRole enum:** The 8 structural roles (Orchestrator, Implementer, Reviewer, etc.) are hardcoded in Rust. Are these infrastructure constants (like HTTP methods) or governance patterns that should be plugin-declared?

4. **Per-type resolved workflows:** 22 per-artifact-type workflows exist beyond the 7 stage-level workflows documented in DOC-fd3edf48. Should the spec be updated, or should these be removed?

---

## Detailed Findings Files

- `.state/team/reviewer/arch-engine-daemon.md`

- `.state/team/reviewer/arch-plugin-system.md`

- `.state/team/reviewer/arch-frontend.md`

- `.state/team/reviewer/arch-connector-cli.md`

- `.state/team/reviewer/arch-orqa-structure.md`
