## Review Summary

5 areas reviewed. 4 ARCHITECTURE-GAP findings, 1 PLANNED note, PASS verdicts for compliant areas.

---

## Verdicts

### AC: Connector hooks are thin adapters -- no business logic

**Verdict:** PASS
**Evidence:**
All 8 hook files in connectors/claude-code/src/hooks/ follow the same pattern: read stdin -> call daemon HTTP endpoint -> output result.

- rule-engine.ts: calls POST /hook, outputs block/warn/allow based on daemon response.

- prompt-injector.ts: calls POST /prompt, layers only a connector-specific contextLine -- formatting concern, not business logic.

- knowledge-injector.ts: calls POST /knowledge, formats returned entries -- formatting only.

- save-context.ts: calls POST /compact-context, writes returned document to disk -- pure I/O.

- impact-check.ts: calls POST /parse, formats impact message from daemon -- no scoring or classification logic.

- departure-detection.ts: calls POST /hook, outputs daemon messages.

- findings-check.ts: calls POST /hook, outputs daemon result.

- subagent-review.ts: calls POST /hook, outputs daemon messages.

- validate-artifact.ts: calls POST /parse, formats validation errors from daemon -- no schema logic.

No if/else trees with domain heuristics, no scoring algorithms, no rule evaluation. All enforcement logic is delegated to the daemon.

---

### AC: Connector source language is Rust (calls engine crates directly)

**Verdict:** ARCHITECTURE-GAP
**Evidence:**
DOC-4d531f5e section 8.2: The connector source is Rust -- it calls engine crates directly (not via daemon HTTP).

DOC-62969bc3 section 3.1: the connector source is Rust (calls engine crates directly), but generates output in whatever language the target tool requires.

The current connector source is TypeScript, not Rust:

- connectors/claude-code/package.json -- TypeScript package (@orqastudio/claude-code-cli)

- connectors/claude-code/tsconfig.json -- TypeScript compiler config

- All source files under connectors/claude-code/src/ are .ts

The hooks call the daemon via HTTP (callDaemon() in shared.ts lines 76-91), contradicting the requirement that the connector calls engine crates directly and makes no HTTP calls to the daemon.

**Issue:** The connector is TypeScript and communicates via daemon HTTP. Violates DOC-4d531f5e section 8.2 and DOC-62969bc3 section 3.1. Requires connector rewrite in Rust with direct engine crate linkage.

---

### AC: Connector watcher is manifest-driven (no hardcoded paths)

**Verdict:** ARCHITECTURE-GAP
**Evidence:**
DOC-4d531f5e section 8.5 prohibits hardcoded WATCH_DIRS constants. Watch paths must come from plugin manifest declarations (provides.watchers) at daemon startup.

connectors/claude-code/src/watcher.ts lines 26-34 contain hardcoded constants:
  WATCH_PATTERNS = [".orqa/workflows", ".orqa/learning/rules", ".orqa/schema.composed.json"]
  PLUGINS_DIR = "plugins"
  PLUGIN_MANIFEST_NAME = "orqa-plugin.json"

The watcher also lives in the connector TypeScript source, not in the daemon manifest-driven registry as the architecture requires. Hardcoded watcher constants are explicitly listed as an anti-pattern in DOC-4d531f5e section 8.5.

**Issue:** WATCH_PATTERNS and PLUGINS_DIR are hardcoded. Per DOC-4d531f5e section 8.5, watch paths must come from plugin manifest declarations managed by the daemon registry.

---

### AC: CLI has orqa enforce as a top-level command

**Verdict:** ARCHITECTURE-GAP
**Evidence:**
cli/src/commands/enforce.ts exists and implements runEnforceCommand. However, enforce is NOT registered in cli/src/cli.ts. The switch statement (lines 78-132) routes to: install, plugin, check, test, build, graph, daemon, mcp, metrics, summarize, lsp, version, id, migrate, git, dev. No case for enforce.

DOC-62969bc3 section 3.3: orqa enforce is the universal enforcement entry point. orqa enforce --staged is what the pre-commit git hook calls. Flags like --eslint, --clippy, --vale are dynamically generated from installed plugins.

enforce is currently only accessible as orqa check enforce (sub-command of check).

**Issue:** orqa enforce exists as a module but is not wired into cli.ts as a top-level command. Architecture requires it as a first-class top-level command.

---

### AC: Pre-commit hook calls orqa enforce --staged

**Verdict:** ARCHITECTURE-GAP
**Evidence:**
DOC-62969bc3 section 3.3: orqa enforce --staged is what the pre-commit git hook calls.

The actual .githooks/pre-commit hook does NOT call orqa enforce --staged. It calls:

- $ORQA fix autolink --staged  (orqa fix is not a registered top-level command)

- $ORQA validate --staged      (orqa validate is not a registered top-level command)

- $ORQA check rustfmt

- $ORQA check clippy

- npx eslint (line 148)            -- HARDCODED direct tool invocation, violates P1

- npx markdownlint-cli2 (line 163) -- HARDCODED direct tool invocation, violates P1

- $ORQA check stubs --staged

- $ORQA check lint-suppressions --staged

- $ORQA check test-rust --staged

- $ORQA check test-frontend --staged

The hook hardcodes npx eslint and npx markdownlint-cli2 instead of routing through the plugin registry. Violates P1 (Plugin-Composed Everything). The hook also calls non-existent top-level commands.

**Issue:** Pre-commit hook hardcodes npx eslint and npx markdownlint-cli2 directly. Must route through orqa enforce. Hook also calls non-existent top-level commands (orqa validate, orqa fix autolink).

---

### AC: CLI language is TypeScript (PLANNED migration to Rust)

**Verdict:** PLANNED
**Evidence:**
DOC-62969bc3 section 3.1: Rust is the base language for all libraries, the CLI, and the daemon.
Current CLI (cli/src/cli.ts) is TypeScript. The CLI wraps the engine via subprocess and HTTP calls. Flagged as PLANNED -- migration docs plan this transition. No remediation required in the current phase.

---

### AC: generator.ts contains generation and file-writing logic only

**Verdict:** PASS
**Evidence:**
connectors/claude-code/src/generator.ts delegates to:

- agent-file-generator.ts for agent .md files

- daemon POST /context for active rule titles and workflow names

- plugin manifests (provides.hooks, provides.mcpServers, provides.lspServers) for hooks.json, .mcp.json, and .lsp.json generation

buildClaudemd() contains a hardcoded fallback P1-P7 principles table (lines 187-199). This is reference content used as a fallback when the architecture doc is absent -- not enforcement business logic. Acceptable as a formatting fallback.

No if/else enforcement trees or domain heuristics. The generator reads from plugin manifests and daemon, and writes files.

---

### AC: connector-setup.ts owns only Claude Code-specific wiring

**Verdict:** PASS
**Evidence:**
connectors/claude-code/src/connector-setup.ts limits itself to building the .claude/agents/ merged directory by symlinking core agent files and plugin-declared agent files. Explicitly notes that symlinks and aggregated files are handled by the plugin framework universal mechanisms. The CLI installer has no knowledge of .claude/ -- correct boundary separation.

---

## Blocking Issues

1. **Connector source language (ARCHITECTURE-GAP)**: Connector is TypeScript making daemon HTTP calls, not Rust calling engine crates directly. Violates DOC-4d531f5e section 8.2 and DOC-62969bc3 section 3.1. Requires connector rewrite in Rust.

2. **Hardcoded watch paths (ARCHITECTURE-GAP)**: WATCH_PATTERNS and PLUGINS_DIR constants in connectors/claude-code/src/watcher.ts lines 26-34. DOC-4d531f5e section 8.5 prohibits this -- watch paths must come from plugin manifest declarations managed by the daemon.

3. **Pre-commit hook hardcodes tool invocations (ARCHITECTURE-GAP)**: .githooks/pre-commit lines 148 and 163 invoke npx eslint and npx markdownlint-cli2 directly. Must route through orqa enforce --staged. Hook also calls non-existent commands (orqa validate, orqa fix autolink).

4. **orqa enforce not registered as top-level command (ARCHITECTURE-GAP)**: enforce.ts exists but not wired into cli.ts. Only accessible as orqa check enforce. Architecture requires it as a first-class top-level command.
