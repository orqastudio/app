# Migration Tasks: Phases 4-5

Exhaustive, atomic task lists derived from ARCHITECTURE.md section 13 (phases 4-5), file-audit/phase2-03-connector-gaps.md, and file-audit/phase2-02-plugin-gaps.md.

**Prerequisite:** Phases 1-3 must be complete. The daemon must have endpoints POST /hook, POST /parse, POST /query operational. Engine crates (graph, enforcement, search, workflow, plugin, prompt, agent) must be extracted.

---

## Phase 4: Connector Cleanup

Phase 4 refactors the Claude Code connector from a business-logic-containing application into a pure generation + translation layer. Every boundary violation identified in phase2-03-connector-gaps.md has a corresponding task.

### Phase 4 Prerequisites (Engine Daemon Endpoints)

These tasks create the daemon endpoints that the connector refactoring depends on. They are Phase 2/3 deliverables but listed here for completeness — Phase 4 cannot start without them.

#### TASK P4-PRE-1: Build POST /prompt daemon endpoint

**What:** Create a new daemon HTTP endpoint `POST /prompt` in the Rust daemon that accepts `{ message: string, role: string, project_path: string }` and returns `{ prompt: string, type: string, method: string, tokens: number, budget: number, sections: object[] }`. This endpoint must absorb: (a) prompt classification (semantic search against thinking-mode artifacts + keyword regex fallback), (b) thinking-mode resolution (THINKING_MODE_MAP mappings), (c) workflow stage mapping (promptTypeToStage), (d) preamble composition, (e) full prompt generation via the prompt crate's pipeline.

**Acceptance Criteria:**
- [ ] Endpoint accepts POST with JSON body `{ message, role, project_path }`
- [ ] Returns classified prompt type, generation method, composed prompt text, token count, budget, and section list
- [ ] Semantic classification (Tier 1) uses ONNX search against thinking-mode knowledge
- [ ] Keyword classification (Tier 2) uses regex patterns as fallback
- [ ] Default (Tier 3) returns "general" type
- [ ] Thinking-mode mappings (e.g., "learning-loop" -> "governance") are loaded from workflow plugin definitions, not hardcoded
- [ ] Workflow stage resolution maps prompt type to methodology stage
- [ ] Preamble document is composed as part of the response

**Reviewer Checks:**
- Verify no classification heuristics remain outside the engine
- Verify thinking-mode mappings come from plugin definitions, not hardcoded strings
- Verify the endpoint returns enough data for the connector to format output without additional logic

---

#### TASK P4-PRE-2: Build POST /knowledge daemon endpoint

**What:** Create a new daemon HTTP endpoint `POST /knowledge` in the Rust daemon that accepts `{ agent_prompt: string, project_path: string }` and returns `{ entries: [{ id: string, title: string, path: string, source: string, score?: number }] }`. This endpoint must absorb: (a) role detection from prompt text (ROLE_PATTERNS regex), (b) prompt registry query for declared knowledge by role, (c) semantic search with MIN_SCORE=0.25 threshold and MAX_SEMANTIC=5 limit, (d) deduplication of declared vs semantic results, (e) query extraction from agent prompt.

**Acceptance Criteria:**
- [ ] Endpoint accepts POST with JSON body `{ agent_prompt, project_path }`
- [ ] Returns deduplicated knowledge entries with id, title, path, source, and optional score
- [ ] Role detection uses the same patterns currently in knowledge-injector.ts ROLE_PATTERNS
- [ ] Declared knowledge (Layer 1) comes from prompt registry by detected role
- [ ] Semantic knowledge (Layer 2) comes from ONNX search with configurable threshold/limit
- [ ] Already-declared IDs are excluded from semantic results
- [ ] Search query is extracted from the agent prompt

**Reviewer Checks:**
- Verify role detection patterns match or exceed current connector patterns
- Verify scoring threshold and max results are configurable (not hardcoded in the endpoint)
- Verify deduplication logic prevents double-injection

---

#### TASK P4-PRE-3: Build POST /compact-context daemon endpoint

**What:** Create a new daemon HTTP endpoint `POST /compact-context` in the Rust daemon that accepts `{ project_path: string }` and returns `{ context_document: string, summary: string }`. This endpoint must absorb: (a) querying active epics and in-progress tasks, (b) reading existing session state, (c) composing the governance context document with recovery instructions (without hardcoded artifact path references like `.orqa/process/agents/orchestrator.md`).

**Acceptance Criteria:**
- [ ] Endpoint accepts POST with JSON body `{ project_path }`
- [ ] Returns the full context document text and a summary string
- [ ] Queries active epics via graph engine
- [ ] Queries in-progress tasks via graph engine
- [ ] Reads existing session state from `.state/session-state.md`
- [ ] Composed document does NOT reference hardcoded paths to agent files
- [ ] Recovery instructions reference dynamically-resolved agent sources

**Reviewer Checks:**
- Verify no hardcoded `.orqa/process/agents/` references
- Verify the document structure preserves all information currently in save-context.ts composition

---

#### TASK P4-PRE-4: Build POST /session-start daemon endpoint

**What:** Create a new daemon HTTP endpoint `POST /session-start` in the Rust daemon that accepts `{ project_path: string }` and returns `{ checks: object[], warnings: string[], session_state: string, checklist: string[] }`. This endpoint must absorb: (a) installation verification, (b) daemon health status, (c) graph integrity check (orqa enforce --fix), (d) git state inspection, (e) session continuity (loading .state/ files), (f) dogfood mode detection.

**Acceptance Criteria:**
- [ ] Endpoint accepts POST with JSON body `{ project_path }`
- [ ] Returns structured health check results, warnings, loaded session state, and checklist items
- [ ] Installation check verifies `.claude/agents`, `.claude/rules`, `.claude/CLAUDE.md` exist
- [ ] Graph integrity runs enforcement with fix mode
- [ ] Git state warnings include stash list and uncommitted files
- [ ] Session state loads from `.state/migration-context.md`, `.state/session-state.md`, `.state/governance-context.md`
- [ ] Dogfood mode detected from `project.json`

**Reviewer Checks:**
- Verify all current session-start.sh checks are covered
- Verify response is structured enough for the connector to format without additional logic

---

#### TASK P4-PRE-5: Enhance POST /parse to include should_warn field

**What:** Modify the existing daemon `POST /parse` endpoint response to include a `should_warn: boolean` (or `impact_level: "high" | "low"`) field. The daemon already provides `high_influence` and `downstream_count`; add the threshold decision (`high_influence || downstream_count > 20`) to the daemon response so the connector does not need to evaluate this heuristic.

**Acceptance Criteria:**
- [ ] POST /parse response includes `should_warn: boolean` field
- [ ] `should_warn` is true when `high_influence == true` OR `downstream_count > 20`
- [ ] Threshold is configurable in daemon config (not hardcoded in the endpoint handler)
- [ ] Existing response fields (`high_influence`, `downstream_count`) remain for backwards compatibility

**Reviewer Checks:**
- Verify threshold is not hardcoded in the endpoint code
- Verify existing callers are not broken by the new field

---

### Phase 4 Connector Refactoring Tasks

> **Review against architecture -> keep/adapt/drop. Never blind copy.** Each refactoring and file move must be validated against ARCHITECTURE.md to confirm the target location and approach are correct.

#### TASK P4-01: Refactor prompt-injector.ts to thin adapter (MAJOR — addresses phase2-03 prompt-injector violation)

**What:** Rewrite `connectors/claude-code/src/hooks/prompt-injector.ts` from 303 lines to ~40 lines. Delete: `classifyWithSearch()`, `classifyPrompt()`, `resolveThinkingMode()`, `THINKING_MODE_MAP`, `promptTypeToStage()`, preamble document composition (lines 166-190), prompt classification heuristics (lines 42-79, 237-280), thinking-mode resolution (lines 30-43), workflow stage mapping (lines 86-97). Keep: `getContextLine()` (lines 287-300, connector-specific UX), input parsing, output formatting. Add: single `callDaemon("/prompt", { message, role, project_path })` call.

**Files modified:**
- `connectors/claude-code/src/hooks/prompt-injector.ts` — rewrite

**Acceptance Criteria:**
- [ ] File is ~40 lines (tolerance: 30-60)
- [ ] No `classifyWithSearch`, `classifyPrompt`, `resolveThinkingMode`, `THINKING_MODE_MAP`, or `promptTypeToStage` functions exist
- [ ] No regex-based keyword classification patterns exist in the file
- [ ] No semantic search calls exist in the file
- [ ] No preamble composition or `.state/orchestrator-preamble.md` writing exists in the file
- [ ] Single `callDaemon("/prompt", ...)` call handles all classification + generation
- [ ] `getContextLine()` is preserved (reads project.json for dogfood mode context line)
- [ ] Output is a Claude Code `systemMessage` containing the daemon's composed prompt + context line
- [ ] No imports from `@orqastudio/cli` (generatePrompt, PromptResult removed)

**Reviewer Checks:**
- Verify zero business logic remains — only I/O marshalling and presentation formatting
- Verify the daemon call provides all data needed (no post-processing of daemon response beyond formatting)
- Verify `getContextLine()` is the only function that reads local state

---

#### TASK P4-02: Refactor knowledge-injector.ts to thin adapter (MAJOR — addresses phase2-03 knowledge-injector violation)

**What:** Rewrite `connectors/claude-code/src/hooks/knowledge-injector.ts` from 195 lines to ~35 lines. Delete: `ROLE_PATTERNS` regex array (lines 42-57), `detectRole()`, `getDeclaredKnowledge()` (prompt registry queries, lines 66-84), `searchSemanticKnowledge()` (MCP IPC semantic search, lines 130-159), `MIN_SCORE = 0.25` constant, `MAX_SEMANTIC = 5` constant, query extraction logic (lines 98-100), knowledge deduplication logic. Keep: input parsing, output formatting via `outputWarn()`, telemetry. Add: single `callDaemon("/knowledge", { agent_prompt, project_path })` call.

**Files modified:**
- `connectors/claude-code/src/hooks/knowledge-injector.ts` — rewrite

**Acceptance Criteria:**
- [ ] File is ~35 lines (tolerance: 25-50)
- [ ] No `ROLE_PATTERNS`, `MIN_SCORE`, `MAX_SEMANTIC` constants exist
- [ ] No `detectRole`, `getDeclaredKnowledge`, `searchSemanticKnowledge` functions exist
- [ ] No MCP IPC calls (`mcpSearchCall`) exist in the file
- [ ] No prompt registry reads exist in the file
- [ ] Single `callDaemon("/knowledge", ...)` call returns knowledge entries
- [ ] Knowledge entries formatted as Claude Code `systemMessage` via `outputWarn()`
- [ ] Empty entries result in `outputAllow()` (no injection)
- [ ] No imports from `@orqastudio/cli` (readPromptRegistry, queryKnowledge removed)

**Reviewer Checks:**
- Verify zero knowledge injection heuristics remain
- Verify no direct MCP/TCP connections
- Verify telemetry still fires correctly

---

#### TASK P4-03: Refactor save-context.ts to thin adapter (MODERATE — addresses phase2-03 save-context violation)

**What:** Rewrite `connectors/claude-code/src/hooks/save-context.ts` from 112 lines to ~30 lines. Delete: dual daemon queries (POST /query for epics and tasks), governance context document composition (lines 47-79), recovery instructions with hardcoded path `.orqa/process/agents/orchestrator.md` (line 77). Keep: file writing to `.state/governance-context.md` (file I/O is connector-specific), building Claude Code `systemMessage` summary. Add: single `callDaemon("/compact-context", { project_path })` call.

**Files modified:**
- `connectors/claude-code/src/hooks/save-context.ts` — rewrite

**Acceptance Criteria:**
- [ ] File is ~30 lines (tolerance: 20-45)
- [ ] No dual POST /query calls to daemon
- [ ] No governance context document composition logic
- [ ] No hardcoded path reference to `.orqa/process/agents/orchestrator.md`
- [ ] Single `callDaemon("/compact-context", ...)` call returns context document + summary
- [ ] Daemon's `context_document` written to `.state/governance-context.md`
- [ ] Daemon's `summary` used for Claude Code `systemMessage`
- [ ] Telemetry still fires

**Reviewer Checks:**
- Verify no context composition logic remains
- Verify `.state/governance-context.md` write is preserved
- Verify no hardcoded artifact paths

---

#### TASK P4-04: Simplify impact-check.ts (MINOR — addresses phase2-03 impact-check violation)

**What:** Modify `connectors/claude-code/src/hooks/impact-check.ts` (88 lines -> ~75 lines). Delete: the threshold heuristic `shouldInject = highInfluence || downstreamCount > 20` (line 57). Replace with: reading the daemon's `should_warn` field from the POST /parse response.

**Files modified:**
- `connectors/claude-code/src/hooks/impact-check.ts` — modify 1 line + adjust response reading

**Acceptance Criteria:**
- [ ] No `downstreamCount > 20` threshold exists in the file
- [ ] No `shouldInject` computation exists (if it was computed locally)
- [ ] Uses `result.should_warn` (or equivalent) from daemon POST /parse response
- [ ] All other formatting/marshalling logic unchanged
- [ ] Telemetry unchanged

**Reviewer Checks:**
- Verify the threshold decision is fully in the daemon
- Verify no other heuristics snuck in

---

#### TASK P4-05: Remove MCP IPC from shared.ts (MODERATE — addresses phase2-03 shared.ts violation)

**What:** Modify `connectors/claude-code/src/hooks/shared.ts` (267 lines -> ~135 lines). Delete: `mcpSearchCall()` function (lines 170-242, ~73 lines), `readIpcPort()` function (lines 145-163, ~11 lines), `getIpcPortFilePath()` function (lines ~5 lines), all JSON-RPC-over-TCP protocol code, MCP initialize handshake code. Keep: `readInput()`, `callDaemon()`, `mapEvent()`, `outputBlock()`, `outputWarn()`, `outputAllow()`, `isOrqaArtifact()`, `callBinary()`. Remove exports: `SearchResult`, `getIpcPortFilePath`, `readIpcPort`, `mcpSearchCall`.

**Files modified:**
- `connectors/claude-code/src/hooks/shared.ts` — delete ~130 lines
- Any files importing `mcpSearchCall`, `readIpcPort`, `getIpcPortFilePath` — update imports

**Acceptance Criteria:**
- [ ] No `mcpSearchCall` function exists
- [ ] No `readIpcPort` function exists
- [ ] No `getIpcPortFilePath` function exists
- [ ] No `node:net` import (TCP connection code removed)
- [ ] No JSON-RPC protocol implementation
- [ ] No MCP initialize handshake code
- [ ] `SearchResult`, `getIpcPortFilePath`, `readIpcPort`, `mcpSearchCall` removed from exports
- [ ] All remaining functions (`readInput`, `callDaemon`, `mapEvent`, `outputBlock`, `outputWarn`, `outputAllow`, `isOrqaArtifact`, `callBinary`) work correctly
- [ ] File is ~135 lines (tolerance: 100-160)
- [ ] No other files import the removed functions (grep confirms zero references)

**Reviewer Checks:**
- Verify no TCP/IPC code remains anywhere in the connector
- Verify all callers of removed functions have been updated (P4-01 and P4-02 should eliminate all callers)
- Verify `callDaemon()` is the only mechanism for engine communication

**Dependencies:** Run AFTER P4-01 and P4-02 (which eliminate the callers of mcpSearchCall)

---

#### TASK P4-06: Delete artifact-bridge.ts (LEGACY — addresses phase2-03 artifact-bridge violation)

**What:** Delete `connectors/claude-code/src/artifact-bridge.ts` (284 lines). This file is legacy — its symlink management is superseded by `connector-setup.ts` (for agents) and the plugin framework's `provides.symlinks` (for rules). Also remove the `ArtifactBridge` and `BridgeMapping` exports from `connectors/claude-code/src/index.ts`.

**Files modified:**
- `connectors/claude-code/src/artifact-bridge.ts` — DELETE
- `connectors/claude-code/src/index.ts` — remove `ArtifactBridge` and `BridgeMapping` exports

**Acceptance Criteria:**
- [ ] `src/artifact-bridge.ts` does not exist
- [ ] `ArtifactBridge` is not exported from `src/index.ts`
- [ ] `BridgeMapping` is not exported from `src/index.ts`
- [ ] No other file in the connector imports from `./artifact-bridge` or `../artifact-bridge`
- [ ] TypeScript compiles successfully (`npx tsc --noEmit`)

**Reviewer Checks:**
- Grep entire connector for `artifact-bridge`, `ArtifactBridge`, `BridgeMapping` — zero hits
- Verify no downstream consumers of these exports exist in the monorepo

---

#### TASK P4-07: Simplify connector-setup.ts (TRANSITIONAL — addresses phase2-03 connector-setup violations)

**What:** Modify `connectors/claude-code/src/connector-setup.ts` (201 lines) to remove hardcoded path assumptions and direct plugin manifest parsing. (a) Remove hardcoded `app/.orqa/process/agents/` fallback path (lines 60-62) — agent sources should be resolved via the CLI's plugin registry APIs. (b) Replace direct `orqa-plugin.json` file parsing (lines 147-169) with CLI plugin registry API calls.

**Files modified:**
- `connectors/claude-code/src/connector-setup.ts` — modify

**Acceptance Criteria:**
- [ ] No hardcoded `app/.orqa/process/agents/` path exists
- [ ] No direct `fs.readFileSync` of `orqa-plugin.json` files
- [ ] Agent sources resolved via CLI plugin registry APIs (e.g., `listInstalledPlugins()` or daemon call)
- [ ] Plugin agent declarations discovered via registry, not raw file parsing
- [ ] Core agents still take precedence over plugin agents in merged directory
- [ ] TypeScript compiles successfully

**Reviewer Checks:**
- Verify no hardcoded monorepo structure knowledge (no `app/` path references)
- Verify plugin discovery uses official APIs, not filesystem assumptions

---

#### TASK P4-08: Simplify session-start.sh (MODERATE — addresses phase2-03 session-start violation)

**What:** Rewrite `connectors/claude-code/hooks/scripts/session-start.sh` from 128 lines to ~40 lines. Replace the six separate concerns (installation check, daemon health gate, graph integrity, git state, session continuity, dogfood detection, checklist output) with a single `curl` call to the daemon `POST /session-start` endpoint. Keep: session guard (`.state/.session-started` check), output formatting for Claude Code.

**Files modified:**
- `connectors/claude-code/hooks/scripts/session-start.sh` — rewrite

**Acceptance Criteria:**
- [ ] File is ~40 lines (tolerance: 25-60)
- [ ] Session guard (`.state/.session-started` file check) preserved
- [ ] Single `curl` call to `POST /session-start` replaces all manual checks
- [ ] No manual installation verification logic (delegated to daemon)
- [ ] No manual `orqa enforce --fix` call (delegated to daemon)
- [ ] No manual git state inspection (delegated to daemon)
- [ ] No manual `.state/` file loading (delegated to daemon)
- [ ] No manual `project.json` dogfood detection (delegated to daemon)
- [ ] Output formatted as Claude Code `systemMessage` from daemon's structured response
- [ ] Fallback behavior if daemon is unreachable (blocks with helpful message, same as current daemon-gate.sh)

**Reviewer Checks:**
- Verify all 6 current concerns are covered by the daemon endpoint
- Verify the script is pure formatting + I/O
- Verify daemon-down fallback works correctly

---

#### TASK P4-09: Move connector knowledge artifacts to methodology plugin

**What:** Move the 10 knowledge files from `connectors/claude-code/knowledge/` to the appropriate plugins. Per ARCHITECTURE.md, connector knowledge that is actually methodology/workflow knowledge belongs in the methodology plugin (`agile-workflow`) or domain knowledge plugins.

**Knowledge files to relocate:**
1. `decision-tree/KNOW.md` (KNOW-3155cdaa) — orchestrator reasoning protocol -> `agile-workflow/knowledge/` (workflow knowledge)
2. `delegation-patterns/KNOW.md` (KNOW-ac314f61) — delegation patterns -> `agile-workflow/knowledge/` (workflow knowledge)
3. `implementer-tree/KNOW.md` (KNOW-b1593311) — implementer reasoning -> `agile-workflow/knowledge/` (role-specific knowledge)
4. `reviewer-tree/KNOW.md` (KNOW-08fcd847) — reviewer reasoning -> `agile-workflow/knowledge/` (role-specific knowledge)
5. `artifact-creation/KNOW.md` (KNOW-816ebef3) — artifact creation protocol -> `core/knowledge/` (framework knowledge)
6. `artifact-ids/KNOW.md` (KNOW-9573eeea) — ID format -> `core/knowledge/` (framework knowledge)
7. `rule-enforcement/KNOW.md` (KNOW-bcfeb64e) — rule enforcement -> `core/knowledge/` (framework knowledge)
8. `tool-mapping/KNOW.md` (KNOW-b0b55e54) — MCP tool guide -> `connectors/claude-code/knowledge/` (KEEP — connector-specific, maps Claude Code tools)
9. `KNOW-4a58e7dd.md` — project migration -> `connectors/claude-code/knowledge/` (KEEP — connector-specific)
10. `KNOW-03421ec0.md` — project inference -> `connectors/claude-code/knowledge/` (KEEP — connector-specific)

**Files modified:**
- Move 4 files from `connectors/claude-code/knowledge/` to `plugins/agile-workflow/knowledge/`
- Move 3 files from `connectors/claude-code/knowledge/` to `plugins/core/knowledge/`
- Update `connectors/claude-code/orqa-plugin.json` — remove relocated entries from `provides.knowledge`
- Update `plugins/agile-workflow/orqa-plugin.json` — add new entries to `provides.knowledge` and `knowledge_declarations`
- Update `plugins/core/orqa-plugin.json` — add new entries to `provides.knowledge` and `knowledge_declarations`

**Acceptance Criteria:**
- [ ] 4 knowledge files exist in `plugins/agile-workflow/knowledge/` with correct IDs
- [ ] 3 knowledge files exist in `plugins/core/knowledge/` with correct IDs
- [ ] 3 knowledge files remain in `connectors/claude-code/knowledge/` (tool-mapping, project-migration, project-inference)
- [ ] Connector `orqa-plugin.json` `provides.knowledge` array has exactly 3 entries
- [ ] agile-workflow `orqa-plugin.json` has new `knowledge_declarations` for the 4 moved files
- [ ] core `orqa-plugin.json` has new `knowledge_declarations` for the 3 moved files
- [ ] No broken file references (all `content_file` paths resolve)

**Reviewer Checks:**
- Verify each knowledge file is in the architecturally correct plugin
- Verify manifest declarations match the moved files
- Verify no orphaned references to old paths

---

#### TASK P4-10: Create generator.ts (connector's primary job)

**What:** Create `connectors/claude-code/src/generator.ts` — the module that transforms engine data into a Claude Code Plugin. This is the connector's primary job per ARCHITECTURE.md 8.1. It reads base role definitions from the methodology plugin, composed workflow from resolved YAML, active rules and enforcement entries, and installed plugin agent declarations. It generates: `.claude/agents/*.md` (from base roles + workflow context), `.claude/CLAUDE.md` (generated orchestrator context), `hooks/hooks.json` (assembled from plugin hook declarations), `.mcp.json` and `.lsp.json` (aggregated server configs).

**Files created:**
- `connectors/claude-code/src/generator.ts`

**Acceptance Criteria:**
- [ ] `generatePlugin(projectRoot: string): GenerateResult` function exported
- [ ] Reads base role definitions from methodology plugin via CLI/daemon API
- [ ] Reads composed workflow state from engine
- [ ] Reads active rules and enforcement entries
- [ ] Reads installed plugin agent declarations
- [ ] Generates agent markdown files from base roles + workflow context
- [ ] Generates CLAUDE.md with project-specific orchestrator context
- [ ] Generates hooks.json from plugin hook declarations
- [ ] Generates .mcp.json from aggregated mcpServers declarations
- [ ] Generates .lsp.json from aggregated lspServers declarations
- [ ] Supports `ORQA_DRY_RUN=true` environment variable — writes to `.state/dry-run/` instead of live paths
- [ ] TypeScript compiles successfully

**Reviewer Checks:**
- Verify generator reads from APIs, not hardcoded paths
- Verify dry-run mode writes to comparison directory
- Verify generated output matches the structure defined in `targets/claude-code-plugin/`

---

#### TASK P4-11: Create watcher.ts (live regeneration)

**What:** Create `connectors/claude-code/src/watcher.ts` — the module that watches for changes to plugins, rules, and workflow compositions and triggers regeneration of the Claude Code Plugin via `generator.ts`.

**Files created:**
- `connectors/claude-code/src/watcher.ts`

**Acceptance Criteria:**
- [ ] `watchAndRegenerate(projectRoot: string): void` function exported
- [ ] Watches `.orqa/workflows/*.resolved.yaml` for workflow changes
- [ ] Watches `.orqa/learning/rules/*.md` for rule changes
- [ ] Watches `plugins/*/orqa-plugin.json` for plugin manifest changes
- [ ] Watches `.orqa/schema.composed.json` for schema changes
- [ ] Triggers `generatePlugin()` on detected changes
- [ ] Debounces rapid changes (does not regenerate on every keystroke)
- [ ] Logs regeneration events
- [ ] Graceful cleanup on process exit

**Reviewer Checks:**
- Verify watch paths cover all inputs to the generation pipeline
- Verify debouncing prevents excessive regeneration
- Verify file watcher handles missing directories gracefully

---

#### TASK P4-12: Generate hooks.json from plugin declarations

**What:** Extend `generator.ts` (or create a dedicated function) to generate `hooks/hooks.json` from plugin `provides.hooks` declarations. Currently hooks.json is a static file; in the target architecture it should be assembled from what plugins declare.

**Files modified:**
- `connectors/claude-code/src/generator.ts` — add hooks.json generation
- `connectors/claude-code/hooks/hooks.json` — becomes generated output (delete static version once generator is validated)

**Acceptance Criteria:**
- [ ] Generator produces a `hooks.json` that includes all hook declarations from all installed plugins
- [ ] Hook wiring maps plugin-declared events to the correct script paths
- [ ] Generated hooks.json matches the current static hooks.json structure
- [ ] Timeouts are preserved from plugin declarations
- [ ] Matcher patterns are preserved from plugin declarations

**Reviewer Checks:**
- Verify generated output matches or exceeds current static hooks.json
- Verify all 14 current hook registrations are present
- Verify the generator handles plugins that don't declare hooks

---

#### TASK P4-13: Validate generated output against target Claude Code Plugin

**What:** Create a validation script or test that compares the output of `generator.ts` against `targets/claude-code-plugin/`. This is the Phase 10 validation for the connector specifically, but should be built during Phase 4 to enable continuous verification.

**Files created:**
- `scripts/validate-connector-output.mjs` (or equivalent)

**Acceptance Criteria:**
- [ ] Script runs `generatePlugin()` in dry-run mode
- [ ] Compares generated `.claude/agents/` against `targets/claude-code-plugin/.claude/agents/`
- [ ] Compares generated `CLAUDE.md` against target
- [ ] Compares generated `hooks.json` against target
- [ ] Compares generated `.mcp.json` against target
- [ ] Compares generated `.lsp.json` against target
- [ ] Reports diff for any mismatches
- [ ] Exits with non-zero code if any mismatch found

**Reviewer Checks:**
- Verify comparison is structural (not byte-exact for whitespace-insensitive content)
- Verify all generated artifacts are compared

---

#### TASK P4-14: Update connector package.json and index.ts exports

**What:** Clean up `connectors/claude-code/package.json` and `connectors/claude-code/src/index.ts` to reflect the refactored connector. Remove references to deleted modules (artifact-bridge). Add exports for new modules (generator, watcher). Update dependencies if any are no longer needed after removing MCP IPC and business logic.

**Files modified:**
- `connectors/claude-code/src/index.ts` — update exports
- `connectors/claude-code/package.json` — update if dependencies changed

**Acceptance Criteria:**
- [ ] `ArtifactBridge` and `BridgeMapping` not exported
- [ ] `generatePlugin` and `watchAndRegenerate` exported
- [ ] `runConnectorSetup` still exported
- [ ] No unused dependency imports
- [ ] TypeScript compiles successfully
- [ ] `npm run build` succeeds

**Reviewer Checks:**
- Verify all exports correspond to modules that exist
- Verify no unused dependencies in package.json

---

#### TASK P4-15: Build connector and verify TypeScript compilation

**What:** After all Phase 4 changes, run `npm run build` in the connector directory to verify the entire connector compiles cleanly.

**Acceptance Criteria:**
- [ ] `npx tsc --noEmit` passes with zero errors
- [ ] `npm run build` produces `dist/` with all hook scripts
- [ ] All 10 hook scripts in `dist/hooks/` are present and valid JS
- [ ] No TypeScript errors or warnings

**Reviewer Checks:**
- Run `npx tsc --noEmit` independently
- Verify dist/ output matches expected file list

**Dependencies:** All P4-01 through P4-14 must be complete

---

## Phase 5: Plugin Manifest Standardization

Phase 5 updates all 17 plugin manifests (16 plugins + 1 connector) to support the architecture's taxonomy, installation constraints, and naming conventions. Every GAP-* from phase2-02-plugin-gaps.md has a corresponding task.

### Manifest Taxonomy Tasks

#### TASK P5-01: Define canonical manifest schema with new fields (addresses GAP-TAX-1, GAP-MAN-1, GAP-MAN-2, GAP-MAN-3)

**What:** Create a JSON schema document `targets/plugin-manifests/orqa-plugin.schema.json` that defines the canonical `orqa-plugin.json` format with the new required fields: `purpose` (array of enum values: methodology, workflow, domain-knowledge, connector, infrastructure, app-extension, sidecar), `stage_slot` (string, required for workflow plugins only), `affects_schema` (boolean — does this plugin contribute to schema composition?), `affects_enforcement` (boolean — does this plugin contribute to enforcement?). Also standardize the `category` vocabulary to the 5 values from ARCHITECTURE.md 4.3: methodology, workflow, domain-knowledge, connector, infrastructure.

**Files created:**
- `targets/plugin-manifests/orqa-plugin.schema.json`

**Acceptance Criteria:**
- [ ] Schema defines `purpose` as required array of enum values (7 possible values)
- [ ] Schema defines `stage_slot` as conditionally required (required when purpose is "workflow")
- [ ] Schema defines `affects_schema` as boolean
- [ ] Schema defines `affects_enforcement` as boolean
- [ ] Schema defines `category` as enum with 5 values matching ARCHITECTURE.md
- [ ] Schema validates all existing manifest fields (name, version, displayName, description, provides, content, etc.)
- [ ] Schema includes `provides.workflows` as structured objects only (not flat strings)

**Reviewer Checks:**
- Verify schema covers ALL fields found in current manifests
- Verify the 7 purpose values align with ARCHITECTURE.md 4.3
- Verify the 5 category values match the taxonomy exactly

---

#### TASK P5-02: Update agile-workflow manifest — fix category + add purpose/stage_slot (addresses GAP-TAX-2, GAP-NAME-1)

**What:** Update `plugins/agile-workflow/orqa-plugin.json`:
- Change `category` from `"governance"` to `"methodology"`
- Add `"purpose": "methodology"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`
- Do NOT add `stage_slot` (methodology plugins don't fill a stage slot — they DEFINE the stages)
- Note: the plugin RENAME (agile-workflow -> agile-methodology) is a separate task (P5-29)

**Files modified:**
- `plugins/agile-workflow/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"methodology"`
- [ ] `purpose` is `["methodology"]`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`
- [ ] No `stage_slot` field present
- [ ] `role` field updated from `"core:workflow"` to reflect methodology status (or removed if role is being retired)
- [ ] All other manifest content unchanged

**Reviewer Checks:**
- Verify this is the ONLY plugin with `"methodology"` in its `purpose` array
- Verify `category` matches ARCHITECTURE.md taxonomy

---

#### TASK P5-03: Update core manifest — fix category + add purpose/stage_slot (addresses GAP-TAX-3)

**What:** Update `plugins/core/orqa-plugin.json`:
- Change `category` from `"framework"` to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "learning"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": true`

**Files modified:**
- `plugins/core/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"`
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"learning"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `true`
- [ ] `uninstallable: true` flag preserved (core's special status)
- [ ] All other manifest content unchanged

**Reviewer Checks:**
- Verify `stage_slot` matches the contribution point name in the methodology workflow
- Verify `uninstallable` flag is still present

---

#### TASK P5-04: Update agile-discovery manifest — fix category + add purpose/stage_slot (addresses GAP-TAX-4)

**What:** Update `plugins/agile-discovery/orqa-plugin.json`:
- Change `category` from `"discovery"` to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "discovery"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/agile-discovery/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"`
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"discovery"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify `stage_slot` matches the contribution point in agile-methodology.workflow.yaml

---

#### TASK P5-05: Update agile-planning manifest — fix category + add purpose/stage_slot (addresses GAP-TAX-2, GAP-TAX-4)

**What:** Update `plugins/agile-planning/orqa-plugin.json`:
- Change `category` from `"methodology"` (WRONG — this is a workflow plugin, not the methodology plugin) to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "planning"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/agile-planning/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"` (NOT `"methodology"` — that belongs to agile-workflow)
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"planning"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify this plugin no longer claims to be the methodology plugin
- Verify `stage_slot` matches the contribution point in agile-methodology.workflow.yaml

---

#### TASK P5-06: Update agile-documentation manifest — fix category + add purpose/stage_slot + fix workflow format (addresses GAP-TAX-4, GAP-MAN-5, GAP-WF-2)

**What:** Update `plugins/agile-documentation/orqa-plugin.json`:
- Change `category` from `"documentation"` to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "documentation"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`
- Fix `provides.workflows` from flat string array `["workflows/documentation.contribution.workflow.yaml"]` to structured object format `[{ "artifact_type": null, "path": "workflows/documentation.contribution.workflow.yaml", "contribution": true }]`

**Files modified:**
- `plugins/agile-documentation/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"`
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"documentation"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`
- [ ] `provides.workflows` is an array of structured objects, NOT flat strings
- [ ] The workflow object has `path`, `contribution: true` fields

**Reviewer Checks:**
- Verify the workflow declaration format matches other plugins (agile-discovery, agile-planning, agile-review)
- Verify `stage_slot` matches the contribution point in agile-methodology.workflow.yaml

---

#### TASK P5-07: Update agile-review manifest — fix category + add purpose/stage_slot (addresses GAP-TAX-4)

**What:** Update `plugins/agile-review/orqa-plugin.json`:
- Change `category` from `"stage-definition"` to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "review"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/agile-review/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"`
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"review"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify `stage_slot` matches the contribution point in agile-methodology.workflow.yaml

---

#### TASK P5-08: Update software-kanban manifest — fix category + add purpose/stage_slot + fix missing contribution workflow (addresses GAP-TAX-4, GAP-MAN-4, GAP-WF-1, GAP-DUAL-1)

**What:** Update `plugins/software-kanban/orqa-plugin.json`:
- Change `category` from `"delivery"` to `"workflow"`
- Add `"purpose": ["workflow"]`
- Add `"stage_slot": "implementation"`
- Add `"affects_schema": true`
- Add `"affects_enforcement": false`
- Add `workflows/implementation.contribution.workflow.yaml` to `provides.workflows` array as a structured object with `contribution: true`

**Files modified:**
- `plugins/software-kanban/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"workflow"`
- [ ] `purpose` is `["workflow"]`
- [ ] `stage_slot` is `"implementation"`
- [ ] `affects_schema` is `true`
- [ ] `affects_enforcement` is `false`
- [ ] `provides.workflows` array contains 4 entries: milestone, epic, task (existing) + implementation contribution (new)
- [ ] Implementation contribution entry has `contribution: true` and correct path
- [ ] The file `workflows/implementation.contribution.workflow.yaml` exists on disk

**Reviewer Checks:**
- Verify the contribution workflow file exists at the declared path
- Verify the workflow YAML has `contributes_to.point: implementation-workflow`
- Verify all 4 workflow entries use structured object format

---

#### TASK P5-09: Update cli manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/cli/orqa-plugin.json`:
- Change `category` from `"tooling"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/cli/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`
- [ ] No `stage_slot` field present (not a workflow plugin)

**Reviewer Checks:**
- Verify this plugin provides knowledge but not schemas or workflows

---

#### TASK P5-10: Update rust manifest — fix category + add purpose + declare dual-purpose (addresses GAP-TAX-1, GAP-DUAL-1)

**What:** Update `plugins/rust/orqa-plugin.json`:
- Change `category` from `"coding-standards"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge", "infrastructure"]` (provides knowledge AND generates linting infrastructure)
- Add `"affects_schema": false`
- Add `"affects_enforcement": true` (per ARCHITECTURE.md table — rust generates enforcement config)

**Files modified:**
- `plugins/rust/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge", "infrastructure"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `true`

**Reviewer Checks:**
- Verify multi-purpose array captures the plugin's infrastructure role (linting)

---

#### TASK P5-11: Update svelte manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/svelte/orqa-plugin.json`:
- Change `category` from `"tooling"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/svelte/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify no infrastructure dual-purpose needed for svelte

---

#### TASK P5-12: Update tauri manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/tauri/orqa-plugin.json`:
- Change `category` from `"tooling"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/tauri/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify tauri provides knowledge but not schema

---

#### TASK P5-13: Update typescript manifest — fix category + add purpose + declare dual-purpose (addresses GAP-TAX-1, GAP-DUAL-1)

**What:** Update `plugins/typescript/orqa-plugin.json`:
- Change `category` from `"coding-standards"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge", "infrastructure"]` (provides knowledge AND generates linting infrastructure)
- Add `"affects_schema": false`
- Add `"affects_enforcement": true` (per ARCHITECTURE.md table — typescript generates enforcement config)

**Files modified:**
- `plugins/typescript/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge", "infrastructure"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `true`

**Reviewer Checks:**
- Verify multi-purpose array captures the plugin's infrastructure role

---

#### TASK P5-14: Update coding-standards manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/coding-standards/orqa-plugin.json`:
- Change `category` from `"coding-standards"` to `"infrastructure"`
- Add `"purpose": ["infrastructure"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": true` (per ARCHITECTURE.md table — coding-standards affects enforcement)

**Files modified:**
- `plugins/coding-standards/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"infrastructure"`
- [ ] `purpose` is `["infrastructure"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `true`

**Reviewer Checks:**
- Verify coding-standards is infrastructure with enforcement role, per ARCHITECTURE.md table line 1307

---

#### TASK P5-15: Update systems-thinking manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/systems-thinking/orqa-plugin.json`:
- Change `category` from `"thinking"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/systems-thinking/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify systems-thinking is domain knowledge

---

#### TASK P5-16: Update plugin-dev manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/plugin-dev/orqa-plugin.json`:
- Change `category` from `"development"` to `"domain-knowledge"`
- Add `"purpose": ["domain-knowledge"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `plugins/plugin-dev/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"domain-knowledge"`
- [ ] `purpose` is `["domain-knowledge"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify plugin-dev provides knowledge for plugin development

---

#### TASK P5-17: Update githooks manifest — fix category + add purpose (addresses GAP-TAX-1)

**What:** Update `plugins/githooks/orqa-plugin.json`:
- Change `category` from `"enforcement"` to `"infrastructure"`
- Add `"purpose": ["infrastructure"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": true` (per ARCHITECTURE.md table — githooks generates enforcement infrastructure)

**Files modified:**
- `plugins/githooks/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"infrastructure"`
- [ ] `purpose` is `["infrastructure"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `true`

**Reviewer Checks:**
- Verify githooks generates enforcement infrastructure (git hooks)
- Verify `affects_enforcement` is `true` per ARCHITECTURE.md table line 1309

---

#### TASK P5-18: Update connector manifest — add purpose (addresses GAP-TAX-1)

**What:** Update `connectors/claude-code/orqa-plugin.json`:
- Verify `category` is already `"connector"` (it should be)
- Add `"purpose": ["connector"]`
- Add `"affects_schema": false`
- Add `"affects_enforcement": false`

**Files modified:**
- `connectors/claude-code/orqa-plugin.json`

**Acceptance Criteria:**
- [ ] `category` is `"connector"`
- [ ] `purpose` is `["connector"]`
- [ ] `affects_schema` is `false`
- [ ] `affects_enforcement` is `false`

**Reviewer Checks:**
- Verify the connector manifest is correctly categorized

---

### Duplicate Resolution Tasks

#### TASK P5-19: Resolve AGENT-ae63c406 duplication across plugins (addresses GAP-DUP-1)

**What:** Delete the stale `plugins/agile-workflow/agents/AGENT-ae63c406.md` (created 2026-03-14, simpler version). Keep `plugins/core/agents/AGENT-ae63c406.md` (updated 2026-03-24, richer version). Update `agile-workflow/orqa-plugin.json` to remove any reference to this agent file. Note: both copies will be deleted in Phase 7 (AGENT-*.md removal), but for Phase 5 the immediate fix is deduplication.

**Files modified:**
- `plugins/agile-workflow/agents/AGENT-ae63c406.md` — DELETE
- `plugins/agile-workflow/orqa-plugin.json` — remove agent reference if present

**Acceptance Criteria:**
- [ ] `plugins/agile-workflow/agents/AGENT-ae63c406.md` does not exist
- [ ] `plugins/core/agents/AGENT-ae63c406.md` still exists (unchanged)
- [ ] agile-workflow manifest does not reference AGENT-ae63c406

**Reviewer Checks:**
- Verify only the stale copy was deleted
- Verify no other files reference the deleted path

---

#### TASK P5-20: Deduplicate knowledge pairs in agile-workflow (addresses GAP-DUP-2)

**What:** Merge three pairs of duplicate-topic knowledge artifacts in `plugins/agile-workflow/`:

**Pair 1:** "Thinking Mode: Learning Loop"
- KNOW-83039175.md (installed path `../../.orqa/process/knowledge/`) and KNOW-85e392ea.md (plugin-local)
- Keep the plugin-local version (KNOW-85e392ea.md) as source of truth. If KNOW-83039175.md has richer content, merge that content into KNOW-85e392ea.md first. Delete or archive KNOW-83039175.md reference.

**Pair 2:** "Plugin Artifact Usage"
- KNOW-0444355f.md (installed path) and KNOW-8d1c4be6.md (plugin-local)
- Keep plugin-local (KNOW-8d1c4be6.md). Merge content if needed. Remove KNOW-0444355f reference.

**Pair 3:** "Governance Maintenance"
- KNOW-8c359ea4.md (installed path) and KNOW-8d76c3c7.md (plugin-local)
- Keep plugin-local (KNOW-8d76c3c7.md). Merge content if needed. Remove KNOW-8c359ea4 reference.

**Files modified:**
- `plugins/agile-workflow/knowledge/KNOW-85e392ea.md` — merge content from KNOW-83039175 if richer
- `plugins/agile-workflow/knowledge/KNOW-8d1c4be6.md` — merge content from KNOW-0444355f if richer
- `plugins/agile-workflow/knowledge/KNOW-8d76c3c7.md` — merge content from KNOW-8c359ea4 if richer
- `plugins/agile-workflow/orqa-plugin.json` — update `provides.knowledge` and `knowledge_declarations` to remove duplicate entries

**Acceptance Criteria:**
- [ ] Only 1 knowledge file per topic (not 2)
- [ ] Plugin-local files (KNOW-85e392ea, KNOW-8d1c4be6, KNOW-8d76c3c7) are the canonical versions
- [ ] Installed-path references (KNOW-83039175, KNOW-0444355f, KNOW-8c359ea4) removed from `knowledge_declarations`
- [ ] `provides.knowledge` array updated — cross-reference keys cleaned up
- [ ] No content lost — richer content from either source preserved

**Reviewer Checks:**
- Verify content from both versions was compared before dedup
- Verify no knowledge_declaration still references `../../.orqa/process/knowledge/` paths for these 3 artifacts
- Verify the `provides.knowledge` entries are consistent (no stale key references)

---

#### TASK P5-21: Fix duplicate knowledge_declaration IDs in agile-workflow (addresses GAP-DUP-3)

**What:** Fix the 4 `knowledge_declarations` entries in `plugins/agile-workflow/orqa-plugin.json` that all use `id: "thinking-mode-governance"`. Each must have a unique ID:

1. `id: "thinking-mode-governance"`, content_file: KNOW-c89f28b3 — KEEP this ID (it is the actual thinking-mode-governance)
2. `id: "thinking-mode-governance"`, content_file: KNOW-57365826 — RENAME to `"schema-first-frontmatter"`
3. `id: "thinking-mode-governance"`, content_file: KNOW-e3432947 — RENAME to `"plugin-canonical-architecture"`
4. `id: "thinking-mode-governance"`, content_file: KNOW-6d80cf39 — RENAME to `"documentation-placement-guide"`

**Files modified:**
- `plugins/agile-workflow/orqa-plugin.json` — fix 3 duplicate IDs

**Acceptance Criteria:**
- [ ] All `knowledge_declarations` entries have unique IDs
- [ ] `thinking-mode-governance` ID appears exactly ONCE (for KNOW-c89f28b3)
- [ ] KNOW-57365826 has ID `"schema-first-frontmatter"` (or similar unique, descriptive ID)
- [ ] KNOW-e3432947 has ID `"plugin-canonical-architecture"` (or similar unique, descriptive ID)
- [ ] KNOW-6d80cf39 has ID `"documentation-placement-guide"` (or similar unique, descriptive ID)
- [ ] No two entries share the same ID anywhere in the manifest

**Reviewer Checks:**
- Verify all IDs are unique by extracting all `id` values from `knowledge_declarations`
- Verify the renamed IDs are descriptive and match the knowledge content

---

#### TASK P5-22: Deduplicate knowledge_declarations in software-kanban (addresses GAP-DUP-4)

**What:** Fix duplicate knowledge_declarations in `plugins/software-kanban/orqa-plugin.json` where the same content exists with both plugin-local and installed-path references:

1. `code-quality-review` (content_file: `knowledge/KNOW-f0efaf83.md`) + `code-quality-review-91a7a6c1` (content_file: `../../.orqa/process/knowledge/KNOW-91a7a6c1.md`) — keep plugin-local, remove installed-path reference
2. `security-audit` (content_file: `knowledge/KNOW-170c220e.md`) + `security-audit-45b5f8a8` (content_file: `../../.orqa/process/knowledge/KNOW-45b5f8a8.md`) — keep plugin-local, remove installed-path reference
3. `test-engineering` (content_file: `knowledge/KNOW-bcb42347.md`) + `test-engineering-5f4db8f7` (content_file: `../../.orqa/process/knowledge/KNOW-5f4db8f7.md`) — keep plugin-local, remove installed-path reference
4. `delivery-completion` (no content_file, summary only) + `delivery-unit-completion-discipline` (content_file: `../../.orqa/process/knowledge/KNOW-0188373b.md`) — merge into single entry with plugin-local content_file. If KNOW-0188373b.md does not exist in plugin source, copy it from installed path.

**Files modified:**
- `plugins/software-kanban/orqa-plugin.json` — remove 4 duplicate `knowledge_declarations` entries
- Possibly create `plugins/software-kanban/knowledge/KNOW-0188373b.md` if it only exists at installed path

**Acceptance Criteria:**
- [ ] No knowledge_declaration references `../../.orqa/process/knowledge/` paths
- [ ] Each knowledge topic appears exactly once in `knowledge_declarations`
- [ ] Plugin-local content_file paths used for all declarations
- [ ] All referenced content_files exist on disk in the plugin source directory
- [ ] No content lost — summaries and metadata from both entries preserved in the survivor

**Reviewer Checks:**
- Grep for `../../.orqa/` in software-kanban manifest — zero hits
- Verify each kept entry has all metadata (tier, roles, stages, tags, priority, summary)
- Count knowledge_declarations entries before vs after — confirm correct reduction

---

### Missing File Resolution Tasks

#### TASK P5-23: Create or relocate KNOW-3f307edb in software-kanban (addresses GAP-MISS-1, GAP-MAN-6)

**What:** The manifest declares KNOW-3f307edb ("Orqa Testing Patterns") but the file only exists at `.orqa/process/knowledge/KNOW-3f307edb.md`, not in the plugin's own `knowledge/` directory. Copy the file from the installed location into the plugin source directory. Update the `knowledge_declarations` entry to use the plugin-local path.

**Files modified:**
- `plugins/software-kanban/knowledge/KNOW-3f307edb.md` — CREATE (copy from `.orqa/process/knowledge/KNOW-3f307edb.md`)
- `plugins/software-kanban/orqa-plugin.json` — update `knowledge_declarations` content_file path for `orqa-testing-patterns` from `../../.orqa/process/knowledge/KNOW-3f307edb.md` to `knowledge/KNOW-3f307edb.md`

**Acceptance Criteria:**
- [ ] `plugins/software-kanban/knowledge/KNOW-3f307edb.md` exists
- [ ] Content matches the installed copy at `.orqa/process/knowledge/KNOW-3f307edb.md`
- [ ] `knowledge_declarations` entry uses plugin-local path `knowledge/KNOW-3f307edb.md`
- [ ] `provides.knowledge` entry for KNOW-3f307edb is present and correct

**Reviewer Checks:**
- Diff the new plugin-local copy against the installed copy — should be identical
- Verify no other manifest still references the `../../.orqa/` path for this file

---

#### TASK P5-24: Create or remove prompts/review-checklist.md reference in software-kanban (addresses GAP-MISS-2, GAP-MAN-7)

**What:** The manifest declares a `prompt_sections` entry referencing `prompts/review-checklist.md` but the file does not exist anywhere. Either (a) create the file with appropriate review checklist content, or (b) remove the declaration from the manifest. Decision: if review checklist content can be derived from existing review knowledge (KNOW-91a7a6c1, KNOW-f0efaf83), create a minimal file. Otherwise, remove the reference.

**Files modified:**
- `plugins/software-kanban/prompts/review-checklist.md` — CREATE if content can be derived, OR
- `plugins/software-kanban/orqa-plugin.json` — remove the `prompt_sections` entry if file cannot be meaningfully created

**Acceptance Criteria:**
- [ ] Either `prompts/review-checklist.md` exists with meaningful review stage instructions, OR the `prompt_sections` entry for "review-checklist" is removed from the manifest
- [ ] No broken file references in the manifest — every declared `content_file` resolves to an existing file

**Reviewer Checks:**
- If file created: verify content is meaningful and non-trivial
- If reference removed: verify the `prompt_sections` array no longer references it
- Verify `prompts/implementer-role.md` still exists and is referenced correctly

---

### Schema Standardization Tasks

#### TASK P5-25: Standardize schema field naming — title not name (addresses GAP-MAN-8)

**What:** Standardize all artifact type schemas across all plugins to use `title` as the display-name field instead of `name`. Currently 10 schemas use `title` and 6 use `name` (milestone, epic, task, wireframe, discovery-research, planning-research).

**Files modified (schemas using `name` that need changing to `title`):**
- `plugins/software-kanban/orqa-plugin.json` — schemas for milestone, epic, task: rename `name` property to `title` in frontmatter definitions
- Any other plugin schemas using `name` instead of `title` (discovery-research, planning-research, wireframe)

**Also fix actual artifact frontmatter:**
- `plugins/software-kanban/knowledge/KNOW-a700e25a.md` — change `name:` to `title:` in frontmatter
- `plugins/software-kanban/documentation/DOC-4554ff3e.md` (if it exists) — change `name:` to `title:` in frontmatter

**Acceptance Criteria:**
- [ ] ALL schema definitions across ALL plugins use `title` as the display-name field (not `name`)
- [ ] No schema has a `name` property in its `frontmatter.properties` (unless `name` serves a different purpose than display name)
- [ ] All artifacts that had `name:` in frontmatter now have `title:` instead
- [ ] Schema `required` arrays updated if they referenced `name` to reference `title`
- [ ] Grep for `"name":` in schema frontmatter.properties returns zero hits (excluding the manifest `name` field at root level)

**Reviewer Checks:**
- Grep all `orqa-plugin.json` files for `"name"` in frontmatter properties — should only appear if serving a non-display-name purpose
- Verify no `name:` frontmatter in actual artifact .md files within plugins
- Verify `title` is in the `required` array where `name` was previously required

---

### Installation Constraint Tasks

#### TASK P5-26: Implement one-methodology enforcement in orqa install (addresses GAP-INST-1)

**What:** Add enforcement to the `orqa install` command (in the CLI or engine plugin crate) that prevents installing more than one plugin with `purpose: "methodology"`. When a user tries to install a second methodology plugin, the installer should error with a clear message explaining that only one methodology plugin is allowed per project.

**Files modified:**
- CLI or engine plugin installation code (exact file depends on current implementation)

**Acceptance Criteria:**
- [ ] Installing a second `purpose: "methodology"` plugin fails with a descriptive error
- [ ] The error names the currently installed methodology plugin
- [ ] Installing the first methodology plugin succeeds normally
- [ ] Reinstalling the same methodology plugin succeeds (update, not conflict)
- [ ] Non-methodology plugins are unaffected

**Reviewer Checks:**
- Verify the check reads `purpose` from manifests (not heuristics)
- Test with a mock second methodology plugin to confirm the error fires

---

#### TASK P5-27: Implement one-per-stage enforcement in orqa install (addresses GAP-INST-2)

**What:** Add enforcement to the `orqa install` command that prevents installing two workflow plugins that fill the same `stage_slot`. When a user tries to install a workflow plugin whose `stage_slot` is already filled by another installed plugin, the installer should error with a clear message.

**Files modified:**
- CLI or engine plugin installation code

**Acceptance Criteria:**
- [ ] Installing a workflow plugin with `stage_slot: "discovery"` when another plugin already fills that slot fails with error
- [ ] The error names the conflicting plugin and the stage slot
- [ ] Different stage slots can be filled without conflict
- [ ] Non-workflow plugins (no `stage_slot`) are unaffected

**Reviewer Checks:**
- Verify the check reads `stage_slot` from manifests
- Verify the check only applies to workflow plugins

---

#### TASK P5-28: Implement definition vs non-definition plugin distinction in orqa install (addresses GAP-INST-3)

**What:** Add logic to `orqa install` that distinguishes definition plugins (`affects_schema: true`) from non-definition plugins (`affects_schema: false`). Definition plugins trigger full schema recomposition after installation. Non-definition plugins only install their assets (knowledge, views, widgets) without recomposing the schema. Separately, plugins with `affects_enforcement: true` trigger enforcement config regeneration.

**Files modified:**
- CLI or engine plugin installation code

**Acceptance Criteria:**
- [ ] Plugins with `affects_schema: true` trigger schema recomposition after install
- [ ] Plugins with `affects_schema: false` do NOT trigger recomposition
- [ ] Plugins with `affects_enforcement: true` trigger enforcement config regeneration after install
- [ ] Both plugin types install their declared assets correctly
- [ ] If `affects_schema` or `affects_enforcement` is not present, default to `false` (safe default)

**Reviewer Checks:**
- Verify recomposition is actually triggered (not just flagged)
- Verify enforcement regeneration is triggered for `affects_enforcement: true` plugins
- Verify the default behavior is safe (no recomposition on missing field)

---

### Plugin Rename Tasks

#### TASK P5-29: Rename agile-workflow to agile-methodology (addresses GAP-NAME-1)

**What:** Rename the methodology plugin from `agile-workflow` to `agile-methodology` to make its role self-evident. This is a multi-file change:

**Directory:**
- Rename `plugins/agile-workflow/` to `plugins/agile-methodology/`

**Manifest changes:**
- `plugins/agile-methodology/orqa-plugin.json` — update `name` from `@orqastudio/plugin-agile-workflow` to `@orqastudio/plugin-agile-methodology`
- `plugins/agile-methodology/package.json` (if exists) — update package name

**References to update:**
- `connectors/claude-code/orqa-plugin.json` — update `requires` entry from `@orqastudio/plugin-agile-workflow` to `@orqastudio/plugin-agile-methodology`
- Any other `orqa-plugin.json` files that reference `@orqastudio/plugin-agile-workflow` in `requires` or `dependencies`
- Any import statements or path references in source code
- `ARCHITECTURE.md` — update references to `agile-workflow` (document references the plugin name)
- `CLAUDE.md` — if it references agile-workflow
- `.orqa/` installed content paths if they reference the plugin name

**Acceptance Criteria:**
- [ ] Directory `plugins/agile-methodology/` exists
- [ ] Directory `plugins/agile-workflow/` does NOT exist
- [ ] Manifest name is `@orqastudio/plugin-agile-methodology`
- [ ] All `requires` references across all manifests updated
- [ ] Grep for `agile-workflow` across all `.json` and `.ts` files returns zero hits (except git history)
- [ ] Grep for `agile-workflow` in ARCHITECTURE.md and CLAUDE.md returns zero hits
- [ ] All knowledge_declarations content_file paths still resolve correctly after rename

**Reviewer Checks:**
- Grep entire repository for `agile-workflow` (case-insensitive) — only git history and potentially this task list should reference it
- Verify no broken imports or requires
- Verify `orqa install` still works after rename

---

#### TASK P5-30: Evaluate core plugin naming (addresses GAP-NAME-2)

**What:** Evaluate whether the `core` plugin should be renamed to better communicate its dual role as the learning-stage workflow plugin and framework provider. Options: `core-framework`, `learning-loop`, keep as `core`. This is a DESIGN DECISION task — write findings with recommendation, do not rename without approval.

**Note:** This is intentionally a planning/research task, not an implementation task. The `core` plugin has `uninstallable: true` and special status; renaming requires careful consideration.

**Files created:**
- Findings document with recommendation

**Acceptance Criteria:**
- [ ] Analysis of naming options with pros/cons
- [ ] Recommendation for whether to rename and to what
- [ ] Impact assessment (what files/references would change)
- [ ] Explicit call-out that `uninstallable: true` means this is a framework component

**Reviewer Checks:**
- Verify all naming options were considered
- Verify impact assessment is complete

---

### Role Definition Tasks

#### TASK P5-31: Resolve extra roles (Planner, Designer, Governance Steward) vs architecture (addresses GAP-ROLE-1)

**What:** ARCHITECTURE.md section 6.1 defines 5 base roles (Orchestrator, Implementer, Reviewer, Researcher, Writer). The codebase implements 8 roles (adding Planner, Designer, Governance Steward). This is a DESIGN DECISION task — determine whether the 3 extra roles are intentional extensions or should be removed. If intentional, update ARCHITECTURE.md section 6.1 to document all 8 roles. If not intentional, flag role YAML files for deletion.

**Note:** CLAUDE.md already references Planner and Governance Steward in the role table, suggesting they are intentional.

**Files modified (if keeping roles):**
- `ARCHITECTURE.md` — update section 6.1 to list all 8 base roles

**Files modified (if removing roles):**
- `plugins/agile-methodology/roles/planner.yaml` — DELETE
- `plugins/agile-methodology/roles/designer.yaml` — DELETE
- `plugins/agile-methodology/roles/governance_steward.yaml` — DELETE

**Acceptance Criteria:**
- [ ] Decision documented with rationale
- [ ] If roles kept: ARCHITECTURE.md section 6.1 lists all 8 roles with descriptions
- [ ] If roles removed: role YAML files deleted, no references remain
- [ ] No discrepancy between ARCHITECTURE.md and actual role definitions

**Reviewer Checks:**
- Verify CLAUDE.md role table aligns with the decision
- Verify no orphaned references to removed roles (if removed)

---

### Workflow Completeness Tasks

#### TASK P5-32: Implement resolved workflow generation (addresses GAP-WF-3)

**What:** Implement the workflow composition pipeline that produces resolved workflow files at `.orqa/workflows/<name>.resolved.yaml`. Per ARCHITECTURE.md 4.6 and 5.1, the methodology workflow + contribution workflows should be composed into one resolved file per stage. This requires the engine's workflow crate to be operational (Phase 2 dependency).

**Files created:**
- `.orqa/workflows/agile-methodology.resolved.yaml` (or per-stage files)

**Files modified:**
- CLI `orqa install` or `orqa compose` command — add workflow resolution step

**Acceptance Criteria:**
- [ ] Resolved workflow files exist at `.orqa/workflows/`
- [ ] Each resolved file contains the methodology skeleton + the stage's contribution workflow composed into it
- [ ] Resolved files are regenerated when source workflows change
- [ ] Resolved files validate against the workflow schema

**Reviewer Checks:**
- Verify the composition correctly merges contribution points
- Verify each contribution point has exactly one filler
- Verify resolved files match `targets/workflows/` (if target files exist)

---

### Dual-Purpose Declaration Tasks

#### TASK P5-33: Update core manifest purpose array to reflect multi-purpose nature (addresses GAP-DUAL-2)

**What:** The core plugin serves multiple purposes (framework artifact schemas, learning-stage workflow, enforcement mechanisms, knowledge). Update its `purpose` array to explicitly declare all purposes. P5-03 sets `purpose: ["workflow"]` as the primary; this task expands it to capture the full scope.

**Files modified:**
- `plugins/core/orqa-plugin.json` — update `purpose` array to include all applicable values

**Acceptance Criteria:**
- [ ] Core manifest `purpose` is an array capturing its multi-role nature (e.g., `["workflow", "infrastructure"]`)
- [ ] Both framework provision and learning-stage workflow roles are reflected in the array
- [ ] No `dual_purpose` field exists — multi-purpose is expressed via the `purpose` array itself

**Reviewer Checks:**
- Verify `purpose` is an array consistent with the schema from P5-01
- Verify `affects_schema` and `affects_enforcement` are both `true` (set in P5-03)

---

### Content Installation Target Tasks

#### TASK P5-34: Declare content installation targets in plugin manifests (addresses ARCHITECTURE.md Phase 5 item 4)

**What:** Each plugin manifest should declare where its content installs in the `.orqa/` hierarchy. ARCHITECTURE.md Phase 5 item 4 says manifests should declare content installation targets. Review each plugin's `content.mappings` and ensure every content type (knowledge, documentation, workflows, schemas, rules) has an explicit installation path in the `.orqa/` hierarchy. Standardize the mapping format across all plugins.

**Files modified:**
- All 17 `orqa-plugin.json` files — verify/add `content.mappings` with explicit `.orqa/` target paths

**Acceptance Criteria:**
- [ ] Every plugin manifest declares where each content type installs in the `.orqa/` hierarchy
- [ ] Knowledge content maps to `.orqa/documentation/<category>/knowledge/`
- [ ] Documentation content maps to `.orqa/documentation/<category>/`
- [ ] Resolved workflow content maps to `.orqa/workflows/`
- [ ] Schema content maps to `.orqa/schemas/`
- [ ] Rule content maps to `.orqa/learning/rules/`
- [ ] Decision content maps to `.orqa/learning/decisions/` or `.orqa/planning/decisions/`
- [ ] Lesson content maps to `.orqa/learning/lessons/`
- [ ] All mapping paths use the post-Phase-7 `.orqa/` structure (not the legacy `.orqa/process/` paths)

**Reviewer Checks:**
- Verify every plugin with content has explicit mappings
- Verify mapping paths match the Phase 7 target directory structure

**Dependencies:** P5-01 through P5-33 (manifest fields must be standardized first)

---

### Design Decision Tasks

#### TASK P5-35: Resolve GAP-ROLE-3 — pre-defined vs runtime knowledge composition (addresses GAP-ROLE-3)

**What:** GAP-ROLE-3 identifies that roles predefine knowledge composition at definition time rather than allowing runtime composition. This is a DESIGN DECISION task — evaluate whether predefined knowledge composition is intentional architecture (roles as curated knowledge bundles) or a limitation that should be addressed with a runtime composition mechanism. Write findings with recommendation.

**Files created:**
- Findings document with recommendation

**Acceptance Criteria:**
- [ ] Analysis of current role-based knowledge declaration behavior
- [ ] Comparison of predefined vs runtime composition approaches with pros/cons
- [ ] Recommendation for whether current behavior is correct or needs change
- [ ] If change needed: scope of work and which phase it belongs in
- [ ] If no change needed: explicit justification documented

**Reviewer Checks:**
- Verify the analysis addresses the specific concern from GAP-ROLE-3
- Verify the recommendation is actionable (not "needs further investigation")

---

### Validation Tasks

#### TASK P5-36: Validate all manifests against canonical schema (was P5-34)

**What:** Run all 17 `orqa-plugin.json` files through the canonical schema created in P5-01. Fix any validation errors found. This is the final validation gate for Phase 5.

**Files needed:**
- All 17 `orqa-plugin.json` files
- `targets/plugin-manifests/orqa-plugin.schema.json`

**Acceptance Criteria:**
- [ ] All 17 manifests pass schema validation with zero errors
- [ ] All manifests have `purpose` field
- [ ] All workflow plugins have `stage_slot` field
- [ ] All manifests have `affects_schema` and `affects_enforcement` fields
- [ ] All manifests use category values from the ARCHITECTURE.md taxonomy
- [ ] No duplicate IDs exist in any manifest's `knowledge_declarations`
- [ ] No `../../.orqa/` relative paths exist in any manifest's `content_file` references
- [ ] All `content_file` paths resolve to existing files

**Reviewer Checks:**
- Run the validation independently
- Spot-check 3 manifests by hand against the schema
- Verify the validation script/tool exists and is reusable

**Dependencies:** All P5-01 through P5-35 must be complete

---

#### TASK P5-37: Verify orqa install works with standardized manifests (was P5-35)

**What:** Run `orqa install` end-to-end with the standardized manifests to verify the installation pipeline works correctly. This tests that the new fields don't break existing functionality and that the new constraint enforcement (P5-26, P5-27, P5-28) works.

**Acceptance Criteria:**
- [ ] `orqa install` completes without errors
- [ ] All plugins are installed correctly
- [ ] Knowledge files are installed to correct locations
- [ ] Workflows are installed to correct locations
- [ ] Schema composition succeeds (if `affects_schema: true` plugins are present)
- [ ] No runtime errors in the daemon after installation
- [ ] One-methodology enforcement works (tested with mock conflict)
- [ ] One-per-stage enforcement works (tested with mock conflict)

**Reviewer Checks:**
- Run `orqa install` independently
- Verify installed content matches plugin source
- Test constraint enforcement with deliberate violations

**Dependencies:** P5-36 must be complete

---

## Summary

### Phase 4 Task Count
- 5 prerequisite engine endpoint tasks (P4-PRE-1 through P4-PRE-5)
- 15 connector refactoring tasks (P4-01 through P4-15)
- **Total: 20 tasks**

### Phase 5 Task Count
- 1 schema definition task (P5-01)
- 17 manifest taxonomy update tasks (P5-02 through P5-18, one per plugin + connector)
- 4 duplicate resolution tasks (P5-19 through P5-22)
- 2 missing file resolution tasks (P5-23, P5-24)
- 1 schema field standardization task (P5-25)
- 3 installation constraint tasks (P5-26 through P5-28)
- 2 plugin rename tasks (P5-29, P5-30)
- 1 role resolution task (P5-31)
- 1 workflow resolution task (P5-32)
- 1 multi-purpose declaration task (P5-33)
- 1 content installation targets task (P5-34)
- 1 design decision task (P5-35)
- 2 validation tasks (P5-36, P5-37)
- **Total: 37 tasks**

### Grand Total: 57 tasks

### Dependency Chain
```
Phase 4:
  P4-PRE-1..5 (engine endpoints) -> P4-01..04 (hook refactoring) -> P4-05 (shared.ts cleanup)
  P4-06..09 (independent: delete, simplify, move knowledge)
  P4-10..12 (generator + watcher + hooks.json generation) -- can start after P4-01..05
  P4-13 (validation) -- after P4-10..12
  P4-14 (exports cleanup) -- after P4-06, P4-10, P4-11
  P4-15 (build verification) -- LAST in Phase 4

Phase 5:
  P5-01 (schema) -> P5-02..18 (manifest updates, parallelizable)
  P5-19..24 (dedup + missing files, parallelizable, no dependency on P5-01)
  P5-25 (schema field naming) -- can run in parallel with P5-02..18
  P5-26..28 (installer enforcement) -- after P5-02..18 (need purpose/stage_slot/affects_* fields to exist)
  P5-29 (rename) -- after P5-02 (need category fixed first)
  P5-30..31 (design decisions) -- independent
  P5-32 (resolved workflows) -- after P5-06..08 (need stage_slot and contribution declarations fixed)
  P5-33 (core multi-purpose) -- after P5-03
  P5-34 (content install targets) -- after P5-01..33
  P5-35 (GAP-ROLE-3 decision) -- independent
  P5-36 (validation) -- after ALL other P5 tasks
  P5-37 (integration test) -- after P5-36
```

### GAP Coverage Verification

Every GAP-* from phase2-02-plugin-gaps.md is addressed:

| Gap ID | Addressed By |
|--------|-------------|
| GAP-TAX-1 | P5-01, P5-02 through P5-18 |
| GAP-TAX-2 | P5-02, P5-05 |
| GAP-TAX-3 | P5-03 |
| GAP-TAX-4 | P5-04, P5-05, P5-06, P5-07, P5-08 |
| GAP-TAX-5 | P5-01, P5-03 through P5-08 |
| GAP-MAN-1 | P5-01, P5-02 through P5-18 |
| GAP-MAN-2 | P5-01, P5-03 through P5-08 |
| GAP-MAN-3 | P5-01, P5-02 through P5-18 |
| GAP-MAN-4 | P5-08 |
| GAP-MAN-5 | P5-06 |
| GAP-MAN-6 | P5-23 |
| GAP-MAN-7 | P5-24 |
| GAP-MAN-8 | P5-25 |
| GAP-AGENT-1 | Deferred to Phase 7 (AGENT-*.md removal) — noted in P5-19 |
| GAP-AGENT-2 | Deferred to Phase 7 (AGENT-*.md removal) — noted in P5-19 |
| GAP-DUP-1 | P5-19 |
| GAP-DUP-2 | P5-20 |
| GAP-DUP-3 | P5-21 |
| GAP-DUP-4 | P5-22 |
| GAP-MISS-1 | P5-23 |
| GAP-MISS-2 | P5-24 |
| GAP-ROLE-1 | P5-31 |
| GAP-ROLE-2 | No action needed (correctly placed) |
| GAP-ROLE-3 | P5-35 (design decision: pre-defined vs runtime knowledge composition) |
| GAP-WF-1 | P5-08 |
| GAP-WF-2 | P5-06 |
| GAP-WF-3 | P5-32 |
| GAP-NAME-1 | P5-29 |
| GAP-NAME-2 | P5-30 |
| GAP-DUAL-1 | P5-10, P5-13, P5-08 |
| GAP-DUAL-2 | P5-33 |
| GAP-INST-1 | P5-26 |
| GAP-INST-2 | P5-27 |
| GAP-INST-3 | P5-28 |
| GAP-INST-4 | P5-06 |

Every boundary violation from phase2-03-connector-gaps.md is addressed:

| Violation | Addressed By |
|-----------|-------------|
| prompt-injector.ts (MAJOR) | P4-PRE-1, P4-01 |
| knowledge-injector.ts (MAJOR) | P4-PRE-2, P4-02 |
| save-context.ts (MODERATE) | P4-PRE-3, P4-03 |
| impact-check.ts (MINOR) | P4-PRE-5, P4-04 |
| shared.ts MCP IPC (MODERATE) | P4-05 |
| artifact-bridge.ts (LEGACY) | P4-06 |
| connector-setup.ts (TRANSITIONAL) | P4-07 |
| session-start.sh (MODERATE) | P4-PRE-4, P4-08 |
