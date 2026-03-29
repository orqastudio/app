# Architecture Review: Plugin System

Reviewer: reviewer agent
Date: 2026-03-29

## Review Summary

**FAIL** -- Multiple architecture gaps across manifest format, enforcement model, composition pipeline, and type definitions.

- PASS: 3
- ARCHITECTURE-GAP: 7

---

## Verdicts

### Component 1: Plugin Directory Taxonomy

**Verdict:** PASS
**Evidence:** plugins/ contains 4 subdirectories matching DOC-41ccf7c4 section 4.5: infrastructure/ (1: coding-standards), knowledge/ (7: cli, plugin-dev, rust, svelte, systems-thinking, tauri, typescript), methodology/ (1: agile-methodology), workflows/ (6: agile-discovery, agile-documentation, agile-planning, agile-review, core, software-kanban).
**Issue:** None.

---

### Component 2: Plugin categories field -- manifest format

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.1 states manifests MUST declare categories as a plural array with values including enforcement-generator and enforcement-contributor. NO plugin manifest uses categories. All plugins use singular category field (single-value enum in PluginManifest type at libs/types/dist/plugin.d.ts:328). The enforcement sub-types appear nowhere.
**Issue:** categories array with enforcement sub-types not implemented. PluginManifest type must add categories: string[]. All plugin manifests must be updated. Manifest validator must enforce if/then category-to-block structural requirements.

---

### Component 3: Manifest JSON Schema -- if/then category-to-block validation

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.1 requires if/then conditional validation enforced by JSON schema -- declaring a category requires the corresponding config block. Actual state: validateManifest() in cli/src/lib/manifest.ts (lines 35-81) checks only name, version, role format, provides structure, non-emptiness. No category-to-block structural validation.
**Issue:** If/then enforcement for enforcement-generator and enforcement-contributor categories is not implemented.

---

### Component 4: Enforcement plugins -- generator model

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.3 requires enforcement-generator plugins to provide: enforcement block with role, engine, generator script, actions, watch, file_types. Actual state: coding-standards, typescript, rust, svelte all have provides.enforcement_mechanisms but NONE have an enforcement block. No generator scripts exist.
**Issue:** All four enforcement plugins are missing the enforcement manifest block. The architecture enforcement-generator model is not implemented.

---

### Component 5: Enforcement plugins -- contributor model

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.3 requires enforcement-contributor plugins to declare enforcement.role: contributor, contributes_to, rules_path, and dependencies on the generator. Actual state: No plugin declares enforcement.contributes_to or enforcement.role: contributor.
**Issue:** The contributor model is not implemented.

---

### Component 6: Plugin cross-package ESLint exports (P1 violation)

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.3: no cross-package imports between plugins. Actual state: plugins/knowledge/typescript/src/eslint/index.ts exports base/recommended as Node.js modules. plugins/knowledge/svelte/src/eslint/index.ts imports { base } from @orqastudio/plugin-typescript/eslint (cross-package import). targets/enforcement/eslint/eslint.config.js imports { svelte } from @orqastudio/plugin-svelte/eslint -- target uses wrong model.
**Issue:** typescript/src/eslint/ and svelte/src/eslint/ must become generator scripts producing self-contained config files, not importable Node.js modules.

---

### Component 7: Daemon file watcher -- plugin registration model

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-41ccf7c4 section 4.3 and DOC-70063f55 section 10.2 require plugins to declare watcher paths in enforcement manifest block; daemon reads these at startup. Actual state (confirmed by plugin-generation-findings.md): No plugin declares watcher registrations. Daemon WATCH_DIRS and RULES_DIR are hardcoded constants. No plugin watcher registration API exists.
**Issue:** Manifest-driven watcher registration is entirely absent.

---

### Component 8: orqa enforce -- dynamic flag dispatch

**Verdict:** ARCHITECTURE-GAP
**Evidence:** DOC-70063f55 section 10.2 requires orqa enforce to build flags dynamically from installed plugin manifests and dispatch to plugin-declared actions. Actual state: cli/src/commands/enforce.ts does NOT read installed manifests to build flags. Flags are static. No dispatch to plugin-declared actions. Command routes only to Rust artifact validator.
**Issue:** Dynamic dispatch to installed enforcement plugins via `orqa enforce --<engine>` is not implemented.

---

### Component 9: Composition pipeline -- methodology/workflow constraint enforcement

**Verdict:** PASS
**Evidence:** cli/src/lib/installer.ts implements enforceOneMethodology() (line 128) and enforceOnePerStage() (line 166). Both throw on conflict and are called during install. Schema recomposition gated on affects_schema, workflow resolution triggered on affects_schema || affects_enforcement. Aligns with DOC-41ccf7c4 section 4.6.
**Issue:** None.

---

### Component 10: Schema composition pipeline (schema-composer.ts)

**Verdict:** PASS
**Evidence:** cli/src/lib/schema-composer.ts reads installed plugin manifests, collects provides.schemas and provides.relationships, composes them into .orqa/schema.composed.json. Reads resolved workflows for state category enrichment. workflow-resolver.ts called after schema composition. Satisfies DOC-41ccf7c4 section 4.8 steps 4-7.
**Issue:** None on schema composition. Gap: enforcement plugin generators do not run at install because generator scripts do not exist.

---

## Blocking Issues

1. categories array field missing from PluginManifest type and all plugin manifests -- the foundational manifest format contract from DOC-41ccf7c4 section 4.1 is not implemented.

2. No enforcement generator model -- plugins coding-standards, rust, typescript, svelte declare enforcement capability via legacy provides.enforcement_mechanisms but none implement the enforcement manifest block.

3. Cross-package ESLint exports are an active P1 violation -- typescript/src/eslint/ and svelte/src/eslint/ export plugin internals as Node.js modules, explicitly prohibited by DOC-41ccf7c4.

4. orqa enforce does not dispatch to plugin engines -- dynamic flag dispatch from installed manifests is not implemented.

5. Daemon watcher is hardcoded -- no plugin-registration API for file watchers; manifest-driven watcher registration is absent.

6. Manifest structural validation (if/then category-to-block) absent -- validateManifest() does not enforce the categories-to-config-block contract.
