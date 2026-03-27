# Phase 2: Connector Gap Analysis + Target Claude Code Plugin

## Question

What boundary violations exist in the current Claude Code connector, what should the generated Claude Code Plugin look like, and what should the connector's actual job be once cleaned up?

---

## Part 1: Boundary Violation Analysis

For each connector source file/hook, classification against ARCHITECTURE.md section 8.3: "The connector's code should be generation and translation logic only. If it contains if/else trees, scoring algorithms, or domain-specific heuristics, it has exceeded its role."

### CORRECTLY SCOPED Files

#### `src/hooks/rule-engine.ts` (48 lines)
**Verdict: CORRECTLY SCOPED** -- pure thin adapter.
- Reads stdin, builds `HookContext`, calls daemon `POST /hook`, maps result to block/warn/allow.
- Zero business logic. All enforcement is in the Rust daemon.
- **Keep as-is in generated plugin.**

#### `src/hooks/memory-redirect.ts` (41 lines)
**Verdict: CORRECTLY SCOPED** -- structurally identical to rule-engine.ts.
- Reads stdin, builds `HookContext`, calls daemon `POST /hook`.
- Daemon decides which memory paths redirect to lessons.
- **Keep as-is in generated plugin.**

#### `src/hooks/findings-check.ts` (54 lines)
**Verdict: CORRECTLY SCOPED** -- thin adapter with telemetry.
- Reads stdin, builds `HookContext`, calls daemon `POST /hook`.
- Daemon enforces RULE-04684a16 (findings-file existence).
- **Keep as-is in generated plugin.**

#### `src/hooks/departure-detection.ts` (57 lines)
**Verdict: CORRECTLY SCOPED** -- thin adapter with telemetry.
- Reads stdin, builds `HookContext`, calls daemon `POST /hook`.
- Daemon handles departure pattern matching.
- **Keep as-is in generated plugin.**

#### `src/hooks/subagent-review.ts` (54 lines)
**Verdict: CORRECTLY SCOPED** -- thin adapter with telemetry.
- Reads stdin, builds `HookContext`, calls daemon `POST /hook`.
- Daemon handles stub detection, deferral scanning, artifact integrity.
- **Keep as-is in generated plugin.**

#### `src/hooks/validate-artifact.ts` (83 lines)
**Verdict: CORRECTLY SCOPED** -- thin adapter.
- Calls daemon `POST /parse`, formats validation errors as systemMessage.
- The `isOrqaArtifact()` path filter is legitimate pre-flight (avoids calling daemon for non-artifact files).
- Error message formatting is presentation concern -- belongs in the connector.
- **Keep as-is in generated plugin.**

#### `src/hooks/telemetry.ts` (35 lines)
**Verdict: CORRECTLY SCOPED** -- fire-and-forget telemetry logger.
- Sends metrics to dev controller dashboard.
- Pure I/O marshalling, no business logic.
- **Keep as-is in generated plugin.**

#### `src/types.ts` (83 lines)
**Verdict: CORRECTLY SCOPED** -- type definitions only.
- Defines Claude Code hook JSON contract interfaces.
- **Keep as-is in generated plugin.**

#### `src/index.ts` (26 lines)
**Verdict: CORRECTLY SCOPED** -- pure re-export/entry point.
- Re-exports graph/daemon APIs from `@orqastudio/cli`.
- **This is connector source code, not generated plugin code.** The generated plugin does not need a library entry point.

### BOUNDARY VIOLATIONS

#### `src/hooks/prompt-injector.ts` (303 lines) -- MAJOR VIOLATION

**Violated principles:** ARCHITECTURE.md 8.3 ("Prompt generation/assembly" belongs in the engine's prompt pipeline), 3.1 (engine provides prompt pipeline), 7 (prompt generation pipeline).

**Specific violations:**

1. **Prompt classification heuristics (lines 42-79, 237-280):** The three-tier classification system -- semantic search against thinking-mode artifacts (Tier 1), keyword regex matching (Tier 2), and "general" default (Tier 3) -- is business logic that belongs in the engine. The connector should not decide what type of work the user is doing; the engine's prompt pipeline should accept a raw user message and return a classified, composed prompt.

2. **Thinking-mode resolution (lines 30-43):** The `THINKING_MODE_MAP` and `resolveThinkingMode()` function encode domain-specific mappings (`"learning-loop" -> "governance"`, `"dogfood-implementation" -> "implementation"`). This is workflow knowledge that belongs in the engine or a workflow plugin.

3. **Workflow stage mapping (lines 86-97):** `promptTypeToStage()` maps prompt types to workflow stage strings. This mapping belongs in the engine -- the connector should not know the names of workflow stages.

4. **Preamble document composition (lines 166-190):** The connector composes a full preamble document and writes it to `.state/orchestrator-preamble.md`. This is prompt pipeline output formatting that the engine should own.

**What should move to the engine:**
- Prompt classification (semantic + keyword) -> engine `POST /classify` or integrated into `generatePrompt()`
- Thinking-mode mapping -> workflow plugin definitions consumed by the engine
- Stage resolution -> engine's workflow engine
- Preamble composition -> engine's prompt pipeline output

**What should remain in the connector (thin wrapper):**
```
async function main() {
  const input = await readInput();
  const userMessage = input.user_message ?? input.prompt ?? "";
  const projectDir = input.cwd ?? ".";

  // Single engine call that handles classification + generation
  const result = await callDaemon("/prompt", {
    message: userMessage,
    role: input.agent_type ?? "orchestrator",
    project_path: projectDir,
  });

  // Connector-specific UX (stays here)
  const parts = [result.prompt, getContextLine(projectDir)];
  process.stdout.write(JSON.stringify({ systemMessage: parts.join("\n\n") }));
  process.exit(0);
}
```

The `getContextLine()` function (lines 287-300) is CORRECTLY SCOPED -- it reads project.json for a connector-specific UX line. This stays.

---

#### `src/hooks/knowledge-injector.ts` (195 lines) -- MAJOR VIOLATION

**Violated principles:** ARCHITECTURE.md 8.3 ("Knowledge injection algorithms" belong in the engine's prompt pipeline), 6.2 (engine generates task-specific agents), 7 (prompt generation pipeline).

**Specific violations:**

1. **Role detection heuristics (lines 42-57):** `ROLE_PATTERNS` regex array detects agent roles from prompt text. This is a heuristic that should be in the engine -- the engine should know what role an agent has, not the connector parsing prompt text with regex.

2. **Knowledge query and deduplication (lines 66-84, 130-159):** Layer 1 reads the prompt registry and queries knowledge by role. This is exactly what the engine's prompt pipeline does (ARCHITECTURE.md section 7: "Schema Assembly" stage -- "For a (base role, workflow, task) tuple, collect applicable prompt sections").

3. **Semantic search scoring threshold (lines 33-34):** `MIN_SCORE = 0.25` and `MAX_SEMANTIC = 5` are domain-specific heuristics about knowledge relevance. These belong in the engine's knowledge injection tier configuration.

4. **Query extraction from prompts (lines 98-100):** Extracting a search query from the agent prompt by finding the first `\n\n` boundary is a heuristic that the engine should own.

5. **Knowledge injection message composition (lines 162-179):** Formatting knowledge references into a Claude Code-friendly message is borderline. The format is connector-specific (Claude Code systemMessage), but the content selection is engine business logic.

**What should move to the engine:**
- Role detection -> engine already knows the role at delegation time (it generates the agent)
- Knowledge query by role -> engine's prompt pipeline "Schema Assembly" stage
- Semantic search with scoring -> engine's prompt pipeline "Section Resolution" stage
- Knowledge deduplication -> engine's prompt pipeline
- Query extraction -> engine's search engine

**What should remain in the connector (thin wrapper):**
```
async function main() {
  const input = await readInput();
  const prompt = input.tool_input?.prompt ?? "";
  const projectDir = input.cwd ?? process.cwd();

  // Engine returns the knowledge that should be injected
  const result = await callDaemon("/knowledge", {
    agent_prompt: prompt,
    project_path: projectDir,
  });

  if (result.entries.length === 0) {
    outputAllow();
  }

  // Format for Claude Code's systemMessage (connector-specific presentation)
  const parts = formatKnowledgeEntries(result.entries);
  outputWarn(parts);
}
```

---

#### `src/hooks/impact-check.ts` (88 lines) -- MINOR VIOLATION

**Violated principles:** ARCHITECTURE.md 8.3 ("Impact analysis logic" belongs in the engine's graph engine).

**Specific violation:**

1. **Downstream relationship threshold (line 57):** `shouldInject = highInfluence || downstreamCount > 20` -- the threshold of 20 is a domain-specific heuristic. The daemon already provides `high_influence` and `downstream_count`; the daemon should also provide the `should_inject` decision (or a richer `impact_level` classification).

**What should move to the engine:**
- The `shouldInject` decision -> daemon `POST /parse` response should include a `should_warn: boolean` or `impact_level: "high" | "low"` field, making the threshold configurable at the engine level.

**What should remain:**
- Everything else is correctly scoped. The file reads daemon output and formats a Claude Code systemMessage. Once the threshold moves to the daemon, this becomes a pure adapter.

---

#### `src/hooks/save-context.ts` (112 lines) -- MODERATE VIOLATION

**Violated principles:** ARCHITECTURE.md 8.3 (borderline -- "what to preserve during compaction" is business logic).

**Specific violations:**

1. **Governance context document composition (lines 47-79):** The hook decides what information to preserve before compaction (active epics, active tasks, previous session state). It also writes recovery instructions that reference specific paths (`.orqa/process/agents/orchestrator.md`). The structure of what constitutes "important context" is a business decision.

2. **Hard-coded artifact path reference (line 77):** `"3. '.orqa/process/agents/orchestrator.md' for your role definition"` -- references a path that should not exist in the target architecture (agents are generated, not stored as files).

**What should move to the engine:**
- Context preservation logic -> engine `POST /compact-context` endpoint that returns the structured governance context to preserve. The engine knows what epics/tasks are active and what the agent needs after compaction.

**What should remain:**
- Writing the result to `.state/governance-context.md` (file I/O is connector-specific).
- Building the Claude Code `systemMessage` summary.

---

#### `src/hooks/shared.ts` (267 lines) -- MIXED

**Correctly scoped portions:**
- `readInput()` -- Claude Code stdin parsing (lines 52-58)
- `callDaemon()` -- HTTP POST to daemon (lines 64-79)
- `mapEvent()` -- Claude Code event name -> canonical event name (lines 82-93)
- `outputBlock()`, `outputWarn()`, `outputAllow()` -- Claude Code output formatting (lines 99-120)
- `isOrqaArtifact()` -- path filter (lines 127-131)
- `callBinary()` -- binary fallback (lines 251-266)

**Boundary violations:**

1. **MCP IPC protocol implementation (lines 170-242):** The full JSON-RPC-over-TCP client with MCP initialize handshake is too much protocol complexity for a connector hook. The connector should call the engine (daemon) for search, and the engine routes to its search subsystem. The connector should not have a direct IPC connection to the MCP server.

2. **IPC port file resolution (lines 145-163):** Platform-specific port file discovery (`LOCALAPPDATA` / `~/.local/share/`) is infrastructure that the engine/CLI should abstract. The connector should call `orqa search` or `POST /search` on the daemon, not establish its own MCP TCP connection.

**What should move:**
- `mcpSearchCall()` and `readIpcPort()` should be replaced by a daemon endpoint `POST /search` that the connector calls via `callDaemon()`. The daemon already handles search internally.

---

#### `src/artifact-bridge.ts` (284 lines) -- LEGACY, SHOULD NOT EXIST

**Violated principles:** ARCHITECTURE.md 6.4 ("No more monolithic AGENT-*.md specialist definitions"), 8.1 ("the connector is not in the runtime path -- it is a live generation pipeline").

The ArtifactBridge maps `.claude/` to `.orqa/` via symlinks for three hardcoded paths: CLAUDE.md -> orchestrator agent, rules/ -> process rules, agents/ -> process agents. In the target architecture:
- There is no `agents/` directory (agents are generated at runtime).
- CLAUDE.md should be generated by the connector, not symlinked to an orchestrator agent file.
- Rules symlink is already handled by `orqa-plugin.json` `provides.symlinks`.

**What should happen:** Delete this file. Its functionality is superseded by `connector-setup.ts` (for agents) and the plugin framework's `provides.symlinks` (for rules).

---

#### `src/connector-setup.ts` (201 lines) -- TRANSITIONAL, PARTIALLY CORRECT

**Correctly scoped:** The concept of a merged agents directory with core + plugin agents is valid installation-time logic. Building `.claude/agents/` from plugin manifests is connector-specific directory wiring.

**Violations:**
1. **Hard-coded path assumptions (lines 60-62):** `app/.orqa/process/agents/` fallback to `.orqa/process/agents/` encodes monorepo structure knowledge. The engine/CLI should resolve agent sources.
2. **Plugin manifest parsing (lines 147-169):** Directly reading `orqa-plugin.json` from each plugin directory. This should use the CLI's plugin registry APIs, not raw file parsing.

In the target architecture, agents are generated at runtime -- there should be no agents directory to wire up. This file is transitional infrastructure for the current (legacy) agent model.

---

#### Shell Scripts

**`hooks/scripts/daemon-gate.sh` (31 lines) -- CORRECTLY SCOPED.** Pure health check. Blocks if daemon unreachable. Stays in generated plugin.

**`hooks/scripts/session-start.sh` (128 lines) -- MODERATE VIOLATION.**
- Lines 26-46: Installation check (connector installation verification) -- correctly scoped.
- Lines 48-65: Daemon health gate -- correctly scoped (duplicate of daemon-gate.sh, could consolidate).
- Lines 67-73: `orqa enforce --fix` -- correctly scoped (calls engine CLI).
- Lines 76-87: Git state warnings -- correctly scoped (presentation concern).
- Lines 89-106: Session continuity (loading `.state/` files) -- borderline. The decision of what files to load is business logic, but it's minimal and presentation-focused.
- Lines 108-115: Dogfood mode detection -- correctly scoped.
- Lines 117-122: Session start checklist -- correctly scoped (UX concern).

The main violation is that the script does too many things. In the target architecture, session start should be a single daemon call (`POST /session-start`) that returns a structured response, and the script should format it for Claude Code output.

**`hooks/scripts/stop-checklist.sh` (57 lines) -- CORRECTLY SCOPED.** Safety net for session state. Minimal business logic.

---

### Violation Summary

| File | Severity | Primary Violation |
|------|----------|-------------------|
| `prompt-injector.ts` | **MAJOR** | Prompt classification + generation pipeline in connector |
| `knowledge-injector.ts` | **MAJOR** | Knowledge injection algorithms in connector |
| `save-context.ts` | **MODERATE** | Context preservation logic in connector |
| `impact-check.ts` | **MINOR** | Threshold heuristic in connector (1 line) |
| `shared.ts` (MCP IPC) | **MODERATE** | Direct MCP protocol client bypasses daemon |
| `artifact-bridge.ts` | **LEGACY** | Entire file is superseded |
| `connector-setup.ts` | **TRANSITIONAL** | Partially correct, hard-coded paths |
| `session-start.sh` | **MODERATE** | Does too many things, should be one daemon call |

---

## Part 2: Target Claude Code Plugin

This is the ideal OUTPUT that the connector should GENERATE. It is a Claude Code Plugin -- the tool-native artifact that Claude Code consumes.

### 2.1 Directory Structure

```
.claude-plugin/
  plugin.json                    # Claude Code plugin manifest
  marketplace.json               # Local marketplace descriptor

hooks/
  hooks.json                     # Hook wiring (events -> scripts)
  dist/
    rule-engine.js               # PreToolUse: Write|Edit|Bash|TeamCreate|Agent
    knowledge-injector.js        # PreToolUse: Agent (knowledge injection)
    memory-redirect.js           # PreToolUse: Write|Edit (memory -> lessons)
    validate-artifact.js         # PostToolUse: Write|Edit (artifact validation)
    findings-check.js            # PostToolUse: TaskUpdate (findings file check)
    prompt-injector.js           # UserPromptSubmit (prompt classification + injection)
    departure-detection.js       # UserPromptSubmit (departure signal detection)
    save-context.js              # PreCompact (governance context preservation)
    subagent-review.js           # SubagentStop (subagent output review)
    shared.js                    # Shared I/O: readInput, callDaemon, output*
    telemetry.js                 # Fire-and-forget telemetry
  scripts/
    daemon-gate.sh               # UserPromptSubmit gate: block if no daemon
    session-start.sh             # SessionStart: health checks + session recovery
    stop-checklist.sh            # Stop: session state safety net

commands/
  orqa.md                        # /orqa — governance summary + graph browser
  orqa-save.md                   # /orqa-save — write session state
  orqa-create.md                 # /orqa-create — guided artifact creation
  orqa-validate.md               # /orqa-validate — integrity check

agents/                          # Generated at install time, not bundled
  orchestrator.md                # -> symlink or generated from base role
  implementer.md                 # -> symlink or generated from base role
  reviewer.md                    # -> symlink or generated from base role
  researcher.md                  # -> symlink or generated from base role
  writer.md                      # -> symlink or generated from base role
  (plugin-provided agents)       # -> symlinks to plugin agent files

rules/                           # -> symlink to .orqa/rules/
  (symlinked rule files)

CLAUDE.md                        # Generated: project-specific orchestrator context

.mcp.json                        # Aggregated MCP server configs
.lsp.json                        # Aggregated LSP server configs
```

### 2.2 Permission Configuration

Role-scoped file access enforced by the generated hooks via the daemon. The daemon evaluates role permissions; hooks apply the result. Permission model defined here for documentation; enforcement is mechanical.

| Role | Read Scope | Write Scope | Shell Access | Notes |
|------|-----------|-------------|--------------|-------|
| **Orchestrator** | Everything | `.state/session-state.md` only | No | Reads summaries, delegates work |
| **Implementer** | Everything | `src/`, `app/`, `lib/`, tests, configs | Yes | Source code and build tools |
| **Reviewer** | Everything | None (read-only) | Yes (checks only: test, lint, typecheck) | Produces verdicts, no edits |
| **Researcher** | Everything | `.state/team/*/task-*.md`, `.orqa/discovery/research/` | No | Creates research artifacts only |
| **Writer** | Everything | `docs/`, `.orqa/documentation/`, README files | No | Documentation only |
| **Governance Steward** | Everything | `.orqa/` files only | No | Governance artifacts only |

These permissions are enforced by `rule-engine.js` which calls the daemon. The daemon has the role-to-scope mapping. The generated plugin does NOT hardcode these scopes -- they come from the engine.

### 2.3 Agent Definitions

In the target architecture, the `agents/` directory contains generated agent definitions. Each agent file is generated by the connector from the engine's base role definitions + workflow context.

**Base agent files (generated from methodology plugin's base roles):**

```markdown
# Orchestrator

You coordinate work through delegation. You do NOT implement.

## Boundaries
- Read any file, but do NOT edit source code
- Delegate ALL implementation to background agents via teams
- Read structured summaries from findings files
- Stay available for conversation with the user

## Tool Access
- TeamCreate, TaskCreate, TaskUpdate, Agent, SendMessage
- Read, Glob, Grep (read-only exploration)
- No: Write, Edit, Bash (except .state/session-state.md)

## Knowledge References
(injected dynamically by knowledge-injector hook based on active workflow)
```

Similar generated files for Implementer, Reviewer, Researcher, Writer. Each contains:
1. Role identity and behavioral boundaries
2. Tool access restrictions (enforced mechanically by rule-engine hook)
3. Artifact scope (what they can create/edit)
4. Placeholder for dynamically injected knowledge references

**In the target state:** These files are generated by the connector at install time (and regenerated when plugins change). They are NOT hand-authored AGENT-*.md files from `.orqa/process/agents/`.

### 2.4 Commands (Slash Commands)

| Command | File | Purpose |
|---------|------|---------|
| `/orqa` | `commands/orqa.md` | Governance summary, graph browser reference, artifact type hierarchy, relationship vocabulary. The "help" command. |
| `/orqa-save` | `commands/orqa-save.md` | Write session state to `.state/session-state.md` with structured sections. |
| `/orqa-create` | `commands/orqa-create.md` | Guided artifact creation with type selection, ID allocation, relationship determination. |
| `/orqa-validate` | `commands/orqa-validate.md` | Full integrity check reference. Runs `orqa enforce` and explains results. |

Commands are static markdown files. They do not contain business logic -- they are instructions to the LLM that reference CLI commands (`orqa graph`, `orqa enforce`) for actual operations. **Correctly scoped as-is.**

### 2.5 Hooks (Target State)

Every hook should follow the pattern: **receive event -> call engine (via daemon) -> format response for Claude Code**. No hook should contain business logic beyond presentation formatting.

| Hook Script | Event | Matcher | Target State |
|-------------|-------|---------|-------------|
| `rule-engine.js` | PreToolUse | Write\|Edit\|Bash\|TeamCreate\|Agent | **DONE** -- already a pure thin adapter |
| `knowledge-injector.js` | PreToolUse | Agent | **NEEDS REFACTOR** -- should call `POST /knowledge` and format result |
| `memory-redirect.js` | PreToolUse | Write\|Edit | **DONE** -- already a pure thin adapter |
| `validate-artifact.js` | PostToolUse | Write\|Edit | **DONE** -- already a pure thin adapter |
| `findings-check.js` | PostToolUse | TaskUpdate | **DONE** -- already a pure thin adapter |
| `prompt-injector.js` | UserPromptSubmit | * | **NEEDS REFACTOR** -- should call `POST /prompt` and format result |
| `departure-detection.js` | UserPromptSubmit | * | **DONE** -- already a pure thin adapter |
| `save-context.js` | PreCompact | * | **NEEDS REFACTOR** -- should call `POST /compact-context` and write result |
| `subagent-review.js` | SubagentStop | * | **DONE** -- already a pure thin adapter |
| `daemon-gate.sh` | UserPromptSubmit | * | **DONE** -- pure health check |
| `session-start.sh` | SessionStart | * | **NEEDS REFACTOR** -- should call `POST /session-start` and format result |
| `stop-checklist.sh` | Stop | * | **DONE** -- minimal safety net |

**Target hook structure (all hooks):**

```typescript
// Target: every hook follows this pattern
async function main() {
  const input = await readInput();
  const context = buildContext(input);    // Map Claude Code event to canonical
  const result = await callDaemon(endpoint, context);  // Single engine call
  formatAndOutput(result);                // Claude Code-specific formatting
}
```

### 2.6 What Is NOT Included

The generated Claude Code Plugin does NOT contain:

| Excluded Component | Where It Belongs | Why |
|-------------------|-----------------|-----|
| **Git hooks** (pre-commit, commit-msg) | `githooks` plugin generates these from engine rules | Different enforcement layer |
| **ESLint configs** | `coding-standards` or `typescript` plugin | Language-specific tooling |
| **Clippy configs** | `rust` plugin | Language-specific tooling |
| **Prettier configs** | `coding-standards` plugin | Language-specific tooling |
| **Test runners/configs** | Project infrastructure, not governance | Not OrqaStudio's concern |
| **Build scripts** | Project infrastructure | Not governance |
| **CI/CD workflows** | The connector's own CI (for publishing the plugin package) is separate from the generated plugin's content | These `.github/` files are for the connector NPM package |
| **Knowledge files** | Installed into `.orqa/knowledge/` by `orqa install` | Part of `.orqa/`, not `.claude/` |
| **Prompt registry** | Built by the engine at install time | Engine's internal data |
| **MCP/LSP server definitions** | Aggregated into `.mcp.json`/`.lsp.json` by the plugin framework | Declared in `orqa-plugin.json`, assembled by `orqa install` |

---

## Part 3: The Connector's Actual Job

### 3.1 What the Connector Source Should Be

After cleanup, the connector's source code should contain ONLY:

1. **Generation logic** -- transforms engine output into Claude Code Plugin format
2. **File watching** -- watches for plugin/rule/workflow changes and regenerates
3. **Installation wiring** -- Claude Code-specific directory setup at install time
4. **Thin hook adapters** -- event -> daemon call -> Claude Code output format

### 3.2 Target Connector Source Structure

```
connectors/claude-code/
  src/
    index.ts                     # Entry point: re-exports + CLI setup command
    generator.ts                 # NEW: generates the Claude Code Plugin from engine data
    watcher.ts                   # NEW: watches for changes, triggers regeneration
    connector-setup.ts           # SIMPLIFIED: install-time wiring only
    types.ts                     # Claude Code hook JSON contract types

    hooks/                       # Thin adapters ONLY (no business logic)
      shared.ts                  # SIMPLIFIED: readInput, callDaemon, output*, mapEvent
                                 #   REMOVED: mcpSearchCall, readIpcPort (use daemon instead)
      telemetry.ts               # Unchanged
      rule-engine.ts             # Unchanged (already thin)
      memory-redirect.ts         # Unchanged (already thin)
      findings-check.ts          # Unchanged (already thin)
      departure-detection.ts     # Unchanged (already thin)
      subagent-review.ts         # Unchanged (already thin)
      validate-artifact.ts       # Unchanged (already thin)
      impact-check.ts            # SIMPLIFIED: remove threshold, daemon provides should_warn
      prompt-injector.ts         # SIMPLIFIED: call POST /prompt, format result
      knowledge-injector.ts      # SIMPLIFIED: call POST /knowledge, format result
      save-context.ts            # SIMPLIFIED: call POST /compact-context, write result

  hooks/
    hooks.json                   # Unchanged
    scripts/
      daemon-gate.sh             # Unchanged
      session-start.sh           # SIMPLIFIED: call daemon, format result
      stop-checklist.sh          # Unchanged

  commands/                      # Unchanged (static markdown)
  knowledge/                     # Unchanged (installed into .orqa/ by orqa install)
  skills/                        # Unchanged (installed by orqa install)
  .claude-plugin/                # Unchanged (plugin manifest)
  orqa-plugin.json               # Unchanged (OrqaStudio plugin manifest)
  package.json                   # Unchanged
  tsconfig.json                  # Unchanged
```

### 3.3 Files to Delete

| File | Reason |
|------|--------|
| `src/artifact-bridge.ts` | Legacy. Superseded by connector-setup.ts and provides.symlinks. |

### 3.4 New Engine Endpoints Required

For the connector to become thin, the engine needs these daemon endpoints:

| Endpoint | Input | Output | Currently in |
|----------|-------|--------|-------------|
| `POST /prompt` | `{ message, role, project_path }` | `{ prompt, type, method, tokens, budget, sections }` | prompt-injector.ts classification + @orqastudio/cli generatePrompt |
| `POST /knowledge` | `{ agent_prompt, project_path }` | `{ entries: [{ id, title, path, source, score? }] }` | knowledge-injector.ts Layer 1 + Layer 2 |
| `POST /compact-context` | `{ project_path }` | `{ context_document, summary }` | save-context.ts composition logic |
| `POST /session-start` | `{ project_path }` | `{ checks, warnings, session_state, checklist }` | session-start.sh health checks + context loading |
| `POST /parse` (enhanced) | `{ file }` | `{ ..., should_warn: boolean }` | impact-check.ts threshold + existing /parse |

### 3.5 New Connector Modules

#### `generator.ts` -- The Connector's Primary Job

The generator is what makes the connector a connector. It transforms engine data into a Claude Code Plugin:

```typescript
/**
 * Generate a Claude Code Plugin from the engine's composed state.
 *
 * This is the connector's primary job. It reads:
 * - Base role definitions from the methodology plugin
 * - Composed workflow from resolved YAML
 * - Active rules and their enforcement entries
 * - Installed plugin agent declarations
 *
 * And generates:
 * - .claude/agents/*.md (from base roles + workflow context)
 * - .claude/CLAUDE.md (generated orchestrator context)
 * - hooks/hooks.json (assembled from plugin hook declarations)
 * - .mcp.json, .lsp.json (aggregated server configs)
 */
export function generatePlugin(projectRoot: string): GenerateResult { ... }
```

#### `watcher.ts` -- Live Regeneration

```typescript
/**
 * Watch for changes to plugins, rules, and workflow compositions.
 * Regenerates the Claude Code Plugin when the composition changes.
 *
 * Watches:
 * - .orqa/workflows/*.resolved.yaml
 * - .orqa/rules/*.md
 * - plugins/*/orqa-plugin.json
 * - .orqa/schema.composed.json
 */
export function watchAndRegenerate(projectRoot: string): void { ... }
```

### 3.6 What Changes in Each Violating File

#### `prompt-injector.ts`: 303 -> ~40 lines
- DELETE: `classifyWithSearch()`, `classifyPrompt()`, `resolveThinkingMode()`, `THINKING_MODE_MAP`, `promptTypeToStage()`, preamble composition
- KEEP: `getContextLine()` (connector-specific UX), input parsing, output formatting
- ADD: Single `callDaemon("/prompt", ...)` call

#### `knowledge-injector.ts`: 195 -> ~35 lines
- DELETE: `ROLE_PATTERNS`, `detectRole()`, `getDeclaredKnowledge()`, `searchSemanticKnowledge()`, `MIN_SCORE`, `MAX_SEMANTIC`
- KEEP: Input parsing, output formatting, telemetry
- ADD: Single `callDaemon("/knowledge", ...)` call

#### `save-context.ts`: 112 -> ~30 lines
- DELETE: Dual daemon queries, context document composition, recovery instructions
- KEEP: File writing (`.state/governance-context.md`), systemMessage output
- ADD: Single `callDaemon("/compact-context", ...)` call

#### `impact-check.ts`: 88 -> ~75 lines
- DELETE: `shouldInject = highInfluence || downstreamCount > 20` (1 line)
- CHANGE: Use daemon's `should_warn` response field instead
- Minimal change -- almost correctly scoped already.

#### `shared.ts`: 267 -> ~135 lines
- DELETE: `mcpSearchCall()` (73 lines), `readIpcPort()` (11 lines), `getIpcPortFilePath()` (5 lines)
- KEEP: Everything else (readInput, callDaemon, mapEvent, output*, isOrqaArtifact, callBinary)

#### `session-start.sh`: 128 -> ~40 lines
- DELETE: Manual health checks, session file loading, dogfood detection
- KEEP: Session guard, output formatting
- ADD: Single `curl` call to daemon `POST /session-start`

### 3.7 Net Effect

| Metric | Current | Target |
|--------|---------|--------|
| Total source lines (hooks/) | ~1,100 | ~500 |
| Files with business logic | 5 | 0 |
| Daemon endpoints used | 3 (`/hook`, `/parse`, `/query`) | 7 (add `/prompt`, `/knowledge`, `/compact-context`, `/session-start`) |
| MCP IPC code in connector | 73 lines | 0 (daemon handles search) |
| Classification heuristics in connector | 2 systems (semantic + keyword) | 0 (engine classifies) |
| Knowledge injection logic in connector | 2 layers (declared + semantic) | 0 (engine injects) |

---

## Recommendations

### Priority Order

1. **Move prompt classification to engine** (CRITICAL) -- This is the largest boundary violation. The entire prompt pipeline entry point is in the connector. Create `POST /prompt` daemon endpoint that accepts raw user message and returns the full composed prompt. This consolidates `classifyWithSearch()` + `classifyPrompt()` + `generatePrompt()` into a single engine call.

2. **Move knowledge injection to engine** (CRITICAL) -- The second-largest violation. Create `POST /knowledge` daemon endpoint that accepts an agent prompt and returns the knowledge entries to inject. Eliminates role detection regex, prompt registry queries, and MCP IPC from the connector.

3. **Remove MCP IPC from shared.ts** (HIGH) -- The connector should not maintain a TCP connection to the MCP server. All search should go through the daemon. This is a prerequisite for both #1 and #2.

4. **Add `should_warn` to daemon /parse response** (LOW) -- One-line change in impact-check.ts after the daemon enhancement.

5. **Consolidate session-start into daemon call** (MEDIUM) -- Create `POST /session-start` that returns all health check results + session context. Simplifies the shell script significantly.

6. **Move context preservation to engine** (MEDIUM) -- Create `POST /compact-context` that returns the governance context document. Simplifies save-context.ts.

7. **Delete artifact-bridge.ts** (LOW) -- Legacy file. Its functionality is already covered by other mechanisms.

8. **Build generator.ts** (FUTURE) -- The connector's primary job per ARCHITECTURE.md 8.1. This is the path to generated-not-hand-written plugins, but requires the engine to have mature agent generation first.

### Prerequisite: Engine Daemon Endpoints

The refactoring above requires 4-5 new daemon endpoints. These should be built in the Rust engine first, then the connector hooks can be simplified. The order should be:

1. Build daemon endpoints
2. Simplify connector hooks (one at a time, test each)
3. Build generator.ts
4. Build watcher.ts
5. Test: generated plugin matches hand-written plugin

This follows ARCHITECTURE.md 8.4's strategy: hand-write the target, then build the generation pipeline to produce it.

---

## Open Questions

1. **Should telemetry remain in the connector?** The current telemetry sends metrics to a dev controller at `localhost:10401`. This is connector-specific infrastructure. In the target state, should the daemon provide a unified telemetry endpoint, or is it acceptable for the connector to have its own? (Current answer: acceptable -- it's a presentation/debugging concern, not business logic.)

2. **Should the connector generate hooks.json?** Currently hooks.json is a static file in the connector source. In the target architecture, should the connector generate it from plugin hook declarations, or is static wiring acceptable? (Current lean: static wiring is fine -- the hook scripts themselves are what get generated, not their wiring.)

3. **What happens to the 10 knowledge files?** The connector provides 10 knowledge artifacts (artifact-creation, delegation-patterns, decision-tree, etc.). Some of these are OrqaStudio domain knowledge that arguably belongs in the core or methodology plugin, not the connector. For example, `delegation-patterns` and `decision-tree` are workflow knowledge, not connector-specific. This needs a separate analysis of which knowledge belongs where.
