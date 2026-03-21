---
id: DOC-e0042602
type: doc
title: Plugin Architecture
category: architecture
description: "How plugins extend OrqaStudio — current CLI companion plugin, four-layer model, component SDK extraction plan, and built-in vs plugin decision framework."
sort: 12
relationships:
  - target: RULE-f9d0279c
    type: documents
  - target: EPIC-0a8a5e72
    type: documents
  - target: RULE-98682b5e
    type: documents
    rationale: Documentation page references RULE-98682b5e
  - target: RULE-deab6ea7
    type: documents
    rationale: Documentation page references RULE-deab6ea7
---

# Plugin Architecture

This document describes OrqaStudio's plugin architecture: what exists today, the four-layer extension model, and the planned SDK extraction for plugin authors.

## Current State

OrqaStudio has two extension surfaces today:

1. **CLI companion plugin** — a Claude Code plugin that replicates governance enforcement in CLI sessions
2. **MCP server integration** — the orqastudio MCP server exposes native search and governance tools to CLI sessions

Both are operational. The component library SDK and view registration API are future work.

---

## CLI Companion Plugin

**Location:** `.orqa/plugins/orqastudio-claude-plugin/`
**Type:** Claude Code plugin (hooks + commands + knowledge)
**Status:** Implemented and active

The CLI companion plugin bridges OrqaStudio's governance framework with Claude Code's hook system. It exists because the app's built-in enforcement engine (Rust, `enforcement_engine.rs`) only runs inside the Tauri app. CLI sessions need an independent enforcement path.

### Plugin Structure

```
.claude-plugin/
  plugin.json            # Manifest: name, version, description
  marketplace.json       # Local marketplace registration for discovery
hooks/
  hooks.json             # Hook configuration (5 hook types)
  scripts/
    rule-engine.mjs      # PreToolUse: enforcement evaluation
    prompt-injector.mjs  # UserPromptSubmit: artifact graph traversal
    graph-guardian.mjs   # PostToolUse: graph integrity advisory
    session-start.sh     # SessionStart: symlinks + health checks
    stop-checklist.sh    # Stop: pre-session-end reminders
commands/
  orqa.md                # /orqa slash command: governance summary
knowledge/
  plugin-setup/KNOW.md  # Installation guide
  rule-enforcement/KNOW.md  # Rule enforcement documentation
```

### Hook Pipeline

| Hook Event | Script | Trigger | Behavior |
|-----------|--------|---------|----------|
| `PreToolUse` | `rule-engine.mjs` | `Write`, `Edit`, `Bash` | Loads active rules with `enforcement` frontmatter entries. Evaluates conditions against tool input. Block verdicts deny the tool call; warn verdicts emit a system message; inject verdicts emit knowledge IDs for the agent to read. |
| `UserPromptSubmit` | `prompt-injector.mjs` | Every user message | Detects artifact references (TASK-NNN, EPIC-NNN) in the prompt. Crawls artifact graph edges (task.epic, task.knowledge, task.docs, epic.research-refs, epic.docs-required) and emits referenced IDs as a system message. |
| `PostToolUse` | `graph-guardian.mjs` | `Write`, `Edit` to `.orqa/` | Checks newly written artifacts for missing relationship fields (e.g., task without `docs`, epic without `research-refs`). Advisory only — warns but never blocks. |
| `SessionStart` | `session-start.sh` | Session begins | Sets up `.claude/` symlinks to `.orqa/` source of truth. Installs plugin knowledge into `.orqa/process/knowledge/`. Runs health checks: stashes, worktrees, orphaned directories, uncommitted file count, previous session state, dogfood mode detection. |
| `Stop` | `stop-checklist.sh` | Session ending | Reminds the agent to commit changes, update session state, and clean up. |

### Session Deduplication

Both the rule engine and prompt injector track what they have already injected during a session:

- **Knowledge injection:** `tmp/.injected-skills.json` — prevents re-injecting the same knowledge content on repeated matches
- **Artifact ID injection:** `tmp/.injected-ids.json` — prevents re-emitting the same artifact IDs when referenced again

Both files are ephemeral (cleared when `tmp/` is cleaned between sessions).

### Symlink Management

The session-start hook manages the `.claude/` compatibility layer. Claude Code discovers project instructions via `.claude/CLAUDE.md`, `.claude/rules/`, `.claude/agents/`, and `.claude/knowledge/`. The plugin creates symlinks from these paths to their `.orqa/` source of truth:

| Symlink | Target |
|---------|--------|
| `.claude/CLAUDE.md` | `.orqa/process/agents/orchestrator.md` |
| `.claude/rules/` | `.orqa/process/rules/` |
| `.claude/agents/` | `.orqa/process/agents/` |
| `.claude/skills/` | `.orqa/process/knowledge/` |

Plugin-bundled skills (e.g., `plugin-setup`, `rule-enforcement`) are symlinked into `.orqa/process/knowledge/` so the artifact scanner and app can discover them alongside core skills.

### Relationship to App Enforcement

The CLI plugin and the app's built-in enforcement engine are **independent implementations** of the same governance model:

| Aspect | CLI Plugin | App (Built-in) |
|--------|-----------|----------------|
| Language | JavaScript (Node.js) | Rust |
| Rule source | Same `.orqa/process/rules/*.md` files | Same `.orqa/process/rules/*.md` files |
| YAML parsing | Custom lightweight parser | `serde_yaml` |
| Pattern matching | JavaScript `RegExp` | Rust `regex` crate |
| Knowledge injection | Emits knowledge IDs as `systemMessage` | Reads KNOW.md content, prepends to tool output |
| Process gates | Not implemented (CLI has no `WorkflowTracker`) | Five process gates via `process_gates.rs` |
| Graph traversal | Simple edge crawl in `prompt-injector.mjs` | Full bidirectional graph in `artifact_graph.rs` |

They share no code. The rule YAML frontmatter format is the contract that both implementations parse.

---

## MCP Server Integration (orqastudio MCP)

**Config:** `.mcp.json`
**Status:** Implemented and active

The orqastudio MCP server exposes native search and governance tools to CLI sessions via the Model Context Protocol. It runs as a local server launched by `orqa mcp` and exposes three search tools:

| Tool | Purpose |
|------|---------|
| `search_regex` | Exact pattern search across the codebase |
| `search_semantic` | Meaning-based search using ONNX embeddings |
| `search_research` | Multi-step architectural analysis |

The same native ONNX Runtime + DuckDB engine (see core architecture doc, System 1) serves both CLI (via MCP) and App (via embedded Tauri commands). The `orqa-code-search` wrapper knowledge artifact resolves to the correct access path based on context.

---

## Four-Layer Plugin Model

OrqaStudio's extension architecture follows four trust layers. Each layer has different discovery, loading, and trust characteristics.

### Layer Definitions

| Layer | Label | Trust Level | Discovery | Examples |
|-------|-------|------------|-----------|---------|
| **Built-in** | `core` | Full | Ships with app binary | Artifact scanner, enforcement engine, streaming pipeline, search engine |
| **Official** | `plugin` | High | OrqaStudio marketplace or bundled git submodule | CLI companion plugin, future GitHub integration |
| **Community** | `community` | Medium | Community marketplace or manual install | Third-party workflow plugins, custom dashboard panels |
| **User** | `user` | Full (local) | User's project `.orqa/` directory | Personal workflow knowledge, project-specific rules |

### Discovery and Loading

**Built-in (core):**
- Compiled into the Tauri binary
- No discovery needed — always available
- Knowledge artifacts with `layer: core` are loaded by the orchestrator based on agent YAML frontmatter

**Official (plugin):**
- Discovered via the `.claude-plugin/marketplace.json` local marketplace or `settings.json` plugin configuration
- Installed as git submodules in `.orqa/plugins/`
- Plugin knowledge artifacts symlinked into `.orqa/process/knowledge/` at session start
- Hooks registered via `hooks.json` in the plugin directory

**Community:**
- Same installation mechanism as official plugins
- Reviewed but not maintained by the OrqaStudio team
- Knowledge artifacts carry `layer: community` for trust distinction

**User:**
- Files directly in `.orqa/process/knowledge/` with `layer: user`
- Rules in `.orqa/process/rules/`
- No installation step — the artifact scanner picks them up automatically

### Trust and Isolation

All layers share the same enforcement pipeline. The difference is in trust defaults:

| Layer | Auto-approve hooks? | Knowledge injection? | Can modify core? |
|-------|-------------------|------------------|-----------------|
| Built-in | Yes | Yes | Yes (firmware) |
| Official | User approves on install | Yes | No |
| Community | User approves per-hook | Yes, with notice | No |
| User | Yes (user authored) | Yes | No |

Core graph artifacts (schemas, orchestrator prompt, core knowledge) are protected by [RULE-98682b5e](RULE-98682b5e). Only the dogfood exception allows modification.

---

## Artifact Graph SDK (Implemented)

**Location:** `ui/src/lib/sdk/artifact-graph.svelte.ts`
**Status:** Implemented and active

The Artifact Graph SDK is a Svelte 5 rune-based client that maintains an in-memory copy of the bidirectional artifact graph built by the Rust backend. After initialization, all resolution and query methods operate synchronously on cached data with no IPC round-trips.

### API Surface

| Category | Method | Description |
|----------|--------|-------------|
| **Lifecycle** | `initialize()` | Fetch full graph from backend, register for auto-refresh |
| **Lifecycle** | `refresh()` | Rebuild graph from disk, re-fetch into cache |
| **Resolution** | `resolve(id)` | Look up a node by artifact ID (e.g., "[EPIC-0a8a5e72](EPIC-0a8a5e72)") |
| **Resolution** | `resolveByPath(path)` | Look up a node by relative file path |
| **Relationships** | `referencesFrom(id)` | All outgoing edges from a node |
| **Relationships** | `referencesTo(id)` | All incoming edges (backlinks) to a node |
| **Bulk queries** | `byType(type)` | All nodes of a given type (e.g., "epic", "task") |
| **Bulk queries** | `byStatus(status)` | All nodes with a given status value |
| **Content** | `readContent(path)` | Read raw markdown from disk (async, always fresh) |
| **Health** | `brokenRefs()` | All references whose target ID does not exist |
| **Health** | `orphans()` | All nodes with no incoming or outgoing references |
| **Subscriptions** | `subscribe(id, callback)` | Get notified when a specific node changes |
| **Subscriptions** | `subscribeType(type, callback)` | Get notified when any node of a type changes |

### Reactive State

| Property | Type | Description |
|----------|------|-------------|
| `graph` | `SvelteMap<string, ArtifactNode>` | All nodes keyed by ID |
| `pathIndex` | `SvelteMap<string, string>` | Path-to-ID reverse lookup |
| `stats` | `GraphStats` | Node count, edge count, orphans, broken refs |
| `loading` | `boolean` | True during refresh |
| `lastRefresh` | `Date` | Timestamp of last successful refresh |
| `error` | `string \| null` | Last error message |

The SDK auto-refreshes when it receives the `"artifact-graph-updated"` Tauri event (emitted by the file watcher when `.orqa/` files change).

### Plugin Relevance

The subscription API (`subscribe`, `subscribeType`) is designed for plugin use. A plugin that registers a custom view for epics can subscribe to type-level changes and re-render when any epic is modified, without polling.

---

## Component Library Extraction Plan (Future)

**Tracking:** [IDEA-53205849](IDEA-53205849)
**Status:** Captured (not yet shaped)

### Current State

Shared components live in `ui/src/lib/components/shared/` and are only importable within the core app. There are 12 shared components today:

| Component | Purpose |
|-----------|---------|
| `ArtifactListItem` | Clickable list item with status dot and description |
| `ConfirmDeleteDialog` | Destructive action confirmation dialog |
| `EmptyState` | Empty list/grid placeholder with icon, title, action |
| `ErrorDisplay` | Error message with retry option |
| `ErrorToast` | Transient error notification |
| `LoadingSpinner` | Branded or minimal loading indicator |
| `MetadataRow` | Icon + label + badge array |
| `SearchInput` | Search input with icon prefix |
| `SelectMenu` | Dropdown select with check marks |
| `SmallBadge` | Compact badge for metadata tags |
| `StatusIndicator` | Multi-mode status display (badge/dot/inline) |
| `ThinkingBlock` | Collapsible AI thinking content |

Additionally, the app uses shadcn-svelte components (`Button`, `Card`, `Dialog`, `Tooltip`, `Resizable`, etc.) as its base component library.

### What the SDK Would Provide

1. **Shared components as an importable package** — The 12 components above, plus key content components (`CodeBlock`, `MarkdownRenderer`), extracted into a package that plugins can import.

2. **Artifact Graph SDK** — Already exists and is documented above. Plugins would import it to query the graph, resolve artifact IDs, and subscribe to changes.

3. **Theme token access** — Plugins need to render UI that matches the active theme. The SDK would expose CSS custom properties and Tailwind utility classes so plugin views are visually consistent without hard-coding colors.

4. **Type definitions** — TypeScript types for `ArtifactNode`, `ArtifactRef`, `GraphStats`, `NavTree`, and other shared interfaces that plugins need for type-safe development.

### Distribution Options (Requires Research)

| Option | Pros | Cons |
|--------|------|------|
| **npm package** | Standard distribution, versioned | Requires publishing infrastructure |
| **Bundled with app** | Always available, no version mismatch | Increases app size |
| **Git submodule** | Simple, version-pinned | Manual updates |
| **Package alias** | Import from app's own `node_modules` | Tight coupling to app build |

The distribution mechanism is an open research question listed in [IDEA-53205849](IDEA-53205849).

---

## View Registration API (Conceptual, Future)

Plugins need to register custom views that render inside OrqaStudio's UI. This API does not exist yet, but the design direction is outlined here based on the app's current architecture.

### Custom Artifact Views

A plugin could register a renderer for a specific artifact type. When the user navigates to an artifact of that type, the plugin's view renders instead of the default markdown viewer.

**Conceptual registration:**

```typescript
// Plugin registers a custom view for "epic" artifacts
orqa.registerArtifactView("epic", {
  component: EpicBoardView,
  label: "Board View",
  // Default markdown view remains available as a tab
});
```

The navigation store already resolves artifact types to views. A registration API would extend this resolution to include plugin-provided views.

### Dashboard Panel Registration

Plugins could register panels that appear in a dashboard or sidebar:

```typescript
// Plugin registers a dashboard panel
orqa.registerDashboardPanel({
  id: "github-prs",
  label: "Pull Requests",
  component: GitHubPRPanel,
  icon: "git-pull-request",
});
```

### Navigation Integration

Plugin-registered views would integrate with the existing navigation system:

- Artifact views appear as alternative renderers (tabs or toggles) on artifact detail pages
- Dashboard panels appear in a configurable dashboard layout
- Plugin-specific navigation entries appear in a "Plugins" activity bar group

### Prerequisites

Before the view registration API can be built:

1. Component library must be extracted as an SDK
2. A plugin sandbox or iframe boundary must be evaluated for security
3. Hot-reloading semantics for plugin views need design
4. The navigation store needs a registration hook for dynamic entries

---

## Built-in vs Plugin Decision Framework

Based on findings from [RES-00ec6dd1](RES-00ec6dd1), this framework guides where new features belong.

### Decision Criteria

| Criterion | Built-in | Plugin |
|-----------|---------|--------|
| Required by ALL users regardless of domain | Yes | No |
| Serves a core pillar directly | Yes | Maybe |
| Needs deep streaming pipeline integration | Yes | No |
| Provider-specific (e.g., Azure, Vertex) | No | Yes |
| Domain-specific (e.g., software-only, GitHub) | No | Yes |
| Requires external service accounts | No | Yes |
| Can evolve independently of the app release cycle | No | Yes |

### Applying the Framework

| Feature | Verdict | Reason |
|---------|---------|--------|
| Provider abstraction (NDJSON, Provider interface) | **Built-in** | Core infrastructure all providers depend on |
| Claude Agent SDK provider | **Built-in** | Reference implementation, ships with app |
| OpenAI-compatible provider | **Built-in** | Universal — one adapter covers cloud + local models |
| Specific cloud adapters (Azure, Vertex) | **Plugin** | Service-specific configuration and auth |
| Ollama/local model support | **Built-in** | Covered by OpenAI-compatible adapter (configuration variant) |
| Model routing per agent role | **Built-in config** | Part of project settings |
| GitHub integration | **Plugin** | Not all users use GitHub; domain-specific |
| Issue tracker sync (Linear, Jira) | **Plugin** | Service-specific |
| Custom dashboards/views | **Plugin** | User-specific needs |
| Component library | **Built-in SDK** | Plugins need shared components to render views |
| Artifact Graph SDK | **Built-in SDK** | Plugins need graph access to query artifacts |
| Rule enforcement engine | **Built-in** | Core governance infrastructure |
| CLI governance enforcement | **Plugin** | CLI-specific compatibility layer |

### Maintaining the Boundary

The boundary between built-in and plugin is maintained by architectural constraints:

1. **Core graph protection** — [RULE-98682b5e](RULE-98682b5e) prevents plugins from modifying schemas, core knowledge, or the orchestrator prompt
2. **Enforcement entry format** — Rules define enforcement declaratively in YAML frontmatter; both the built-in engine and CLI plugin consume the same format
3. **IPC boundary** — Tauri `invoke()` is the only frontend-backend interface; plugins cannot bypass it
4. **Knowledge layer field** — Every knowledge artifact declares its layer (`core`, `project`, `plugin`, `community`, `user`), enabling trust-level filtering

When evaluating whether a new feature should be built-in or a plugin, apply the criteria table above and discuss with the user if the answer is ambiguous. Err toward plugin — it is easier to promote a plugin to built-in than to extract a built-in feature into a plugin.

---

## Related Documents

- [RES-00ec6dd1](RES-00ec6dd1) — Multi-provider AI integration and plugin architecture research
- [IDEA-53205849](IDEA-53205849) — Component Library SDK for Plugin Views
- [IDEA-b77e2955](IDEA-b77e2955) — Plugin distribution model
- [IDEA-9713910f](IDEA-9713910f) — Integration ecosystem
- [RULE-98682b5e](RULE-98682b5e) — Core graph firmware protection
- [RULE-f9d0279c](RULE-f9d0279c) — Path-based knowledge injection
- [RULE-deab6ea7](RULE-deab6ea7) — Knowledge enforcement and tier model
