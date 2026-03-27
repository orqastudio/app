# 07 - Domain Knowledge & Infrastructure Plugins Inventory

Factual inventory of 9 plugins: cli, rust, svelte, tauri, typescript, coding-standards, systems-thinking, plugin-dev, githooks.

---

## 1. plugins/cli/

**orqa-plugin.json**: `@orqastudio/plugin-cli` v0.1.4-dev, category `tooling`, role `extension`. Provides 5 cliTools (orqa-enforce, orqa-graph-stats, orqa-version-check, orqa-repo-license, orqa-repo-readme), 1 enforcement mechanism (tool, strength 6), 7 knowledge references, 7 knowledge_declarations (all tier on-demand, P3, role implementer), 2 hooks (PreCommit: rebuild CLI + orqa verify; SessionStart: orqa version check). Content maps: documentation -> .orqa/documentation, knowledge -> .orqa/process/knowledge, rules -> .orqa/process/rules. npm dependency: @orqastudio/cli.

**package.json**: `@orqastudio/plugin-cli` v0.1.4-dev, private. Dependencies: @orqastudio/cli 0.1.4-dev, @orqastudio/types 0.1.4-dev. No source code (no src/, no build step).

### Knowledge Files (7)

| File | Name | Summary |
|------|------|---------|
| KNOW-990e4f85.md | OrqaStudio CLI Usage | The `orqa` CLI interface — use instead of raw file operations |
| KNOW-ecc181cb.md | README Standards | Every repo must have a README.md with canonical header and structure |
| KNOW-a0947420.md | CLI Plugin Self-Maintenance | How to maintain and extend the CLI plugin itself |
| KNOW-40be8113.md | License Management | Tiered licensing model, component categories have specific licenses |
| KNOW-481059d2.md | Version Management | Single canonical version across all repos via VERSION file |
| KNOW-afaa4e88.md | Plugin Management | Managing plugins via `orqa plugin` commands |
| KNOW-ea78c8e4.md | Dev Environment Management | Managing the orqastudio-dev environment with git submodules |

### Rules (1)

| File | Title |
|------|-------|
| RULE-f3dca71e.md | Pre-Release Version Tagging — all pre-release versions must use -dev suffix |

### Documentation (5)

| File | Title |
|------|-------|
| DOC-8cf6ef38.md | Dev Environment Setup Guide |
| DOC-af962d42.md | License Policy |
| DOC-db794473.md | OrqaStudio CLI Reference |
| DOC-ecc181cb.md | README Standards |
| DOC-f0a1c9b5.md | Versioning System Guide |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |

---

## 2. plugins/rust/

**orqa-plugin.json**: `@orqastudio/plugin-rust` v0.1.4-dev, category `coding-standards`, role `enhancement:delivery`. Provides 3 enforcement mechanisms (clippy strength 6, rustfmt strength 6, cargo-test strength 6), 2 agents, 4 knowledge refs, 3 tools (clippy, rustfmt, cargo-test), 4 knowledge_declarations (all on-demand, P2-P3, paths **/*.rs), LSP server config for rust-analyzer, decision_tree for rust-backend implementation. System dependency: cargo >= 1.70.0. Content maps: agents, documentation, knowledge.

**package.json**: `@orqastudio/plugin-rust` v0.1.4-dev, private, BSL-1.1 license. No dependencies, no build step.

### Knowledge Files (4)

| File | Name | Summary |
|------|------|---------|
| KNOW-694ff7cb.md | Rust Testing Patterns | Tests in #[cfg(test)] modules, integration tests in tests/ |
| KNOW-d4095bd9.md | Clippy Config Management | clippy/rustfmt config managed through coding standards rules |
| KNOW-ea7898e4.md | Rust Async Patterns | Rust async with Tokio, async traits, error handling, concurrency |
| KNOW-f7d03a2c.md | Rust Plugin Installation | Plugin setup, stable toolchain, post-install assess/configure |

### Agents (2)

| File | Title | Description |
|------|-------|-------------|
| AGENT-065a25cc.md | Rust Specialist | Implementer specialist for Rust backend: thiserror, Result<T,E>, zero unwrap, clippy pedantic |
| AGENT-26e5029d.md | Rust Standards Agent | Scoped task agent for Rust coding standards, assess or configure mode |

### Documentation (1)

| File | Title |
|------|-------|
| DOC-2372ed36.md | Rust Development Guide |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |

---

## 3. plugins/svelte/

**orqa-plugin.json**: `@orqastudio/plugin-svelte` v0.1.4-dev, category `tooling`, role `enhancement:delivery`. Extends @orqastudio/plugin-typescript, requires @orqastudio/plugin-typescript. Provides 3 enforcement mechanisms (eslint strength 6, svelte-check strength 6, vitest strength 6), 2 agents, 16 knowledge refs (13 local + 3 virtual: KNOW-0d6c1ece, KNOW-b5f520d5, KNOW-882d8c4f), 3 tools (eslint, svelte-check, vitest), 16 knowledge_declarations (tiers: stage-triggered P1 for best practices/stores, on-demand P2-P3 for reference), LSP server config for svelteserver, configExtensions for tsconfig and eslint, decision_tree for svelte-frontend. npm deps: @orqastudio/types, @orqastudio/plugin-typescript. Has build step.

**package.json**: `@orqastudio/plugin-svelte` v0.1.4-dev, private, type module. main: dist/index.js. Exports: root, ./eslint, ./test. Scripts: build (tsc), check (tsc --noEmit). Dependencies: @orqastudio/types, @orqastudio/plugin-typescript, svelte-language-server. Peer deps: eslint >=9, svelte-check >=4, vitest >=3, typescript >=5. Dev deps: @sveltejs/vite-plugin-svelte, vite, typescript.

### Source Files (5)

| File | Description |
|------|-------------|
| src/eslint/index.ts | Svelte ESLint config: extends TypeScript base with Svelte-specific rules, no-explicit-any in .svelte files, parser config for .svelte |
| src/test/index.ts | Re-exports baseVitestConfig and svelteVitestConfig from config/ |
| src/test/config/index.ts | Re-exports from vitest.base.js and vitest.svelte.js |
| src/test/config/vitest.base.ts | Base vitest config: v8 coverage provider, 80% thresholds for lines/functions/branches/statements |
| src/test/config/vitest.svelte.ts | Svelte vitest config: extends base with jsdom environment for component testing |

### Knowledge Files (13)

| File | Name | Summary |
|------|------|---------|
| KNOW-50382247.md | Svelte 5 Best Practices | Runes, snippets, SvelteKit patterns, TypeScript, SSR state isolation |
| KNOW-abb08445.md | Svelte 5 Runes Reference | Complete API: $state, $derived, $effect, $props, $bindable, $inspect |
| KNOW-c4d3e52b.md | SvelteKit Patterns Reference | Load functions, page props, form actions, SSR state isolation |
| KNOW-3642842e.md | Testing Patterns | Vitest for unit/integration, @testing-library/svelte for component tests |
| KNOW-37496474.md | Svelte 4 to Svelte 5 Migration | Reactive statements, store-to-runes migration, cheat sheets |
| KNOW-4260613a.md | Svelte 5 Patterns | Component patterns, runes-only, SDK store access, scoped styles |
| KNOW-5704b089.md | Svelte 5 TypeScript Reference | Props typing, generic components, constrained generics |
| KNOW-6cfacbb2.md | ESLint Config Management | Coding standards rules define enforcement entries for this plugin |
| KNOW-8cc0f5e4.md | Svelte 5 Snippets Reference | Slots to snippets migration, @render directive |
| KNOW-96aaa407.md | Svelte 5 Events Reference | Event handler migration, callback props, Context API |
| KNOW-a1a195c1.md | Tailwind Design System | Tailwind CSS v4, design tokens, component libraries |
| KNOW-be54e4de.md | Svelte Plugin Installation | Consumed by core installer agent during setup |
| KNOW-df3c489e.md | Svelte 5 Performance Reference | Universal reactivity, anti-patterns, load optimization, testing |

### Agents (2)

| File | Title | Description |
|------|-------|-------------|
| AGENT-5de8c14f.md | Svelte Specialist | Implementer specialist for Svelte 5 / frontend: runes, shadcn-svelte, strict TypeScript |
| AGENT-6f55de0d.md | Svelte Standards Agent | Scoped task agent, assess or configure mode |

### Documentation (2)

| File | Title |
|------|-------|
| DOC-a06f2a63.md | Svelte Plugin Setup |
| DOC-fd1d12bb.md | Svelte Development Guide |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |
| tsconfig.json | TypeScript config: NodeNext module, strict, outDir dist, rootDir src |

---

## 4. plugins/tauri/

**orqa-plugin.json**: `@orqastudio/plugin-tauri` v0.1.4-dev, category `tooling`, role `enhancement:delivery`. Extends @orqastudio/plugin-rust, requires @orqastudio/plugin-rust and @orqastudio/plugin-svelte. Provides 1 agent, 11 knowledge refs (5 local + 6 virtual: KNOW-8615fee2, KNOW-207d9e2c, KNOW-60aefbbc, KNOW-4f81ddc5, KNOW-33b2dc14, KNOW-fbc200e6), 11 knowledge_declarations (stage-triggered P1 for backend best practices, on-demand P2-P3 for patterns/IPC/errors/services/streaming/repos). Content maps: agents, documentation, knowledge. No source code.

**package.json**: `@orqastudio/plugin-tauri` v0.1.4-dev, private, type module. No dependencies, no build step.

### Knowledge Files (5)

| File | Name | Summary |
|------|------|---------|
| KNOW-1da7ecd8.md | Tauri v2 IPC Patterns Reference | Three IPC primitives: commands, events, channels |
| KNOW-59077955.md | Tauri v2 Development | tauri.conf.json, Rust commands, IPC patterns, deployment |
| KNOW-5efbe925.md | Tauri v2 Capabilities & Permissions | Capabilities-based security model, explicit permission grants |
| KNOW-73490bde.md | Tauri v2 Patterns | All communication via #[tauri::command], no HTTP/WS/shared memory |
| KNOW-a274d90d.md | Tauri Plugin Installation | Consumed by core installer agent during setup |

### Agents (1)

| File | Title | Description |
|------|-------|-------------|
| AGENT-65b56a0b.md | Tauri Standards Agent | Extends Rust Standards Agent with Tauri-specific knowledge |

### Documentation (2)

| File | Title |
|------|-------|
| DOC-13c73ecf.md | Tauri Development Guide |
| DOC-9505a5b5.md | Tauri Plugin Setup |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |

---

## 5. plugins/typescript/

**orqa-plugin.json**: `@orqastudio/plugin-typescript` v0.1.4-dev, category `coding-standards`. No role field. Provides 1 enforcement mechanism (tsc, strength 6), 2 knowledge refs, LSP server config for typescript-language-server, 3 tsconfig presets (base, library, app), 2 eslint presets (base, recommended), 1 knowledge_declaration (on-demand P3 for advanced types). Content maps: documentation, knowledge. npm deps: @typescript-eslint/eslint-plugin, @typescript-eslint/parser, typescript-eslint, typescript-language-server. Has build step.

**package.json**: `@orqastudio/plugin-typescript` v0.1.4-dev, type module. NOT private (publishable). main: dist/index.js. Exports: root, ./tsconfig/base.json, ./tsconfig/library.json, ./tsconfig/app.json, ./eslint. Scripts: build (tsc), dev (tsc --watch), check (tsc --noEmit). Published files: dist, src/tsconfig. Repository: git@github.com:orqastudio/orqastudio-plugin-typescript.git. License: BSL-1.1.

### Source Files (2 TS + 3 JSON configs)

| File | Description |
|------|-------------|
| src/index.ts | Main entry: re-exports ESLint configs, defines TsconfigExtension/EslintExtension/ConfigExtensions interfaces for plugin config composition |
| src/eslint/index.ts | Base TypeScript ESLint config: recommended rules, no-explicit-any error, ban @ts-ignore, unused vars allow underscore prefix, no-console error (except workers/logger), test file relaxation |
| src/tsconfig/base.json | Base strict config: strict, noUncheckedIndexedAccess, esModuleInterop, skipLibCheck, forceConsistentCasingInFileNames, isolatedModules |
| src/tsconfig/library.json | Library preset: extends base, ES2022 target, NodeNext module, declaration+declarationMap+sourceMap |
| src/tsconfig/app.json | App preset: extends base, ES2022 target, ESNext module, bundler resolution, DOM libs, verbatimModuleSyntax, noEmit |

### Knowledge Files (1)

| File | Name | Summary |
|------|------|---------|
| KNOW-40e2eb99.md | TypeScript Advanced Types | Generics, conditional types, mapped types, template literals, utility types |

### Documentation (1)

| File | Title |
|------|-------|
| DOC-7062bce9.md | TypeScript Plugin Skills — advanced type patterns reference |

### CI/CD (3 GitHub Actions workflows)

| File | Description |
|------|-------------|
| .github/workflows/ci.yml | CI: checkout, node 22, npm ci, npm run check on push/PR to main |
| .github/workflows/publish.yml | Publish Release: on GitHub release, build + npm publish to GitHub Packages |
| .github/workflows/publish-dev.yml | Publish Dev: on push to main, build, set dev version with SHA, npm publish --tag dev |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |
| tsconfig.json | Build config: NodeNext, strict, outDir dist, rootDir src |

---

## 6. plugins/coding-standards/

**orqa-plugin.json**: `@orqastudio/plugin-coding-standards` v0.1.4-dev, category `coding-standards`, role `enhancement:delivery`. Provides 1 enforcement mechanism (lint, strength 6), 2 knowledge refs, 2 cliTools (check: orqa check, configure: orqa check configure), 2 knowledge_declarations (on-demand P2). Content maps: knowledge, rules. npm deps: @orqastudio/cli, @orqastudio/types. Has build step.

**package.json**: `@orqastudio/plugin-coding-standards` v0.1.4-dev, private, type module. main: dist/index.js. Scripts: build (tsc), check (tsc --noEmit). Dependencies: @orqastudio/cli, @orqastudio/types. Dev deps: @types/node, typescript. Has package-lock.json.

### Source Files (3)

| File | Description |
|------|-------------|
| src/index.ts | Main entry: re-exports ConfigGenerator, CheckRunner and their types |
| src/check-runner.ts | CheckRunner class (136 lines): discovers tools from plugin manifests (provides.tools), runs each via execSync with 120s timeout, returns aggregated CheckSummary with pass/fail counts |
| src/config-generator.ts | ConfigGenerator class (235 lines): reads enforcement rules from .orqa/process/rules/ YAML frontmatter, discovers plugin tools, groups entries by tool, generates tool-specific config files (JSON or TOML format) |

### Knowledge Files (2)

| File | Name | Summary |
|------|------|---------|
| KNOW-126aa140.md | Quality Check Runner | Unified quality checks via orqa check |
| KNOW-1c2d005d.md | Config Generation from Rules | Generate tool configs from OrqaStudio enforcement rules |

### Rules (8)

| File | Title |
|------|-------|
| RULE-216e112e.md | Lint Enforcement Discipline — standards reflected in automated linting |
| RULE-42d17086.md | Tooling Ecosystem Management — linter config matches documented standards |
| RULE-5dd9decd.md | Honest Reporting — report status accurately |
| RULE-83411442.md | Tooltips over title attributes — use shadcn Tooltip, not HTML title |
| RULE-8cb4bd04.md | Testing Standards — test org, coverage, mock boundaries, isolation |
| RULE-97e96528.md | Root Directory Cleanliness — root must stay lean |
| RULE-c382e053.md | No Aliases or Hacks — fix root causes, no shims |
| RULE-eb269afb.md | Reusable Components — check shared library before creating new UI |

### Other Files

| File | Description |
|------|-------------|
| .gitignore | Git ignore patterns |
| CHANGE-LICENSE | License change notice |
| thumbnail.png | Plugin thumbnail image |
| tsconfig.json | Extends ../../plugins/typescript/src/tsconfig/library.json, outDir dist, rootDir src |
| package-lock.json | npm lockfile |

---

## 7. plugins/systems-thinking/

**orqa-plugin.json**: `@orqastudio/plugin-systems-thinking` v0.1.4-dev, category `thinking`, role `core:discovery`. Provides 11 knowledge refs, 11 knowledge_declarations covering: thinking modes (P0 always: Research, Planning, Documentation, Dogfood Implementation), methodologies (P1-P2: composability, systems thinking, diagnostic, restructuring, architectural evaluation), supplementary (P3: artifact relationships, tech debt). Content maps: knowledge, rules. No source code, no build step, no agents, no enforcement mechanisms.

**package.json**: `@orqastudio/plugin-systems-thinking` v0.1.4-dev, private. No dependencies, no scripts.

### Knowledge Files (11)

| File | Name | Tier/Priority |
|------|------|---------------|
| KNOW-0619a413.md | Composability | on-demand P2 |
| KNOW-1ea9291c.md | Artifact Relationships | on-demand P3 |
| KNOW-36befd20.md | Thinking Mode: Research | always P0 |
| KNOW-41849545.md | Systems Thinking | on-demand P2 |
| KNOW-4a4241a5.md | Thinking Mode: Planning | always P0 |
| KNOW-7fadba3f.md | Architectural Evaluation | on-demand P2 |
| KNOW-8564d52c.md | Diagnostic Methodology | stage-triggered P1 (debug stage) |
| KNOW-a3dcdd05.md | Restructuring Methodology | on-demand P2 |
| KNOW-bf70068c.md | Thinking Mode: Documentation | always P0 |
| KNOW-d13d80e1.md | Tech Debt Management | on-demand P3 |
| KNOW-eeceaabf.md | Thinking Mode: Dogfood Implementation | always P0 |

### Rules (3)

| File | Title |
|------|-------|
| RULE-05562ed4.md | Pillar Alignment in Documentation — every doc page must include pillar alignment section |
| RULE-1b238fc8.md | Vision Alignment — every feature must serve at least one active pillar |
| RULE-43f1bebc.md | Systems Thinking First — every change evaluated as part of whole system |

### Other Files

None beyond knowledge/, rules/, orqa-plugin.json, and package.json. No .gitignore, no CHANGE-LICENSE, no thumbnail.

---

## 8. plugins/plugin-dev/

**orqa-plugin.json**: `@orqastudio/plugin-plugin-dev` v0.1.4-dev, category `development`, role `enhancement:development`. Provides 1 agent, 3 knowledge refs, 3 knowledge_declarations (P2-P3 on-demand). Content maps: agents, knowledge. No source code, no build step, no enforcement mechanisms, no rules, no cliTools, no hooks. No package.json present.

### Knowledge Files (3)

| File | Name | Summary |
|------|------|---------|
| KNOW-1b7fa054.md | Third-Party Plugin Development | Standalone plugin creation outside platform monorepo |
| KNOW-2f38309a.md | Plugin Development | Base knowledge: auto-detects first-party vs third-party context, manifest structure, content ownership, lifecycle, scaffolding templates, Artifact Graph SDK |
| KNOW-e6fee7a0.md | First-Party Plugin Development | Dev environment workflow: submodules, managed by dev CLI, published via CI |

### Agents (1)

| File | Title | Description |
|------|-------|-------------|
| AGENT-ce86fb50.md | Plugin Developer | Develops and maintains plugins — scaffolding, manifest management, installation, testing |

### Other Files

None beyond knowledge/, agents/, and orqa-plugin.json.

---

## 9. plugins/githooks/

**orqa-plugin.json**: `@orqastudio/plugin-githooks` v0.1.4-dev, category `enforcement`, role `enhancement:governance`. Provides 1 enforcement mechanism (pre-commit, strength 7 -- highest of any plugin), 3 hooks (pre-commit: validate artifacts, post-commit: auto-push, PreAction: validate relationships), 1 cliTool (hooks-install), behavioral_rules (relationship types must match plugin manifests), session_reminders (relationship types are schema-driven). Content maps: none defined. npm dep: fast-glob.

**package.json**: `@orqastudio/plugin-githooks` v0.1.4-dev, private, BSL-1.1 license. Dependencies: fast-glob ^3.3.3. Has package-lock.json.

### Hook Scripts (4)

| File | Description |
|------|-------------|
| hooks/pre-commit | Bash script (113 lines): detects staged file types (Rust/frontend/artifacts), runs targeted checks via `orqa check` per file type (rustfmt+clippy for .rs, eslint for .svelte/.ts/.js, validate for .orqa/*.md). Logs violations to .state/precommit-violations.jsonl for stability tracking. Falls back to node CLI if orqa binary not found. |
| hooks/post-commit | Bash script (67 lines): auto-pushes to remote after every commit (RULE-f609242f enforcement). Sets upstream on first push. Skips during rebase/merge/cherry-pick/revert. Warns on failure but does not block. |
| hooks/install.sh | Bash script (30 lines): sets git config core.hooksPath to plugin hooks directory, checks for existing hooks path conflicts, chmod +x on hooks. |
| hooks/validate-relationships.mjs | Node script (186 lines): validates relationship type fields in artifact frontmatter. Delegates to orqa-validation daemon at localhost:10258. Two modes: pre-commit (file args) and hook (stdin JSON for Write/Edit tool interception). Exit codes: 0=valid, 1=errors (pre-commit), 2=block (hook). |

### Other Files

| File | Description |
|------|-------------|
| package-lock.json | npm lockfile for fast-glob |

---

## Summary Statistics

| Plugin | Category | Role | Knowledge | Rules | Agents | Docs | Source Files | Has Build |
|--------|----------|------|-----------|-------|--------|------|--------------|-----------|
| cli | tooling | extension | 7 | 1 | 0 | 5 | 0 | No |
| rust | coding-standards | enhancement:delivery | 4 | 0 | 2 | 1 | 0 | No |
| svelte | tooling | enhancement:delivery | 13 | 0 | 2 | 2 | 5 | Yes |
| tauri | tooling | enhancement:delivery | 5 | 0 | 1 | 2 | 0 | No |
| typescript | coding-standards | (none) | 1 | 0 | 0 | 1 | 5 | Yes |
| coding-standards | coding-standards | enhancement:delivery | 2 | 8 | 0 | 0 | 3 | Yes |
| systems-thinking | thinking | core:discovery | 11 | 3 | 0 | 0 | 0 | No |
| plugin-dev | development | enhancement:development | 3 | 0 | 1 | 0 | 0 | No |
| githooks | enforcement | enhancement:governance | 0 | 0 | 0 | 0 | 4 (hooks) | No |

**Total across all 9 plugins**: 46 knowledge files, 12 rules, 6 agents, 11 docs, 17 source/hook files.

### Plugin Dependency Chain

```
typescript  <-- svelte  <-- tauri (also requires svelte)
                         ^
                         |
            rust --------+
```

- `svelte` extends and requires `typescript`
- `tauri` extends `rust` and requires both `rust` and `svelte`
- `coding-standards` depends on `cli` and `types` (npm)
- `cli` depends on `cli` and `types` (npm)
- `githooks` depends on `fast-glob` (npm, external)
- `systems-thinking`, `plugin-dev` have no npm dependencies

### Publishable vs Private

- **Publishable (to GitHub Packages)**: `typescript` (has repository URL, license, CI/CD workflows, `files` field)
- **Private**: All others (marked `private: true` or no package.json)

### Virtual Knowledge References

Some plugins declare knowledge IDs in their `provides.knowledge` that do NOT have corresponding files in their own `knowledge/` directory. These are "virtual" references — the content files live elsewhere (typically generated at install time or provided by other plugins):

- **svelte**: KNOW-0d6c1ece (Frontend Best Practices), KNOW-b5f520d5 (Store Patterns), KNOW-882d8c4f (Store Orchestration)
- **tauri**: KNOW-8615fee2 (Backend Best Practices), KNOW-207d9e2c (Error Composition), KNOW-60aefbbc (Domain Services), KNOW-4f81ddc5 (IPC Patterns), KNOW-33b2dc14 (Streaming Pipeline), KNOW-fbc200e6 (Repository Pattern)
