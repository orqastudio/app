# Connector Inventory: connectors/claude-code/

## Overview

The Claude Code connector is an npm package (`@orqastudio/claude-code-cli`, v0.1.4-dev) that bridges OrqaStudio's governance system with Claude Code's hook/plugin infrastructure. It is simultaneously:

1. A **Claude Code plugin** (`.claude-plugin/plugin.json` + hooks + skills + commands)
2. An **OrqaStudio plugin** (`orqa-plugin.json` with content, knowledge, hooks declarations)
3. A **reusable library** that re-exports graph/daemon APIs from `@orqastudio/cli`

---

## Source Files (src/)

### src/index.ts
- **Path:** `connectors/claude-code/src/index.ts`
- **Lines:** 26
- **What it does:** Package entry point. Re-exports graph browsing APIs (`scanArtifactGraph`, `queryGraph`, `getGraphStats`), daemon client functions (`callDaemonGraph`, `isDaemonRunning`), and plugin management (`installPlugin`, `uninstallPlugin`, `listInstalledPlugins`) from `@orqastudio/cli`. Also exports local modules `ArtifactBridge` and `runConnectorSetup`.
- **Exports:** `scanArtifactGraph`, `queryGraph`, `getGraphStats`, `GraphNode`, `GraphQueryOptions`, `GraphStats`, `callDaemonGraph`, `isDaemonRunning`, `DaemonArtifactNode`, `DaemonArtifactRef`, `DaemonHealthResponse`, `installPlugin`, `uninstallPlugin`, `listInstalledPlugins`, `ArtifactBridge`, `BridgeMapping`, `runConnectorSetup`, `ConnectorSetupResult`
- **External dependencies:** `@orqastudio/cli`
- **Business logic:** No. Pure re-export/marshalling.

### src/types.ts
- **Path:** `connectors/claude-code/src/types.ts`
- **Lines:** 83
- **What it does:** Defines TypeScript interfaces for the Claude Code hook JSON contract: `HookInput` (stdin from Claude Code), `HookToolInput` (tool-specific fields), `HookBlockOutput`/`HookWarnOutput` (stdout/stderr responses), `LoadedEnforcementEntry` (rule artifact enforcement entries), `RuleViolation`, `BashSafetyRule`, and `TelemetryDetails`.
- **Exports:** `HookInput`, `HookToolInput`, `HookBlockOutput`, `HookWarnOutput`, `LoadedEnforcementEntry`, `RuleViolation`, `BashSafetyRule`, `TelemetryDetails`
- **External dependencies:** None
- **Business logic:** No. Type definitions only.

### src/artifact-bridge.ts
- **Path:** `connectors/claude-code/src/artifact-bridge.ts`
- **Lines:** 284
- **What it does:** Maps the `.claude/` directory to OrqaStudio's `.orqa/` artifact graph via symlinks. The `ArtifactBridge` class manages three canonical mappings: `CLAUDE.md` -> orchestrator agent, `rules/` -> process rules, `agents/` -> process agents. Provides methods to create/maintain symlinks (`setupSymlinks`), resolve `.claude/` paths to `.orqa/` paths (`resolveToOrqa`), list agents/skills/rules from `.orqa/`, and report bridge status.
- **Exports:** `ArtifactBridge` (class), `BridgeMapping` (interface)
- **External dependencies:** `node:fs`, `node:path`, `@orqastudio/cli` (for `createSymlink`)
- **Business logic:** Contains structural logic for symlink management and artifact discovery (directory scanning, frontmatter parsing). This is translation/marshalling between two directory structures, not rule evaluation or scoring.

### src/connector-setup.ts
- **Path:** `connectors/claude-code/src/connector-setup.ts`
- **Lines:** 201
- **What it does:** Post-install setup for the Claude Code connector. Builds `.claude/agents/` as a merged directory containing symlinks to both core agents (from `app/.orqa/process/agents/` or `.orqa/process/agents/`) and plugin agents (from `plugins/*/orqa-plugin.json` `provides.agents` entries). Core agents take precedence over plugin agents. Handles migration from old-style symlink-to-directory format.
- **Exports:** `runConnectorSetup` (function), `ConnectorSetupResult` (interface)
- **External dependencies:** `node:fs`, `node:path`, `@orqastudio/cli` (for `ensureSymlink`)
- **Business logic:** Contains structural logic for merged agent directory construction and plugin manifest parsing. Translation/marshalling, not rule evaluation.

### src/hooks/shared.ts
- **Path:** `connectors/claude-code/src/hooks/shared.ts`
- **Lines:** 267
- **What it does:** Shared I/O infrastructure for all hook scripts. Key functions:
  - `readInput()` — reads JSON from stdin (Claude Code hook contract)
  - `callDaemon(path, body)` — HTTP POST to the Rust daemon at `localhost:{ORQA_PORT_BASE+58}`, falls back to spawning `orqa-validation hook --stdin` binary
  - `mapEvent(ccEvent)` — maps Claude Code event names to canonical OrqaStudio event names (PreToolUse->PreAction, PostToolUse->PostAction, etc.)
  - `outputBlock(messages)` — writes denial JSON to stderr, exits 2
  - `outputWarn(messages)` — writes warning JSON to stdout
  - `outputAllow()` — exits 0 silently
  - `isOrqaArtifact(filePath, projectDir)` — checks if a path is a `.orqa/*.md` file
  - MCP IPC client: `readIpcPort()`, `mcpSearchCall()` — TCP JSON-RPC connection to the OrqaStudio MCP server for semantic search, with initialize handshake and 4s timeout
  - `callBinary(path, body)` — fallback to `orqa-validation` binary via `spawnSync`
- **Exports:** `CanonicalEvent`, `HookContext`, `HookResult`, `readInput`, `callDaemon`, `mapEvent`, `outputBlock`, `outputWarn`, `outputAllow`, `isOrqaArtifact`, `SearchResult`, `getIpcPortFilePath`, `readIpcPort`, `mcpSearchCall`
- **External dependencies:** `node:child_process`, `node:fs`, `node:net`, `node:path`, `../types.js`
- **Business logic:** Contains the MCP IPC protocol implementation (JSON-RPC over TCP) and daemon port calculation. The port calculation (`ORQA_PORT_BASE + 58`) and event mapping are heuristic/structural logic. No rule evaluation or scoring.

### src/hooks/telemetry.ts
- **Path:** `connectors/claude-code/src/hooks/telemetry.ts`
- **Lines:** 35
- **What it does:** Fire-and-forget telemetry logger. `logTelemetry()` sends hook execution metrics (hook name, event, duration, outcome, details) to the dev controller dashboard at `localhost:10401/log` via HTTP POST. Never blocks or throws.
- **Exports:** `logTelemetry` (function)
- **External dependencies:** `../types.js` (for `TelemetryDetails`)
- **Business logic:** No. Pure telemetry marshalling.

### src/hooks/rule-engine.ts
- **Path:** `connectors/claude-code/src/hooks/rule-engine.ts`
- **Lines:** 48
- **What it does:** PreToolUse hook for Write, Edit, Bash, TeamCreate, and Agent tool calls. Thin adapter: reads stdin, builds a `HookContext` with `PreAction` event, calls daemon `POST /hook`, outputs block/warn/allow based on daemon response. All enforcement logic is in the Rust daemon.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`
- **Business logic:** No. Pure adapter/marshalling. Delegates ALL enforcement to the daemon.

### src/hooks/save-context.ts
- **Path:** `connectors/claude-code/src/hooks/save-context.ts`
- **Lines:** 112
- **What it does:** PreCompact hook. Queries the daemon for active epics and in-progress tasks (`POST /query`), reads existing session state, and writes a governance context file (`.state/governance-context.md`) preserving active work context before Claude Code's context compaction. Returns a `systemMessage` summary. Includes telemetry logging.
- **Exports:** None (executable entry point)
- **External dependencies:** `fs`, `path`, `./shared.js`, `./telemetry.js`
- **Business logic:** Contains logic for composing the governance context document (deciding what to preserve, formatting). This is business logic in the sense that it encodes what information matters during compaction. Medium complexity.

### src/hooks/memory-redirect.ts
- **Path:** `connectors/claude-code/src/hooks/memory-redirect.ts`
- **Lines:** 41
- **What it does:** PreToolUse hook for Write and Edit. Thin adapter: reads stdin, builds `HookContext`, calls daemon `POST /hook`. The daemon decides which memory paths should be redirected to lessons. Identical structure to `rule-engine.ts`.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`
- **Business logic:** No. Pure adapter.

### src/hooks/findings-check.ts
- **Path:** `connectors/claude-code/src/hooks/findings-check.ts`
- **Lines:** 54
- **What it does:** PostToolUse hook for TaskUpdate. Thin adapter: reads stdin, builds `HookContext` with `PostAction` event, calls daemon `POST /hook`. The daemon enforces RULE-04684a16 (findings-file existence check and deferral detection). Includes telemetry.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`, `./telemetry.js`
- **Business logic:** No. Pure adapter.

### src/hooks/departure-detection.ts
- **Path:** `connectors/claude-code/src/hooks/departure-detection.ts`
- **Lines:** 57
- **What it does:** UserPromptSubmit hook. Detects departure signals (user leaving the session) in the user's message and returns a session-state reminder. Thin adapter: reads stdin, builds `HookContext` with `PromptSubmit` event and `user_message`, calls daemon `POST /hook`. Includes telemetry.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`, `./telemetry.js`
- **Business logic:** No. Pure adapter. Departure pattern matching is in the daemon.

### src/hooks/subagent-review.ts
- **Path:** `connectors/claude-code/src/hooks/subagent-review.ts`
- **Lines:** 54
- **What it does:** SubagentStop hook. Delegates subagent review (stub detection, deferral scanning, artifact integrity) to the daemon. Thin adapter: reads stdin, builds `HookContext` with `SubagentStop` event and `agent_type`, calls daemon `POST /hook`. Includes telemetry.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`, `./telemetry.js`
- **Business logic:** No. Pure adapter.

### src/hooks/validate-artifact.ts
- **Path:** `connectors/claude-code/src/hooks/validate-artifact.ts`
- **Lines:** 83
- **What it does:** PostToolUse hook for Write and Edit to `.orqa/` files. Calls daemon `POST /parse` on the written file to get validation results. If validation errors are found, outputs a non-blocking systemMessage listing all errors with instructions to fix before committing. Uses `isOrqaArtifact()` to filter — only fires for `.orqa/*.md` files.
- **Exports:** None (executable entry point)
- **External dependencies:** `path`, `./shared.js`, `./telemetry.js`
- **Business logic:** Minimal. The filtering logic (`isOrqaArtifact`) and error message formatting are here. Actual schema validation is in the daemon.

### src/hooks/impact-check.ts
- **Path:** `connectors/claude-code/src/hooks/impact-check.ts`
- **Lines:** 88
- **What it does:** PostToolUse hook for Write and Edit to `.orqa/` files. Calls daemon `POST /parse` to get artifact type, high-influence flag, and downstream relationship count. If the artifact is high-influence or has >20 downstream relationships, injects an impact context warning. Uses `isOrqaArtifact()` to filter.
- **Exports:** None (executable entry point)
- **External dependencies:** `fs`, `path`, `./shared.js`, `./telemetry.js`
- **Business logic:** Contains a heuristic: `shouldInject = highInfluence || downstreamCount > 20`. This is a threshold decision. The rest is formatting/marshalling.

### src/hooks/knowledge-injector.ts
- **Path:** `connectors/claude-code/src/hooks/knowledge-injector.ts`
- **Lines:** 195
- **What it does:** PreToolUse hook for Agent tool calls. The most complex hook. Two knowledge injection layers:
  - **Layer 1 (Declared):** Reads the prompt registry (`.orqa/prompt-registry.json`) for knowledge entries matching the detected agent role. Uses `ROLE_PATTERNS` regex array to detect roles from prompt text (implementer, researcher, reviewer, planner, writer, designer, governance-steward).
  - **Layer 2 (Semantic):** Calls `search_semantic` via MCP TCP IPC to find task-specific knowledge beyond declared relationships. Uses `MIN_SCORE = 0.25` threshold and `MAX_SEMANTIC = 5` limit. Excludes already-declared knowledge IDs.
  - Injects both layers as a non-blocking `systemMessage` via `outputWarn()`.
- **Exports:** None (executable entry point)
- **External dependencies:** `./shared.js`, `./telemetry.js`, `@orqastudio/cli` (for `readPromptRegistry`, `queryKnowledge`, `RegistryKnowledgeEntry`)
- **Business logic:** YES. Contains role detection heuristics (`ROLE_PATTERNS`), semantic search scoring threshold (`MIN_SCORE = 0.25`), knowledge deduplication, query extraction from prompts. This is the richest business logic in the connector.

### src/hooks/prompt-injector.ts
- **Path:** `connectors/claude-code/src/hooks/prompt-injector.ts`
- **Lines:** 303
- **What it does:** UserPromptSubmit hook. The largest and most complex hook. Implements a three-tier prompt classification pipeline:
  - **Tier 1 (Semantic):** Uses ONNX semantic search via MCP IPC to classify the user prompt against thinking-mode knowledge artifacts. Matches `thinking-mode:` frontmatter values to `PromptType` enum.
  - **Tier 2 (Keyword):** Regex-based fallback classifier with patterns for implementation, debugging, review, planning, documentation, research, governance.
  - **Tier 3:** Default to "general".
  - After classification, calls `generatePrompt()` from `@orqastudio/cli` with the mapped workflow stage. Writes the full preamble to `.state/orchestrator-preamble.md`. Also reads `project.json` for dogfood mode context.
- **Exports:** None (executable entry point)
- **External dependencies:** `fs`, `path`, `./shared.js`, `./telemetry.js`, `@orqastudio/cli` (for `generatePrompt`, `PromptResult`)
- **Business logic:** YES. Contains prompt classification heuristics (semantic + keyword regex), thinking-mode mapping, workflow stage resolution, context line generation, preamble document composition. This is the core prompt pipeline entry point for the Claude Code connector.

---

## Configuration Files

### .claude-plugin/plugin.json
- **Path:** `connectors/claude-code/.claude-plugin/plugin.json`
- **Lines:** 9
- **What it does:** Claude Code plugin manifest. Declares the plugin as "orqastudio" version 0.1.0-dev by OrqaStudio. Minimal metadata — the real plugin configuration is in `hooks/hooks.json` and `orqa-plugin.json`.

### .claude-plugin/marketplace.json
- **Path:** `connectors/claude-code/.claude-plugin/marketplace.json`
- **Lines:** 17
- **What it does:** Local marketplace descriptor for the Claude Code plugin system. Defines a marketplace named "orqa-local" containing one plugin: "orqastudio" sourced from the current directory.

### package.json
- **Path:** `connectors/claude-code/package.json`
- **Lines:** 44
- **What it does:** npm package configuration for `@orqastudio/claude-code-cli` v0.1.4-dev.
  - **Type:** ES module (`"type": "module"`)
  - **Main:** `dist/index.js`
  - **Scripts:** `build` (tsc), `dev` (tsc --watch), `typecheck` (tsc --noEmit)
  - **Dependencies:** `@orqastudio/cli` 0.1.4-dev, `@orqastudio/types` 0.1.4-dev, `yaml` ^2.8.3
  - **Dev dependencies:** `@types/node` ^22.0.0, `typescript` ^5.7.0
  - **Files:** dist, src, hooks, skills, agents, commands, .claude-plugin, orqa-plugin.json
  - **License:** Apache-2.0
  - **Repository:** `git@github.com:orqastudio/claude-code-cli-plugin.git`

### orqa-plugin.json
- **Path:** `connectors/claude-code/orqa-plugin.json`
- **Lines:** 152
- **What it does:** OrqaStudio plugin manifest. Declares the connector as `@orqastudio/claude-code-connector` v0.1.0-dev, category "connector". Key sections:
  - **requires:** `@orqastudio/plugin-agile-workflow`, `@orqastudio/plugin-systems-thinking`
  - **content.knowledge:** source `knowledge/` -> target `.orqa/process/knowledge`
  - **dependencies.npm:** `@orqastudio/cli`, `@orqastudio/types`, `yaml`
  - **build:** `npm run build`
  - **lifecycle.install:** `node dist/index.js setup`
  - **provides.knowledge:** 10 knowledge entries (artifact-creation, artifact-ids, decision-tree, delegation-patterns, implementer-tree, project-migration, project-inference, reviewer-tree, rule-enforcement, tool-mapping)
  - **provides.hooks:** 7 hook declarations (PreToolUse, UserPromptSubmit, SessionStart, Stop, PostToolUse, PreCompact, SubagentStop)
  - **provides.lspServers:** `orqastudio` (`orqa lsp` for `.md` files)
  - **provides.mcpServers:** `orqastudio` (`orqa mcp`)
  - **provides.symlinks:** `.orqa/process/rules` -> `.claude/rules`
  - **provides.aggregatedFiles:** `.mcp.json` (collected from `provides.mcpServers`), `.lsp.json` (collected from `provides.lspServers`)

### tsconfig.json
- **Path:** `connectors/claude-code/tsconfig.json`
- **Lines:** 20
- **What it does:** TypeScript configuration targeting ES2022 with NodeNext module resolution. Outputs to `dist/` with declaration files, declaration maps, and source maps. Strict mode enabled.

### .lsp.json
- **Path:** `connectors/claude-code/.lsp.json`
- **Lines:** 38
- **What it does:** LSP server configuration. Defines four language servers:
  - `rust` — rust-analyzer for `.rs` files
  - `svelte` — svelteserver (--stdio) for `.svelte` files
  - `typescript` — typescript-language-server (--stdio) for `.ts`, `.tsx`, `.js`, `.jsx`
  - `orqastudio` — `orqa lsp` for `.md` files

### .gitignore
- **Path:** `connectors/claude-code/.gitignore`
- **Lines:** 5
- **Contents:** Ignores `node_modules/`, `.claude/`, `.mcp.json`, `.lsp.json` (the latter two are generated at install time)

---

## Hooks Configuration

### hooks/hooks.json
- **Path:** `connectors/claude-code/hooks/hooks.json`
- **Lines:** 159
- **What it does:** Claude Code hooks wiring. Defines which hook scripts run for which events and tool matchers:

| Event | Matcher | Script | Timeout |
|-------|---------|--------|---------|
| PreToolUse | TeamCreate | `dist/hooks/rule-engine.js` | 10s |
| PreToolUse | Agent | `dist/hooks/rule-engine.js` | 10s |
| PreToolUse | Agent | `dist/hooks/knowledge-injector.js` | 5s |
| PreToolUse | Write\|Edit\|Bash | `dist/hooks/rule-engine.js` | 10s |
| PreToolUse | Write\|Edit | `dist/hooks/memory-redirect.js` | 5s |
| PostToolUse | Write\|Edit | `dist/hooks/validate-artifact.js` | 10s |
| PostToolUse | TaskUpdate | `dist/hooks/findings-check.js` | 5s |
| UserPromptSubmit | * | `hooks/scripts/daemon-gate.sh` | 5s |
| UserPromptSubmit | * | `dist/hooks/prompt-injector.js` | 10s |
| UserPromptSubmit | * | `dist/hooks/departure-detection.js` | 5s |
| SessionStart | * | `hooks/scripts/session-start.sh` | 15s |
| Stop | * | `hooks/scripts/stop-checklist.sh` | 10s |
| PreCompact | * | `dist/hooks/save-context.js` | 10s |
| SubagentStop | * | `dist/hooks/subagent-review.js` | 15s |

**Total: 14 hook registrations across 6 events.**

### hooks/scripts/daemon-gate.sh
- **Path:** `connectors/claude-code/hooks/scripts/daemon-gate.sh`
- **Lines:** 31
- **What it does:** UserPromptSubmit blocking gate. Checks if the OrqaStudio daemon is reachable via `curl` to `http://127.0.0.1:{PORT}/health` (2s timeout). If unreachable, blocks the interaction (exit 2) with a message telling the user to start the daemon. Port = `ORQA_PORT_BASE` (default 10200) + 58.

### hooks/scripts/session-start.sh
- **Path:** `connectors/claude-code/hooks/scripts/session-start.sh`
- **Lines:** 128
- **What it does:** SessionStart hook (runs once per session via guard file `.state/.session-started`). Performs:
  1. Connector installation check (verifies `.claude/agents`, `.claude/rules`, `.claude/CLAUDE.md` exist)
  2. Daemon health gate (blocks if daemon unreachable)
  3. Graph integrity check (`orqa enforce --fix`)
  4. Git state warnings (stash list, uncommitted files on main)
  5. Session continuity (loads `.state/migration-context.md`, `.state/session-state.md`, `.state/governance-context.md`)
  6. Dogfood mode detection (checks `project.json` for `"dogfood": true`)
  7. Session start checklist output

### hooks/scripts/stop-checklist.sh
- **Path:** `connectors/claude-code/hooks/scripts/stop-checklist.sh`
- **Lines:** 57
- **What it does:** Stop hook (session end). Safety net for session state: if the orchestrator didn't maintain `.state/session-state.md`, auto-generates a minimal one with recent commits and uncommitted file count. Preserves orchestrator-maintained state if structured sections are present.

---

## Commands (Slash Commands)

### commands/orqa.md
- **Path:** `connectors/claude-code/commands/orqa.md`
- **Lines:** 95
- **What it does:** `/orqa` slash command. Comprehensive governance summary and graph browser reference. Documents the `.claude/` -> `.orqa/` symlink mappings, CLI usage for graph browsing (`orqa graph --stats`, `--type`, `--id`, `--related-to`, `--search`, `--tree`, `--json`), artifact type hierarchy (pillars, decisions, rules, lessons, knowledge, agents), relationship vocabulary table, process workflow instructions, and quick actions.

### commands/orqa-save.md
- **Path:** `connectors/claude-code/commands/orqa-save.md`
- **Lines:** 49
- **What it does:** `/orqa-save` slash command. Instructs writing session state to `.state/session-state.md` with structured sections: Scope, What Was Done, In Progress, Next Steps, Blockers, Commits This Session. Includes rules: always overwrite, use absolute dates, be specific.

### commands/orqa-create.md
- **Path:** `connectors/claude-code/commands/orqa-create.md`
- **Lines:** 79
- **What it does:** `/orqa-create` slash command. Guided artifact creation with YAML frontmatter. Covers: artifact type selection (task, epic, idea, decision, rule, lesson, research), ID allocation, relationship determination, artifact writing with governance-steward delegation, and validation via `orqa enforce`.

### commands/orqa-enforce.md
- **Path:** `connectors/claude-code/commands/orqa-enforce.md`
- **Lines:** 58
- **What it does:** `/orqa-validate` slash command (note: filename says "enforce" but the content is titled "orqa-validate"). Full integrity check reference. Documents `orqa enforce` CLI usage, severity levels (ERROR/WARNING/INFO), common issues (missing inverse relationships, invalid status, missing required fields), and the 0 errors/0 warnings baseline.

---

## Knowledge Files

### knowledge/artifact-creation/KNOW.md
- **ID:** KNOW-816ebef3
- **Lines:** 164
- **Description:** How to create valid OrqaStudio artifacts. Covers frontmatter requirements (required fields: id, type, status), artifact status discovery (via MCP, plugin files, schema.json), ID allocation, relationship protocol (always bidirectional), and common patterns for tasks, epics, and decisions.

### knowledge/artifact-ids/KNOW.md
- **ID:** KNOW-9573eeea
- **Lines:** 44
- **Description:** Artifact ID format: `TYPE-XXXXXXXX` (8 lowercase hex chars). Covers generation methods (CLI, shell, code), rules (prefix must match type, exactly 8 hex chars, location-independent, globally unique), and legacy ID compatibility.

### knowledge/decision-tree/KNOW.md
- **ID:** KNOW-3155cdaa
- **Lines:** 79
- **Description:** Orchestrator reasoning protocol. Four-step decision tree: (1) Classify context (implementation, research, planning, feedback/bug, learning loop, review, documentation), (2) Understand what the classification means, (3) Form the right search question, (4) Delegate with context. Employed by AGENT-4c94fe14 (orchestrator).

### knowledge/delegation-patterns/KNOW.md
- **ID:** KNOW-ac314f61
- **Lines:** 64
- **Description:** Delegation patterns for the orchestrator. When to delegate (code->Implementer, .orqa/->Governance Steward, docs->Writer, quality->Reviewer, etc.), subagents vs agent teams decision criteria, delegation protocol (4 steps), role boundaries table, team coordination rules.

### knowledge/implementer-tree/KNOW.md
- **ID:** KNOW-b1593311
- **Lines:** 43
- **Description:** Implementer reasoning protocol. Three-step decision tree: (1) Understand the domain layer (Rust backend, Frontend UI, Reactive state, Governance artifacts), (2) Form the right question and search, (3) Build and verify (all layers, `make check`, acceptance criteria). Employed by AGENT-e5dd38e4 (implementer).

### knowledge/reviewer-tree/KNOW.md
- **ID:** KNOW-08fcd847
- **Lines:** 43
- **Description:** Reviewer reasoning protocol. Three-step decision tree: (1) Understand what is being reviewed (backend, frontend, governance, plan), (2) Form the right question and search for applicable rules/standards, (3) Produce verdict (PASS or FAIL with evidence). Employed by AGENT-8e58cd87 (reviewer).

### knowledge/rule-enforcement/KNOW.md
- **ID:** KNOW-bcfeb64e
- **Lines:** 55
- **Description:** How rule enforcement works in the CLI plugin. Documents the enforcement entry format (YAML in rule frontmatter), event types (`file` for Write/Edit, `bash` for Bash), actions (`block`, `warn`, `inject`), and how to add enforcement to a rule.

### knowledge/tool-mapping/KNOW.md
- **ID:** KNOW-b0b55e54
- **Lines:** 147
- **Description:** MCP tool selection guide. Maps governance operations to tools: `graph_resolve` (by ID), `graph_query` (by type/status), `graph_relationships` (connections), `search_semantic` (concept-level, two scopes: artifacts and codebase), `search_regex` (exact patterns), `search_research` (end-to-end understanding), `graph_validate` (health check). Includes mandatory pre-delegation steps.

### knowledge/KNOW-4a58e7dd.md
- **ID:** KNOW-4a58e7dd
- **Lines:** 115
- **Title:** Project Migration
- **Description:** Maps existing agentic tool configurations (Claude Code, Cursor, Copilot, Aider) into OrqaStudio's governance structure. Covers source format detection, coexistence strategies (symlinks for Claude Code, generated files for others), migration procedure (7 steps), content extraction patterns, and governance hub mode.

### knowledge/KNOW-03421ec0.md
- **ID:** KNOW-03421ec0
- **Lines:** 134
- **Title:** Project Inference
- **Description:** Reads a project's folder structure to infer characteristics: languages, frameworks, build tools, existing governance, and project type. Produces a structured YAML profile. Detection covers 8+ languages, 10+ frameworks, 6+ build tools, 9 governance tool configurations, and 6 project type patterns.

---

## Skills

### skills/diagnostic-methodology/SKILL.md
- **Lines:** 77
- **User-invocable:** Yes
- **Description:** Root cause analysis methodology: capture, reproduce, isolate, fix, verify. Covers the 5-step debug process (strict sequence), root cause classification table (7 categories: logic error, type error, state error, integration error, data error, race condition, configuration error), anti-patterns table, and critical rules.

### skills/search/SKILL.md
- **Lines:** 69
- **User-invocable:** Yes
- **Description:** Unified search skill. Three MCP search modes: `search_regex` (exact patterns), `search_semantic` (natural language via ONNX), `search_research` (compound query with semantic + symbol extraction + regex follow-up). Includes tool selection decision table and `search_status` health check.

### skills/governance-context/SKILL.md
- **Lines:** 99
- **User-invocable:** No (agent-internal)
- **Description:** How to read and use OrqaStudio governance data. Covers the artifact graph structure (where each type lives), reading the graph (relationship traversal), relationship vocabulary (10 forward/inverse pairs), 12 canonical statuses, rules, schema validation, type constraints (from core.json), and MCP knowledge discovery.

### skills/planning/SKILL.md
- **Lines:** 256
- **User-invocable:** No (agent-internal)
- **Description:** Documentation-first planning methodology. Enforces Document -> Approve -> Implement -> Verify workflow. Covers the collaborative design workflow (5 phases), pre-implementation documentation checklist, plan structure requirements (architectural compliance, systems architecture checklist with 8 dimensions, UX-first design, verification gates), documentation drift prevention rules, and the plan structure template. The largest skill file.

### skills/plugin-setup/SKILL.md
- **Lines:** 312
- **User-invocable:** Yes
- **Description:** OrqaStudio plugin setup guide. Covers detection phase (3 checks: .orqa/ exists, .claude/ file types, plugin installed), migration path (5 steps: migrate CLAUDE.md, rules, agents, knowledge, backup originals), plugin installation (5 steps: clone, register marketplace, register plugin, enable in settings, verify), fresh install path, what the plugin provides, what stays in .claude/, platform notes (Windows symlinks vs macOS/Linux), and LSP server setup.

---

## Shell Scripts (hooks/scripts/)

| Script | Event | Lines | Purpose |
|--------|-------|-------|---------|
| `daemon-gate.sh` | UserPromptSubmit | 31 | Block if daemon unreachable |
| `session-start.sh` | SessionStart | 128 | Installation check, daemon gate, graph integrity, git state, session continuity, dogfood mode |
| `stop-checklist.sh` | Stop | 57 | Auto-generate minimal session state if orchestrator didn't |

---

## CI/CD Workflows (.github/workflows/)

### ci.yml
- **Lines:** 23
- **Trigger:** Push to main, PRs to main
- **What it does:** Install dependencies, run TypeScript check (`tsc --noEmit`), build (`npm run build`). Node 22 on ubuntu-latest.

### publish-dev.yml
- **Lines:** 35
- **Trigger:** Push to main
- **What it does:** Builds the package, sets a dev version using git SHA (`0.1.0-dev.{SHA}`), publishes to GitHub Packages registry with `--tag dev`.

### publish.yml
- **Lines:** 35
- **Trigger:** GitHub release published
- **What it does:** Builds the package, validates that the git tag matches `package.json` version, publishes to GitHub Packages registry.

---

## State Files (.state/)

### .state/orchestrator-preamble.md
- **Lines:** 22
- **What it does:** Written by the `prompt-injector.ts` hook on each UserPromptSubmit. Records the prompt classification (type and method), project context, session state status, and pipeline output (sections included/trimmed, token budget). Snapshot of last run shows: classified as "governance" via keyword, no pipeline sections, 0/2500 tokens.

---

## Other Files

### tmp/governance-context.md
- **Lines:** 12
- **What it does:** Governance context saved before compaction by the `save-context.ts` hook. Contains timestamp, recovery instructions. The snapshot has no active epics or tasks.

### thumbnail.png
- **Lines:** N/A (binary)
- **What it does:** Plugin thumbnail image for the Claude Code plugin marketplace.

---

## Summary Statistics

| Category | Count |
|----------|-------|
| Source files (src/) | 14 (.ts files) |
| Total source lines | ~1,571 |
| Hook scripts (TypeScript) | 10 |
| Shell scripts | 3 |
| Knowledge files | 10 |
| Skills | 5 |
| Commands | 4 |
| CI/CD workflows | 3 |
| Config files | 7 (plugin.json, marketplace.json, package.json, orqa-plugin.json, tsconfig.json, .lsp.json, .gitignore) |

### Business Logic Concentration

Files containing substantive business logic (rule evaluation, scoring, heuristics, knowledge injection, prompt assembly):

| File | Lines | Business Logic Type |
|------|-------|---------------------|
| `prompt-injector.ts` | 303 | Prompt classification (semantic + keyword), workflow stage mapping, pipeline invocation, preamble composition |
| `knowledge-injector.ts` | 195 | Role detection heuristics, semantic search scoring threshold, knowledge deduplication, query extraction |
| `save-context.ts` | 112 | Governance context composition (what to preserve during compaction) |
| `impact-check.ts` | 88 | Downstream relationship threshold heuristic (>20 = inject warning) |
| `shared.ts` | 267 | MCP IPC protocol, daemon port calculation, event mapping |

All other hook files (`rule-engine.ts`, `memory-redirect.ts`, `findings-check.ts`, `departure-detection.ts`, `subagent-review.ts`, `validate-artifact.ts`) are thin adapters that delegate all logic to the Rust daemon via `POST /hook` or `POST /parse`.
