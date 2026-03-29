---
id: KNOW-41ccf7c4
type: knowledge
status: active
title: Plugin Architecture
domain: architecture
description: Plugin taxonomy, purposes, composition pipeline, installation constraints — essential for understanding what plugins provide and how they compose
tier: always
relationships:
  synchronised-with: DOC-41ccf7c4
---

# Plugin Architecture

## Plugin Purposes

Plugins declare `categories` as a plural array — a plugin participates in every category it declares. Valid values: `methodology`, `workflow`, `domain-knowledge`, `enforcement`, `connector`. The `typescript` plugin declares `["domain-knowledge", "enforcement"]`. The frontend tags and filters plugins by every declared category.

**Categories must match config blocks — enforced by JSON schema (`if/then` conditionals):**

| Category | Required block |
| -------- | -------------- |
| `"enforcement"` | `enforcement` block (engine, generator, actions, watch, file_types) |
| `"domain-knowledge"` | `knowledge_declarations` block |
| `"workflow"` | `workflows` block with `stage_slot` |
| `"methodology"` | `methodology` block |
| `"connector"` | `connector` block |

`orqa install` validates the manifest against the JSON schema before any installation action. A manifest claiming `"enforcement"` without an `enforcement` block is rejected at the schema level — not at runtime.

| Purpose | Effect at Install Time |
| --------- | ---------------------- |
| **Methodology definition** | Triggers full schema/workflow recomposition |
| **Workflow definition** | Triggers full schema/workflow recomposition |
| **Knowledge/rules** | Assets installed. Rule changes trigger enforcement regeneration. |
| **App extension** | Assets installed, no recomposition |
| **Sidecar** | Provides LLM inference capability |
| **Connector** | Generates + watches for regeneration |
| **Config generator** | Runs generator → composed config under `.orqa/configs/`. Registers watcher. Declares commands. Installs KNOW/DOC artifacts. A plugin without KNOW artifacts is incomplete. |
| **Config contributor** | Installs rule/standards data to `.orqa/learning/`. Installs KNOW/DOC for the contributed domain. Triggers generator re-run. |

## Config Composition Pattern (Generator + Contributors) — Universal

An **enforcement plugin** is any plugin that declares an `enforcement` section in its manifest. It can be standalone (dedicated to enforcement only) or multi-area (e.g., `typescript` provides domain knowledge AND tsconfig generator AND ESLint contributor rules). The `enforcement` section is a capability declaration, not a plugin category.

This is the **universal enforcement model**. Every mechanical check follows the same pipeline regardless of tool — linting, formatting, type checking, grammar, accessibility, security scanning, license compliance, link checking, or anything else. The engine has no knowledge of specific tools.

**The pipeline (always):**

1. Rules in `.orqa/learning/rules/` declare what must be true
2. Generator plugin translates rules → tool-specific config under `.orqa/configs/`
3. Plugin registers enforcement commands via manifest `enforcement.engine` field (becomes CLI flag)
4. Plugin declares file watchers in manifest
5. `orqa enforce` dispatches to all registered engines — specific tools invisible to caller

**Generator plugin** — owns one enforcement tool. Provides: generator, `enforcement.engine` declaration (becomes the `orqa enforce --<engine>` flag), `check`/`fix` command declarations, file watcher declarations, KNOW/DOC artifacts for the standards it enforces. Responsible for one output file under `.orqa/configs/`.

**Contributor plugins** — provide rule data, standards, compiler options, or any source material that feeds a generator. Also provide KNOW/DOC artifacts for their domain. Install data to `.orqa/learning/`. Declare which generator they contribute to. Do NOT generate config themselves.

**No cross-package imports.** Generated configs are self-contained. **No KNOW artifacts = incomplete plugin.**

**Example — ESLint:**

| Role | Plugin | Contribution |
| ------ | -------- | ------------ |
| Generator | `coding-standards` | Owns `.orqa/configs/eslint.config.js`, watcher, commands |
| Contributor | `typescript` | TypeScript lint rules → `.orqa/learning/rules/typescript/` |
| Contributor | `svelte` | Svelte lint rules → `.orqa/learning/rules/svelte/` |
| Contributor | `coding-standards` | Project-wide rules → `.orqa/learning/rules/standards/` |

**Example — tsconfig:**

| Role | Plugin | Contribution |
| ------ | -------- | ------------ |
| Generator | `typescript` | Owns `.orqa/configs/tsconfig.base.json`, watcher |
| Contributor | `typescript` | Base compiler options → `.orqa/learning/standards/typescript/` |
| Contributor | `svelte` | Svelte compiler options → `.orqa/learning/standards/svelte/` |
| Contributor | `tauri` | Tauri type paths → `.orqa/learning/standards/tauri/` |

**Generated config locations (generator → output):**

| Generator Plugin | Output |
| ---------------- | ------ |
| `coding-standards` | `.orqa/configs/eslint.config.js`, `.orqa/configs/.prettierrc`, `.orqa/configs/.markdownlint.json` |
| `rust` | `.orqa/configs/clippy.toml` |
| `typescript` | `.orqa/configs/tsconfig.base.json` |

## App Enforcement UI

Enforcement controls on plugin pages are **dynamically rendered from the installed plugin registry** — no tool is hardcoded in the frontend (P1).

- **Run** button → Rust backend dispatches `orqa enforce --<engine>`
- **Fix** button → Rust backend dispatches `orqa enforce --<engine> --fix`
- Inline results: errors, warnings, affected files, last run timestamp

Same pipeline: manifest → registry → UI rendering. Install a new enforcement plugin → page gets Run/Fix buttons automatically.

## Methodology vs Workflow (Critical Distinction)

- **Methodology** = overarching approach (e.g., Agile). One per project. Provides named contribution points (slots).
- **Workflow** = self-contained sub-workflow for one methodology stage. Owns its own state machine. No inheritance. One per stage.

## Plugin Taxonomy

**Methodology:** `agile-methodology`

**Workflow plugins (one per stage):**

| Plugin | Stage |
| -------- | ------- |
| `agile-discovery` | Discovery |
| `agile-planning` | Planning |
| `agile-documentation` | Documentation |
| `software-kanban` | Implementation |
| `agile-review` | Review |
| `core` | Learning (uninstallable, also provides framework schemas + git hooks) |

**Domain knowledge (with enforcement roles):**

| Plugin | Enforcement Role |
| -------- | ---------------- |
| `cli` | — |
| `rust` | Generator: `.orqa/configs/clippy.toml` |
| `svelte` | Contributor: ESLint (Svelte rules) |
| `tauri` | Contributor: tsconfig (Tauri type paths) |
| `typescript` | Generator: `.orqa/configs/tsconfig.base.json`; Contributor: ESLint (TS rules) |
| `coding-standards` | Generator: `.orqa/configs/eslint.config.js`, `.orqa/configs/.prettierrc`, `.orqa/configs/.markdownlint.json` |
| `systems-thinking` | — |
| `plugin-dev` | — |

**Connector:** `claude-code` → generates `.claude/` directory

## Installation Constraints

1. **One methodology plugin** per project — error if second installed
2. **One workflow plugin per stage** — error if two claim same stage
3. **Definition plugins** (methodology, workflow) → full recomposition
4. **Non-definition plugins** → assets only, no recomposition. Enforcement plugins also run their generator at install time.

## Composition Pipeline

Runs on every definition plugin install:

1. Read methodology plugin's workflow skeleton
2. Read each workflow plugin's contribution manifest (from plugin dirs, NOT copies)
3. Merge contributions into stage slots
4. Compose full JSON schema from all plugin-provided artifact types
5. Validate composed result
6. Write resolved workflows to `.orqa/workflows/<stage>.resolved.yaml`
7. Write `schema.composed.json` for LSP/MCP validation
8. Write prompt registry for prompt pipeline

**Source workflow definitions stay in plugin directories — only resolved output goes to `.orqa/`.**

## Content Installation

Plugin authors determine destination in `.orqa/` hierarchy. `manifest.json` tracks source hash + installed hash enabling **three-way diff**: plugin source vs installed baseline vs project copy.
