# Plugin Architecture

> This is part of the OrqaStudio Architecture Reference. See ARCHITECTURE.md for the complete document.

---

## 4. Plugin Architecture

### 4.1 Plugin Purposes

A plugin serves one or more of these purposes:

| Purpose | What It Provides | Effect at Install Time |
|---------|-----------------|----------------------|
| **Methodology definition** | The overarching workflow skeleton and stage slots | Triggers full schema/workflow recomposition |
| **Workflow definition** | A complete sub-workflow for one methodology stage | Triggers full schema/workflow recomposition |
| **Knowledge/rules** | Domain expertise, behavioral constraints, documentation | Assets installed. No schema recomposition, but rule changes trigger enforcement regeneration. |
| **App extension** | Custom views, UI components | Assets installed, no recomposition |
| **Sidecar** | LLM provider integration for the app | Provides inference capability |
| **Connector** | Generation pipeline for a third-party tool | Generates + watches for regeneration |
| **Infrastructure** | Tooling integrations (linting config generation) | Generates enforcement configs from engine rules |

A single plugin may serve multiple purposes. The `orqa-plugin.json` manifest is the single source of truth for what each plugin provides and what installation actions are needed. Manifests must declare: plugin purpose/type, stage slot (if workflow), content types provided, and installation targets.

### 4.2 Methodology and Workflows

This distinction is fundamental to composability:

**Methodology** = the overarching approach (e.g., Agile). Defined by a methodology plugin. The methodology plugin provides the workflow skeleton with named contribution points (slots) that workflow plugins fill. One and only one methodology plugin may be installed per project.

**Workflows** = the sub-workflows within the methodology. Each workflow plugin provides the complete sub-workflow for one stage of the methodology — including its own state machine (states, transitions, guards, gates) and artifact types. Each workflow is self-contained with no inheritance from other plugins. One workflow plugin per stage defined in the methodology.

Relationships define the flow both **within** a workflow (e.g., task delivers epic, research informs decision) and **between** workflows and the methodology (e.g., discovery outputs feed planning inputs, implementation delivers against planning). The graph engine computes inverses from forward-only declared relationships, making the entire methodology traceable end-to-end.

### 4.3 Plugin Taxonomy

#### Methodology Plugin

| Plugin | Role |
|--------|------|
| `agile-workflow` | Defines the Agile methodology skeleton and stage slots |

Other methodologies could be created as alternative plugins. A project installs exactly one.

#### Workflow Plugins

Each fills one stage slot defined by the methodology:

| Plugin | Methodology Stage | What It Provides |
|--------|------------------|-----------------|
| `agile-discovery` | Discovery | Workflows for ideas, research, personas, pillars, vision, pivots |
| `agile-planning` | Planning | Workflows for planning decisions, planning ideas, planning research, wireframes |
| `agile-documentation` | Documentation | Documentation contribution workflow |
| `software-kanban` | Implementation | Epic, task, milestone workflows + kanban/roadmap views (dual-purpose) |
| `agile-review` | Review | Review contribution workflow |
| `core` | Learning | The learning loop — lessons, decisions, knowledge, rules. The app's USP. Unified and `uninstallable: true` — also provides framework artifact schemas and git hooks/enforcement. |

#### Domain Knowledge Plugins

Provide expertise without defining methodology or workflows. Do not trigger schema recomposition, but rule changes can trigger enforcement config regeneration.

| Plugin | Domain | Notes |
|--------|--------|-------|
| `cli` | CLI tool domain knowledge | |
| `rust` | Rust development patterns | Dual-purpose: knowledge + generates linting infrastructure from engine |
| `svelte` | Svelte development patterns | |
| `tauri` | Tauri development patterns | |
| `typescript` | TypeScript configs and patterns | Dual-purpose: knowledge + generates linting infrastructure from engine |
| `coding-standards` | Code quality rules and tooling | Generates enforcement configs |
| `systems-thinking` | Systems design knowledge | |
| `plugin-dev` | Plugin development knowledge | |

#### Connector Plugins

| Plugin | Target Framework | Output |
|--------|-----------------|--------|
| `claude-code` | Claude Code | A Claude Code Plugin (`.claude/` directory) |

### 4.4 Plugin Installation Constraints

Enforced by `orqa install`:

1. **One methodology plugin** per project. Error if a second is installed.
2. **One workflow plugin per stage** defined by the methodology. Error if two plugins claim the same stage or a workflow plugin targets a nonexistent stage.
3. **Definition plugins** (methodology, workflow) trigger full recomposition of schemas, resolved workflows, and state machines.
4. **Non-definition plugins** (knowledge, views, sidecars, infrastructure) only install their assets — no recomposition triggered.
5. The `orqa-plugin.json` manifest determines what each plugin provides and what actions are needed at install time.

### 4.5 Plugin Content Installation

When `orqa install` runs, plugins copy their content into the project's `.orqa/` directory. Plugin authors determine where in the hierarchy their artifacts are installed — the destination should make sense as part of a holistic, human-readable collection. Two plugins can install into the same category if it makes topical sense.

The `manifest.json` tracks which files came from which plugin (source hash + installed hash) to enable three-way diff: plugin source vs installed baseline vs project copy.

Source workflow definitions are NOT copied to `.orqa/` — they stay in the plugin directories. Only the resolved output gets written to `.orqa/workflows/`.

### 4.6 Composition Pipeline

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
