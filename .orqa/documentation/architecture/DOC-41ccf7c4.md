---
id: DOC-41ccf7c4
type: doc
status: active
title: Plugin Architecture
domain: architecture
description: Plugin purposes, taxonomy, composition pipeline, methodology vs workflow distinction, and installation constraints
created: 2026-03-28T00:00:00.000Z
---

# Plugin Architecture

> This is part of the OrqaStudio Architecture Reference.

---

## 4. Plugin Architecture

### 4.1 Plugin Purposes

A plugin serves one or more of these purposes:

| Purpose | What It Provides | Effect at Install Time |
| --------- | ----------------- | ---------------------- |
| **Methodology definition** | The overarching workflow skeleton and stage slots | Triggers full schema/workflow recomposition |
| **Workflow definition** | A complete sub-workflow for one methodology stage | Triggers full schema/workflow recomposition |
| **Knowledge/rules** | Domain expertise, behavioral constraints, documentation | Assets installed. No schema recomposition, but rule changes trigger enforcement regeneration. |
| **App extension** | Custom views, UI components | Assets installed, no recomposition |
| **Sidecar** | LLM provider integration for the app | Provides inference capability |
| **Connector** | Generation pipeline for a third-party tool | Generates + watches for regeneration |
| **Config generator** | Generator tool, `check`/`fix` commands, file watcher declarations, KNOW/DOC artifacts for the standards | Runs generator → composed config under `.orqa/configs/`. Registers watcher. Declares commands. Installs KNOW/DOC artifacts. |
| **Config contributor** | Rule/standards data that feeds a generator, plus KNOW/DOC artifacts for the contributed domain | Installs data to `.orqa/learning/`. Installs KNOW/DOC artifacts. Triggers generator re-run. |

A single plugin may serve multiple purposes. The `orqa-plugin.json` manifest is the single source of truth for what each plugin provides and what installation actions are needed. Manifests must declare: `categories` (array — all areas the plugin participates in), stage slot (if workflow), content types provided, and installation targets.

`categories` is a plural array — a plugin appears in every category it declares. Valid values: `methodology`, `workflow`, `domain-knowledge`, `enforcement-generator`, `enforcement-contributor`, `connector` (and others as the system grows). Enforcement has two distinct sub-types rather than a single `enforcement` value — a plugin can declare one or both.

The TypeScript plugin declares `"categories": ["domain-knowledge", "enforcement-contributor"]` — it provides domain knowledge AND contributes enforcement rules. The ESLint plugin can declare `"categories": ["enforcement-generator", "enforcement-contributor"]` — it owns the generator AND contributes its own base rules. The frontend tags and filters plugins by every declared category.

**Manifest schema validation — categories must match config blocks:**

The JSON schema for `orqa-plugin.json` enforces structural consistency using `if/then` conditional validation: **declaring a category in `categories` requires the corresponding configuration block to be present**. You cannot claim a capability without providing its configuration.

| Category declared | Required config block | Additional constraint |
| ----------------- | --------------------- | --------------------- |
| `"enforcement-generator"` | `enforcement` block with `role: generator`, `engine`, `generator`, `actions`, `watch`, `file_types` | — |
| `"enforcement-contributor"` | `enforcement` block with `role: contributor`, `contributes_to`, `rules_path` | `dependencies` must include the generator plugin named in `contributes_to` |
| `"domain-knowledge"` | `knowledge_declarations` block | — |
| `"workflow"` | `workflows` block with `stage_slot` | — |
| `"methodology"` | `methodology` block | — |
| `"connector"` | `connector` block | — |

This is validated structurally by the JSON schema — not at runtime, not by convention. `orqa install` rejects a manifest that declares a category without the matching config block. The schema enforces the contract before any installation action runs.

**Contributor dependency requirement:** An `enforcement-contributor` MUST declare a plugin dependency on the generator it contributes to. The contributor's rules are meaningless without the generator that consumes them. `orqa install` warns (or fails) if the declared generator dependency is not installed.

### 4.2 Methodology and Workflows

This distinction is fundamental to composability:

**Methodology** = the overarching approach (e.g., Agile). Defined by a methodology plugin. The methodology plugin provides the workflow skeleton with named contribution points (slots) that workflow plugins fill. One and only one methodology plugin may be installed per project.

**Workflows** = the sub-workflows within the methodology. Each workflow plugin provides the complete sub-workflow for one stage of the methodology — including its own state machine (states, transitions, guards, gates) and artifact types. Each workflow is self-contained with no inheritance from other plugins. One workflow plugin per stage defined in the methodology.

Relationships define the flow both **within** a workflow (e.g., task delivers epic, research informs decision) and **between** workflows and the methodology (e.g., discovery outputs feed planning inputs, implementation delivers against planning). The graph engine computes inverses from forward-only declared relationships, making the entire methodology traceable end-to-end.

### 4.3 Config Composition Pattern (Generator + Contributors)

This is the **universal enforcement model** for OrqaStudio. Every mechanical check follows the same pipeline regardless of what the tool does — linting, formatting, type checking, grammar, accessibility, security scanning, license compliance, link checking, or anything else. The enforcement engine doesn't matter. The pattern is always the same.

**Enforcement has two complementary layers:**

- **Knowledge injection** — agents learn the standards before they write code. KNOW artifacts from enforcement plugins are injected into agent prompts so agents produce compliant output from the start. The check should rarely fail if knowledge injection is working.
- **Mechanical check** — the safety net that catches anything that slipped through. `orqa enforce` runs after agent work and at pre-commit.

The check is the safety net. Knowledge is the guidance. Both are required.

An **enforcement plugin** is any plugin that declares an `enforcement` section in its manifest. There are two sub-types — a plugin can be one or both:

- **`enforcement-generator`** — owns an enforcement engine: provides the generator script, actions, watcher, config output. Responsible for one output file under `.orqa/configs/`.
- **`enforcement-contributor`** — provides rule files that feed into a generator. Must declare a `dependencies` entry on the generator plugin it contributes to.

A plugin can declare both (`["enforcement-generator", "enforcement-contributor"]`) — for example, the ESLint plugin owns the generator AND contributes its own base rules. A plugin can also combine enforcement with other categories — for example, `typescript` is `["domain-knowledge", "enforcement-contributor"]`.

**The full enforcement contract — when a plugin declares enforcement, it must provide ALL of:**

1. **Generator** — translates rules into tool-specific config under `.orqa/configs/`
2. **Rule contribution files** — rule/standards data installed to `.orqa/learning/rules/` (own rules, or declared as a contributor to a generator)
3. **Enforcement command declaration** — `enforcement.engine` field in manifest becomes the CLI flag for `orqa enforce --<engine>`
4. **File watcher declarations** — in manifest so daemon re-runs generator on rule changes
5. **KNOW artifacts** — agent-optimized knowledge explaining the standards, common violations, and fix patterns; injected into agent prompts for the relevant domain
6. **DOC artifacts** — human-readable reference documentation for the standards

A plugin that provides mechanical checks without KNOW artifacts is incomplete. Agents will fail checks that they could have avoided if they had been given the standards upfront.

**This applies to any mechanical check — not just code tools:**

| Plugin | Tool | What Rules Declare |
| ------ | ---- | ------------------ |
| `coding-standards` | ESLint | Coding standards |
| `rust` | Clippy | Rust lint rules |
| `coding-standards` | Prettier | Formatting rules |
| Grammar checker | Vale | Document quality standards |
| Accessibility checker | axe / pa11y | Accessibility requirements |
| License compliance | license-checker | Allowed/forbidden licenses |
| Security scanner | semgrep / cargo-audit | Security constraints |
| Link checker | lychee | Documentation link requirements |

The engine has no knowledge of these tools. Plugins bring the tools. Rules declare the standards. Knowledge artifacts teach the standards to agents. The generator bridges rules to tool config.

**`enforcement-generator` plugin** — owns one enforcement tool. Provides: the generator script, `enforcement.engine` declaration (becomes the `orqa enforce --<engine>` flag), `check`/`fix` command declarations, file watcher declarations, KNOW/DOC artifacts for the standards it enforces. Registers file watchers over the full contributor path tree. Responsible for one output file under `.orqa/configs/`.

**`enforcement-contributor` plugin** — provides rule data, standards, compiler options, path mappings, or any source material that feeds a generator. Also provides KNOW/DOC artifacts for the domain it contributes. Installs data under `.orqa/learning/`. Declares `contributes_to` (the generator plugin name) and `dependencies` (must include that generator). Does NOT generate config itself.

When ANY contributor's data changes (or a contributor is installed/uninstalled), the generator re-runs and recomposes from all current contributors.

**Example — ESLint:**

| Role | Plugin | What It Does |
| ------ | -------- | ------------- |
| Generator | `coding-standards` | Provides the eslint generator, owns `.orqa/configs/eslint.config.js`, registers watcher on all rule paths, declares `check`/`fix` commands |
| Contributor | `typescript` | Installs TypeScript-specific lint rules to `.orqa/learning/rules/typescript/` |
| Contributor | `svelte` | Installs Svelte-specific lint rules to `.orqa/learning/rules/svelte/` |
| Contributor | `coding-standards` | Installs project-wide style rules to `.orqa/learning/rules/standards/` |

**Example — tsconfig:**

| Role | Plugin | What It Does |
| ------ | -------- | ------------- |
| Generator | `typescript` | Provides the tsconfig generator, owns `.orqa/configs/tsconfig.base.json`, registers watcher on all type standard paths |
| Contributor | `typescript` | Installs base compiler options and path mappings to `.orqa/learning/standards/typescript/` |
| Contributor | `svelte` | Installs Svelte-specific compiler options (e.g., `verbatimModuleSyntax`) to `.orqa/learning/standards/svelte/` |
| Contributor | `tauri` | Installs Tauri-specific type paths to `.orqa/learning/standards/tauri/` |

The same pattern applies universally:

| Tool | Generator | What Contributors Provide |
| ------ | --------- | ------------------------- |
| ESLint | `coding-standards` | Lint rules per domain (typescript, svelte, standards) |
| Clippy | `rust` | Lint rules and allow/deny lists per domain |
| Prettier | `coding-standards` | Format rules per domain |
| markdownlint | `coding-standards` | Markdown lint rules per domain |
| tsconfig | `typescript` | Compiler options, type standards, path mappings per domain |

**Generated configs go to `.orqa/configs/`** — not the project root:

| Output | Generator |
| -------- | --------- |
| `.orqa/configs/eslint.config.js` | `coding-standards` |
| `.orqa/configs/clippy.toml` | `rust` |
| `.orqa/configs/.prettierrc` | `coding-standards` |
| `.orqa/configs/.markdownlint.json` | `coding-standards` |
| `.orqa/configs/tsconfig.base.json` | `typescript` |

Keeping generated configs under `.orqa/configs/` avoids polluting the project root. Enforcement commands declared in plugin manifests reference configs explicitly (e.g., `eslint --config .orqa/configs/eslint.config.js`). `orqa enforce` dispatches to these commands dynamically from the installed plugin registry.

**Generated configs are self-contained** — no cross-package imports between plugins. A plugin must not `import` from another plugin's package. Contributors provide rule data; the generator assembles it all.

**Generator manifest — authoritative `enforcement-generator` contract:**

```json
{
  "name": "@orqastudio/plugin-eslint",
  "categories": ["enforcement-generator", "enforcement-contributor"],
  "enforcement": {
    "engine": "eslint",
    "config_output": ".orqa/configs/eslint.config.js",
    "generator": "scripts/generate-config.rs",
    "actions": {
      "check": { "command": "eslint", "args": ["--config", ".orqa/configs/eslint.config.js"] },
      "fix": { "command": "eslint", "args": ["--config", ".orqa/configs/eslint.config.js", "--fix"] }
    },
    "watch": {
      "paths": [".orqa/learning/rules/*.md"],
      "filter": "enforcement_type: mechanical AND engine: eslint",
      "on_change": "regenerate"
    },
    "file_types": ["*.ts", "*.svelte", "*.js"]
  }
}
```

A multi-area plugin declares all its categories:

```json
{
  "name": "@orqastudio/plugin-typescript",
  "categories": ["domain-knowledge", "enforcement"]
}
```

**What each field does:**

| Field | Purpose |
| ----- | ------- |
| `categories` | Array of areas this plugin participates in — plugin appears in every section it declares |
| `engine` | Enforcement engine name — becomes the `orqa enforce --eslint` dynamic flag |
| `config_output` | Where the generated config is written (always under `.orqa/configs/`) |
| `generator` | The script/binary that composes config from rule files in `.orqa/learning/rules/` |
| `actions.check` / `actions.fix` | Commands registered to the central registry; dispatched by `orqa enforce` |
| `watch.paths` | Paths the daemon watches for changes |
| `watch.filter` | Which rule files apply to this engine (other engines ignore changes that don't match their filter) |
| `watch.on_change` | What the daemon does on a watched file change (`regenerate` re-runs the generator) |
| `file_types` | Which files this engine operates on — used for `--staged` filtering |

The daemon reads this manifest at startup and plugin install time:

1. Registers the watch paths
2. Registers `--eslint` as a valid `orqa enforce` flag
3. Runs the generator to produce the initial config
4. On watched file change matching the filter: re-runs the generator

**Contributor manifest (declares generator dependency):**

```json
{
  "name": "@orqastudio/plugin-typescript",
  "categories": ["domain-knowledge", "enforcement"],
  "enforcement": {
    "role": "contributor",
    "contributes-to": "eslint",
    "rules-path": ".orqa/learning/rules/typescript/"
  }
}
```

### 4.4 App Enforcement UI

The app surfaces enforcement controls on each enforcement plugin's page. The controls are **dynamically rendered from the installed plugin registry** — no enforcement tool is hardcoded in the frontend. This is the same pipeline as everything else: manifest → registry → UI rendering.

**Per-plugin page controls:**

- **Run** button — calls the Rust backend which dispatches to `orqa enforce --<engine>`
- **Fix** button — calls the Rust backend which dispatches to `orqa enforce --<engine> --fix`
- Inline results display: errors, warnings, affected files, last run timestamp

When a new enforcement plugin is installed (e.g., a Vale grammar plugin), its page automatically gets Run and Fix buttons. When it is uninstalled, those controls disappear. No frontend changes required.

This upholds P1 — no governance pattern (including what enforcement tools exist) is hardcoded in the engine or the frontend.

### 4.5 Plugin Taxonomy

#### Methodology Plugin

| Plugin | Role |
| -------- | ------ |
| `agile-methodology` | Defines the Agile methodology skeleton and stage slots |

Other methodologies could be created as alternative plugins. A project installs exactly one.

#### Workflow Plugins

Each fills one stage slot defined by the methodology:

| Plugin | Methodology Stage | What It Provides |
| -------- | ------------------ | ----------------- |
| `agile-discovery` | Discovery | Workflows for ideas, research, personas, pillars, vision, pivots |
| `agile-planning` | Planning | Workflows for planning decisions, planning ideas, planning research, wireframes |
| `agile-documentation` | Documentation | Documentation contribution workflow |
| `software-kanban` | Implementation | Epic, task, milestone workflows + kanban/roadmap views (dual-purpose) |
| `agile-review` | Review | Review contribution workflow |
| `core` | Learning | The learning loop — lessons, decisions, knowledge, rules. The app's USP. Unified and `uninstallable: true` — also provides framework artifact schemas and git hooks/enforcement. |

#### Domain Knowledge Plugins

Provide expertise without defining methodology or workflows. Do not trigger schema recomposition, but rule changes can trigger enforcement config regeneration via the generator tool pattern.

| Plugin | Domain | Enforcement Role |
| -------- | -------- | ---------------- |
| `cli` | CLI tool domain knowledge | — |
| `rust` | Rust development patterns | Generator: `.orqa/configs/clippy.toml` |
| `svelte` | Svelte development patterns | Contributor: ESLint (Svelte rules) |
| `tauri` | Tauri development patterns | Contributor: tsconfig (Tauri type paths) |
| `typescript` | TypeScript configs and patterns | Generator: `.orqa/configs/tsconfig.base.json`; Contributor: ESLint (TS rules) |
| `coding-standards` | Code quality rules and tooling | Generator: `.orqa/configs/eslint.config.js`, `.orqa/configs/.prettierrc`, `.orqa/configs/.markdownlint.json` |
| `systems-thinking` | Systems design knowledge | — |
| `plugin-dev` | Plugin development knowledge | — |

#### Connector Plugins

| Plugin | Target Framework | Output |
| -------- | ----------------- | -------- |
| `claude-code` | Claude Code | A Claude Code Plugin (`.claude/` directory) |

### 4.6 Plugin Installation Constraints

Enforced by `orqa install`:

1. **One methodology plugin** per project. Error if a second is installed.
2. **One workflow plugin per stage** defined by the methodology. Error if two plugins claim the same stage or a workflow plugin targets a nonexistent stage.
3. **Definition plugins** (methodology, workflow) trigger full recomposition of schemas, resolved workflows, and state machines.
4. **Non-definition plugins** (knowledge, views, sidecars, infrastructure) only install their assets — no recomposition triggered. Enforcement plugins also run their generator at install time.
5. The `orqa-plugin.json` manifest determines what each plugin provides and what actions are needed at install time.
6. **Schema validation before installation:** The manifest is validated against the canonical JSON schema before any installation action runs. A manifest that declares `"enforcement"` in `categories` without an `enforcement` config block is rejected. Category declarations and configuration blocks must match structurally.

### 4.7 Plugin Content Installation

When `orqa install` runs, plugins copy their content into the project's `.orqa/` directory. Plugin authors determine where in the hierarchy their artifacts are installed — the destination should make sense as part of a holistic, human-readable collection. Two plugins can install into the same category if it makes topical sense.

The `manifest.json` tracks which files came from which plugin (source hash + installed hash) to enable three-way diff: plugin source vs installed baseline vs project copy.

Source workflow definitions are NOT copied to `.orqa/` — they stay in the plugin directories. Only the resolved output gets written to `.orqa/workflows/`.

### 4.8 Composition Pipeline

The composition pipeline runs whenever a definition plugin is installed (via `orqa plugin install` or as part of the dev environment `orqa install`):

1. Read the installed methodology plugin's workflow skeleton
2. Read each installed workflow plugin's contribution manifest (from plugin directories, not copies)
3. Merge contributions into the methodology's stage slots
4. Compose the full JSON schema from all plugin-provided artifact types and state machines
5. Validate the composed result
6. Write resolved workflows to `.orqa/workflows/<stage>.resolved.yaml` — one file per stage, containing all artifact types, state machines, relationships for that stage
7. Write composed schema (`schema.composed.json`) for LSP/MCP validation
8. Write prompt registry for the prompt pipeline

The runtime reads only resolved files. Recomposition happens only on plugin install/update. Connectors watch for changes and regenerate their output in real time.
