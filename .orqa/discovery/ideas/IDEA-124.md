---
id: IDEA-124
title: "Coding standards as rules — plugin-provided tooling, project-level enforcement, org-level sync"
description: "Coding standards become OrqaStudio rules with enforcement entries keyed by plugin. Plugins provide linting/testing infrastructure. Rules define the config. Organisation mode syncs standards to sub-projects with override tracking."
status: captured
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - How does the file watcher detect rule changes and trigger config rebuilds?
  - What's the merge strategy for org-level rules + project-level overrides?
  - Can we diff the generated config against the current on-disk config to avoid unnecessary rebuilds?
  - How do we handle plugins that aren't installed in a sub-project but have enforcement entries in an org-level rule?
  - Should the generated config files be committed or gitignored? (committed = visible, gitignored = always derived)
  - How does this interact with existing eslint-config and test-config libraries? Migration path?
  - Can we validate that enforcement config entries are valid for their target tool at rule creation time?
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Coding standards as visible governance artifacts with traceable decisions — not hidden config files
  - target: PERSONA-002
    type: benefits
---

# Coding Standards as Rules

## The Model

Coding standards are OrqaStudio rules. Each rule has enforcement entries keyed by plugin. Plugins provide the tooling infrastructure. Rules define what the tools enforce. Decisions govern why.

```
AD-xxx (Decision: "Strict TypeScript, no shortcuts")
  ↓ governs
RULE-xxx (Rule: "Coding Standards")
  ↓ enforcement entries
  Plugin: @orqastudio/plugin-svelte → ESLint config
  Plugin: @orqastudio/plugin-tauri  → Clippy config
  ↓ paired with
DOC-xxx + SKILL-xxx (what the standards are, for humans and agents)
```

## Rule Enforcement Schema

A rule's enforcement section contains entries keyed by plugin and tool, with an array of config lines:

```yaml
enforcement:
  - plugin: "@orqastudio/plugin-svelte"
    tool: eslint
    config:
      - rule: "@typescript-eslint/no-explicit-any"
        severity: error
      - rule: "@typescript-eslint/no-unused-vars"
        severity: warn
        options: { argsIgnorePattern: "^_" }
      - rule: "svelte/no-reactive-reassign"
        severity: error
  - plugin: "@orqastudio/plugin-svelte"
    tool: svelte-check
    config:
      - flag: "--threshold"
        value: "warning"
  - plugin: "@orqastudio/plugin-tauri"
    tool: clippy
    config:
      - lint: "clippy::unwrap_used"
        level: deny
      - lint: "clippy::expect_used"
        level: deny
  - plugin: "@orqastudio/plugin-tauri"
    tool: rustfmt
    config:
      - key: "max_width"
        value: 100
      - key: "edition"
        value: "2021"
```

## Sub-Project Overrides

In organisation mode, coding standards rules propagate from the org level to all sub-projects. A sub-project can override specific config lines by redeclaring them in its own rule:

```yaml
# Org-level rule: RULE-006
enforcement:
  - plugin: "@orqastudio/plugin-svelte"
    tool: eslint
    config:
      - rule: "@typescript-eslint/no-explicit-any"
        severity: error

# Sub-project override in app/.orqa/process/rules/RULE-006-override.md
overrides:
  - source: RULE-006
    plugin: "@orqastudio/plugin-svelte"
    tool: eslint
    config:
      - rule: "@typescript-eslint/no-explicit-any"
        severity: warn  # Relaxed for legacy code migration
    rationale: "Legacy codebase has 200+ any types — migrating incrementally"
    decision: AD-xxx  # Decision that authorises the override
```

The override is a governance artifact — it has a rationale and traces to a decision. It's not a silent config change.

## Config File Generation

Plugins provide a file watcher that:

1. Watches `.orqa/process/rules/` for changes to rules with enforcement entries
2. When a rule changes, collects all enforcement entries for each installed plugin
3. Merges org-level config + sub-project overrides
4. Generates the tool config file (`.eslintrc.json`, `clippy.toml`, `.rustfmt.toml`, etc.)
5. Writes to the appropriate project root (sub-module for sub-projects)

The generated config files are the **output** of the governance system, not the input. Developers never edit `.eslintrc.json` directly — they edit the rule, and the config regenerates.

## Plugins

### @orqastudio/plugin-svelte

Provides:
- ESLint infrastructure (installs deps, generates config)
- svelte-check infrastructure
- Vitest infrastructure
- Skills: Svelte 5 runes, component patterns, store patterns
- Docs: Svelte best practices, testing guide

Registers as a tool provider for enforcement entries with `tool: eslint`, `tool: svelte-check`, `tool: vitest`.

### @orqastudio/plugin-tauri

Provides:
- Clippy infrastructure (generates clippy.toml)
- rustfmt infrastructure (generates .rustfmt.toml)
- cargo test infrastructure
- Skills: Tauri v2 IPC patterns, Rust async, error handling
- Docs: Tauri development guide, Rust testing guide

Registers as a tool provider for enforcement entries with `tool: clippy`, `tool: rustfmt`, `tool: cargo-test`.

### Agent Skills for Config Management

Each plugin ships a skill that teaches agents how to:
- Add/modify enforcement entries on coding standards rules
- Create override rules for sub-projects with proper rationale and decision links
- Understand the config merge hierarchy (org → project → override)
- Run the tools and interpret results
- Fix common violations automatically

This means an agent can manage coding standards on behalf of the user: "Make TypeScript strict across all projects" → agent creates the rule with enforcement entries, links to a decision, generates config. The human approves the decision; the agent handles the plumbing.

Each skill has a paired documentation artifact (`synchronised-with`) so humans can understand the same concepts: how enforcement entries work, how to override standards for a sub-project, how the config merge hierarchy works, what each tool checks and why.

### Plugin Agents

Each plugin ships dedicated agents that are NOT conversational — they're scoped task agents that run, produce structured output, and return:

**Assessment agents** (read-only, return structured findings):
- `svelte-standards-assessor` — scans a project, returns a structured report of violations against configured rules. No conversation, no suggestions — just facts.
- `tauri-standards-assessor` — same for Rust/Tauri.

**Configuration agents** (write, scoped to specific tasks):
- `svelte-standards-configurator` — reads the project's coding standards rules, generates/updates ESLint + svelte-check + Vitest config files. Does the job and exits.
- `tauri-standards-configurator` — same for clippy + rustfmt + cargo test config.

These agents have constrained tool access (file read/write, no shell, no web), a defined output schema, and no memory between runs. They're implementation tools, not thinking partners. The orchestrator delegates to them for specific jobs — they don't participate in conversations.

**Plugin installation** is handled by a core platform agent (not plugin-specific). Each plugin ships an installation skill that teaches the core installer agent how to install that plugin — what dependencies to add, what config to generate, what questions to ask about sub-projects. The agent is generic; the knowledge comes from the plugin's skill.

## orqa check

`orqa check` becomes the single command that:

1. Reads all rules with enforcement entries
2. For each installed plugin, collects the config
3. Runs each tool via the plugin's executor
4. Aggregates results
5. Reports pass/fail per rule

The developer runs one command. The governance system decides which tools run with which config.

## Dependency Management

When a plugin is installed, it ensures the target project has the required tool dependencies:

1. **Check existing** — read `package.json` / `Cargo.toml` for already-installed deps
2. **Add missing** — append any missing dev dependencies (e.g. `eslint`, `@typescript-eslint/parser`, `vitest`)
3. **Install** — run `npm install` / `cargo fetch` to resolve
4. **Idempotent** — if deps already exist, do nothing (existing project support)

### Organisation Mode

When installing to an org-mode project, the plugin asks which sub-projects it applies to:

```
Installing @orqastudio/plugin-svelte...

This org has 7 sub-projects. Based on detected languages:

  Recommended (TypeScript detected):
  [x] app/ui          — Svelte + TypeScript
  [x] libs/sdk        — TypeScript
  [x] libs/cli        — TypeScript
  [x] connectors/claude-code — TypeScript

  Not recommended:
  [ ] app/backend     — Rust only (use @orqastudio/plugin-tauri instead)
  [ ] plugins/software — No source code (skills + docs only)
  [ ] libs/brand      — Assets only

Install to selected projects? [Y/n]
```

Each selected sub-project gets:
- Tool dependencies added to its `package.json`
- Config files generated from the org-level coding standards rules
- Any sub-project-specific overrides preserved

Sub-projects that aren't selected don't get the plugin's enforcement — the tools aren't installed and no config is generated. The org-level rule still exists but its enforcement entries for that plugin are inactive in unselected projects.

## Migration Path

Current `libs/eslint-config` and `libs/test-config` become dependencies of the Svelte plugin. The standalone libraries are absorbed — their config becomes the default enforcement entries in the platform's core rules. Projects that use the Svelte plugin get the standards automatically. Projects that don't use Svelte don't get ESLint config they don't need.
