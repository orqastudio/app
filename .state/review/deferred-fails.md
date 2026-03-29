# Deferred FAILs — Requiring Architectural Decisions or Structural Work

**Date:** 2026-03-29
**Context:** Per-phase review found 84 FAILs. ~45 are being fixed in this session. The remaining ~20 unique issues (counted multiple times across phases) are deferred below with rationale.

---

## 1. ESLint Enforcement Model (P1-S2-01, P10-3.1)

**Issue:** Both the current `eslint.config.js` and the target use the wrong pattern per the updated architecture docs (commit 65ab12b5b). The target imports from `@orqastudio/plugin-svelte/eslint` (a cross-package export). The architecture says enforcement plugins should provide **generators**, not package exports.

**Correct fix:** Create ESLint generator infrastructure — plugin provides a generator tool that composes ESLint config from rules. Generated output goes to `.orqa/configs/eslint.config.js`.

**Why deferred:** Requires new generator interface in plugin manifest schema, daemon watcher registry for rule-change-triggered regeneration, and generator implementations in TypeScript and Svelte plugins. This is future work documented in `.state/review/plugin-generation-findings.md`.

**Tracked by:** IDEA-f3a9b7c4 (Governance-enforceable patterns scan)

---

## 2. Schema-Driven Frontend Configs (P9 — 9.2.2 through 9.2.9, 8 FAILs)

**Issue:** 8 frontend config files (`frontmatter-config.ts`, `category-config.ts`, `sort-orders.ts`, `lesson-stages.ts`, `governance-types.ts`, `action-labels.ts`, `search-config.ts`, `relationship-icons.ts`) are static TypeScript modules. The ACs require them to be "derived from plugin registry/composed schema at runtime."

**Current state:** Values extracted from components into shared config modules (correct intermediate step). Not yet connected to plugin registry runtime.

**Why deferred:** Accepted as interim per IDEA-a3f7c912. The extraction step is architecturally correct — single source, importable. Runtime derivation requires: (a) plugin registry providing these values in manifests, (b) a schema→config pipeline in the frontend, (c) reactive updates when plugins change. This is significant frontend infrastructure work.

**Resolution path:** When plugin manifest format supports UI config declarations, create a `schema-to-config` pipeline that replaces these static files.

---

## 3. PLATFORM_NAVIGATION Hardcoding (P9-9.3.1)

**Issue:** `libs/types/src/project.ts:244-283` hardcodes `Principles`, `Discovery`, `Learning` as navigation groups. These appear regardless of plugin installation. AC requires methodology stages to come from the installed methodology plugin.

**Why deferred:** Making navigation fully plugin-driven requires: (a) methodology plugin declaring its stages in manifest, (b) navigation tree builder consuming manifest declarations, (c) removing PLATFORM_NAVIGATION hardcoded groups. This touches the core navigation pipeline.

**Resolution path:** When plugin manifests gain `navigation_contributions` block, rebuild navigation tree from plugin declarations.

---

## 4. Connector Generation Gaps (P10 — 12 FAILs)

**Issue:** The connector generator (`connectors/claude-code/src/generator.ts`) doesn't produce all target outputs:

- Missing agents: designer.md, orchestrator.md
- Missing frontmatter: model/tools/maxTurns in generated agents
- Missing hooks: TeammateIdle, TaskCompleted events
- Wrong hook paths: dist/hooks/*.js vs scripts/*.mjs
- Missing directories: scripts/, skills/
- Missing files: plugin.json, settings.json, architecture/ files

**Why deferred:** The target Claude Code plugin structure (`targets/claude-code-plugin/`) defines the complete output. The generator needs significant template additions to produce all these files. This is Phase 10 completion work — the generator must be enhanced to match all targets before targets/ can be removed.

**Resolution path:** Enhance generator.ts with templates for each missing output. Add agent frontmatter generation from manifest declarations. Add script and skill generation. Create plugin.json and settings.json generation.

---

## 5. prompt-registry.json Deletion (P6-6.27)

**Issue:** `.orqa/prompt-registry.json` still exists. AC says delete it. But `daemon/src/knowledge.rs` Layer 1 reads this file at runtime for knowledge injection.

**Why deferred:** Deleting the file breaks the daemon's knowledge layer. The replacement is the prompt pipeline (daemon/src/knowledge.rs already has Layer 2 and 3 using ONNX semantic search). Layer 1 (declared knowledge from prompt-registry.json) needs to be replaced with Layer 1 using plugin manifest `knowledge_declarations` directly.

**Resolution path:** Update `daemon/src/knowledge.rs` to read knowledge declarations from installed plugin manifests instead of prompt-registry.json. Then delete the file.

---

## 6. Workflow Format (P1-S4-02)

**Issue:** AC says target resolved workflows should be `.resolved.json` (JSON format). Actual files are `.resolved.yaml` (YAML format).

**Why deferred:** Architecture principle P7 says "Resolved Workflow Is a File" and P4 says "Declarative Over Imperative — state machines, guards, and workflows are YAML declarations." YAML is the correct format per the architecture. The AC was written before this was clarified.

**Resolution:** AC should be updated to accept YAML. No code change needed.

---

## 7. Target Manifest name vs title (P1-S4-03)

**Issue:** AC says target plugin manifests should use `title` instead of `name`. All 16 manifests use `name`.

**Why deferred:** The `name` field is the npm-style package identifier (e.g., `@orqastudio/plugin-agile-methodology`). The `title` field is for display purposes. The schema has `name` as required. Both fields can coexist. The AC confused `name` (package identifier) with `title` (display name).

**Resolution:** AC should be clarified. `name` is correct for the package identifier. `title` can be added as an optional display field.

---

## 8. MCP Spec Deviation (P3-S3-01)

**Issue:** Task spec says MCP should start as part of daemon startup. Implementation correctly does NOT manage MCP from daemon (MCP uses stdio transport, incompatible with daemon management).

**Why deferred:** This is an intentional architectural improvement. `daemon/src/mcp.rs` documents the full rationale. The task spec was written before the design decision was resolved.

**Resolution:** Update task spec to reflect the correct MCP architecture (client-managed, not daemon-managed).

---

## 9. LSP Status in System Tray (P3-S4-01)

**Issue:** System tray doesn't show LSP server status.

**Why deferred:** Minor feature gap. Core tray functionality (icon, quit, open-app) is present. LSP process is managed by the daemon but status isn't surfaced in the tray menu.

**Resolution:** Add LSP status indicator to tray context menu in `daemon/src/tray.rs`.

---

## 10. githooks Plugin (P5-17)

**Issue:** Task references `plugins/githooks/orqa-plugin.json` but the plugin was deleted as a zombie (commit 169f95660).

**Why deferred:** The githooks plugin was intentionally deleted during remediation. The target manifest at `targets/plugin-manifests/githooks.orqa-plugin.json` references a plugin that no longer exists.

**Resolution:** Remove the target manifest, or recreate the plugin if git hooks functionality should be a plugin (rather than a built-in feature).

---

## 11. RoadmapView Epic Hierarchy (P9-9.6.2)

**Issue:** `RoadmapView.svelte` renders milestones only, without child epic hierarchy. AC requires milestone→epic hierarchy from graph relationships.

**Why deferred:** Feature work requiring graph relationship traversal in the frontend component. The graph SDK has the capability but the component doesn't use it.

**Resolution:** Add epic query via `artifactGraphSDK.relationships()` for each milestone.

---

## 12. orqa check schema Errors (P5-37)

**Issue:** `orqa check schema` exits with 2 errors about missing plugin dependencies (`@orqastudio/plugin-agile-methodology` and `@orqastudio/plugin-systems-thinking` not installed).

**Why deferred:** These are dev-checkout dependency errors, not manifest format issues. In a real installation, these would be installed.

**Resolution:** Install the dependent plugins in the dev checkout, or add a dev-mode flag that skips dependency checks.

---

## 13. NavigationSettings Dynamic Sections (P9-9.4.1 partial)

**Issue:** While NavigationSettings.svelte can be removed (being fixed), the ACs for plugin-driven settings sections and dynamic plugin settings pages cannot be met with current infrastructure.

**Why deferred:** Requires plugin manifest support for settings page declarations and a dynamic section builder in SettingsCategoryNav.

**Resolution:** When plugin manifests support `settings_pages` declarations, wire them into the settings navigation.

---

## 14. Target Plugin.json Completeness (P1-S4-01)

**Issue:** `targets/claude-code-plugin/plugin/.claude-plugin/plugin.json` only has name/description/version/author. Missing commands/hooks/skills/resources.

**Why deferred:** This target file needs to be completed to match the full plugin structure. The hooks and skills directories exist but aren't registered in plugin.json. This requires understanding the final target structure.

**Resolution:** Complete the plugin.json with references to the existing hooks and skills files.

---

## Summary

| Category | FAILs | Status |
|----------|-------|--------|
| Being fixed this session | ~64 | In progress |
| Deferred — generator infrastructure | ~3 | Future work |
| Deferred — frontend plugin integration | ~12 | IDEA-a3f7c912 |
| Deferred — connector enhancement | ~12 | Phase 10 completion |
| Deferred — spec/AC corrections | ~4 | Spec update needed |
| Deferred — minor features | ~3 | Backlog |
