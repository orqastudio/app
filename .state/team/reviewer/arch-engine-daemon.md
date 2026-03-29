# Architecture Review: Engine + Daemon vs Architecture Principles

**Reviewer:** reviewer agent
**Scope:** All Rust components: engine crates (types, enforcement, workflow, plugin, prompt, validation, search, agent) and daemon (watcher, config, health, tray, LSP, MCP)
**Architecture docs consulted:** DOC-62969bc3 (core), DOC-70063f55 (enforcement/state machines), DOC-41ccf7c4 (plugins)
**Date:** 2026-03-29
**Tests run:** cargo test -p orqa-workflow: 107 passed, 0 failed
**Linter:** cargo clippy -p orqa-workflow -- -D warnings: clean

---

## Summary

| Verdict | Count |
|---------|-------|
| PASS | 13 |
| ARCHITECTURE-GAP (blocking) | 2 |
| ARCHITECTURE-GAP (lower severity / flag) | 2 |
| OBSERVATION (non-blocking) | 2 |

---

## Component Verdicts

### 1. engine/types

**Verdict:** PASS

All types in engine/types/src/types/ are pure data structures with no governance patterns. Artifact types, relationship types, enforcement types, and workflow types are open structures that callers populate from plugin manifests. No hardcoded rule names, relationship keys, or artifact type lists. Consistent with P1 and P4.

---

### 2. engine/enforcement (scanner.rs + engine.rs)

**Verdict:** PASS

EnforcementEngine::new(rules) receives rules entirely from the caller. No internal rule path, no rule file, no hardcoded rule names. Scanner uses caller-provided ArtifactEntry configs to determine which areas to scan. Nothing is hardcoded in the enforcement engine itself. Consistent with P1.

---

### 3. engine/workflow/transitions.rs

**Verdict:** PASS

evaluate_transitions(graph, statuses) takes caller-provided &[StatusDefinition]. Condition dispatch uses a string-match pattern with open fallthrough for unknown conditions — new conditions can be added without changing the engine. When statuses is empty, returns empty list. Correct open-world design. Consistent with P1 and P4.

---

### 4a. engine/workflow/tracker.rs (path classification)

**Verdict:** PASS

WorkflowTracker receives a TrackerConfig from the caller at construction time. The config holds Vec``<PathRule>`` — path patterns that classify reads into Docs, Planning, and Lessons categories. No path prefixes are hardcoded in the tracker. The module comment explicitly states: "No paths are hardcoded in the tracker itself." Custom config tests confirm the design works with arbitrary caller-supplied patterns. Consistent with P1.

---

### 4b. engine/workflow/tracker.rs (verification command detection) [ARCHITECTURE-GAP]

**Verdict:** ARCHITECTURE-GAP — lower severity

record_command() at lines 169-175 hardcodes six specific verification command substrings:

```text
make check, make test, cargo test, cargo clippy, npm run test, npm run check

```text

This is inconsistent with the config-driven pattern used for path classification in the same module. The set of commands that count as verification is a caller-supplied concern — it should come from TrackerConfig like path rules do. A project using different verification commands (e.g. just verify or pnpm test) would not have those recognised as verification.

**Required fix:** Extend TrackerConfig with a verification_patterns: Vec``<String>`` field (substring match, same pattern as path rules). record_command() tests against config.verification_patterns. Default config can ship the current six strings.

---

### 5. engine/workflow/gates.rs (GateCondition enum) [ARCHITECTURE-GAP — BLOCKING]

**Verdict:** ARCHITECTURE-GAP — BLOCKING

The GateCondition enum is a closed Rust enum with five hardcoded variants:

```text
FirstCodeWriteWithoutResearch
CodeWriteWithoutDocs
CodeWriteWithoutPlanning
CodeWrittenWithoutVerification
SignificantWorkWithoutLessons { threshold: usize }

```text

The module comment says "No gate names, conditions, or messages are hardcoded in this module" — this is accurate for names and messages, but the condition LOGIC is hardcoded. The set of observable session facts that can trigger a gate is fixed in the Rust type system. A plugin cannot declare a new condition type without a Rust code change to the enum.

This violates P1 (Plugin-Composed Everything) and P4 (Declarative Over Imperative). Per DOC-62969bc3 section 2: "State machines, guards, actions, and workflow definitions are YAML declarations validated by JSON Schema — not code. Plugin authors write configuration, not functions."

The evaluate_gate() match block (lines 124-146) embeds evaluation logic for each variant directly. This is imperative code, not declarative evaluation of plugin-supplied condition strings.

Note: The test helper standard_gates() correctly shows that gate definitions (names, messages) come from ProcessGateConfig — this part of the design is right. The problem is that condition evaluation logic is locked to the closed enum variants.

**Required fix:** Replace GateCondition enum with a string-based condition identifier (condition: String) plus optional parameters as HashMap<String, serde_json::Value>. Condition evaluation dispatches by string with open fallthrough for unknown conditions — matching the correct pattern already used in transitions.rs. This makes the condition vocabulary open-ended; plugins can declare any condition name.

---

### 6. engine/plugin

**Verdict:** PASS

Plugin discovery (scan_plugins) reads orqa-plugin.json manifests from the filesystem without hardcoding any plugin names, categories, or rules. Manifest fields are deserialized into open structures. The plugin crate does not define what categories exist or what rules apply. Consistent with P1.

---

### 7. engine/prompt/builder.rs

**Verdict:** PASS

Paths for knowledge artifacts and rules are resolved from ProjectSettings.artifacts when a project.json is present, with well-known defaults as fallback. Agent definitions are sourced exclusively from installed plugins (module comment: "Agent definitions are sourced from installed plugins (P1) rather than any static file"). No governance patterns are hardcoded. The DEFAULT_* constants (lines 21-29) are fallback paths for environments without full setup — these are infrastructure defaults, not governance definitions. Consistent with P1 and P3.

---

### 8. engine/validation/platform.rs

**Verdict:** PASS

PLATFORM static is empty by default. Module comment: "Plugins are now the sole source of truth for artifact type schemas and relationships. There is no longer a compile-time core.json dependency." Meaningful schema data is loaded via scan_plugin_manifests. Consistent with P1.

---

### 9. engine/search

**Verdict:** PASS

Semantic search uses caller-provided query strings and returns results from the indexed corpus. No hardcoded search strategies or fixed result filters. Search parameters (score thresholds, max results) come from caller config (DaemonConfig). Consistent with P1 and P5.

---

### 10. engine/agent/types.rs (BaseRole enum) [ARCHITECTURE-GAP — flag for team]

**Verdict:** ARCHITECTURE-GAP — flag for team decision

BaseRole is a closed Rust enum with eight variants: Orchestrator, Implementer, Reviewer, Researcher, Writer, Planner, Designer, GovernanceSteward.

This is a borderline case. DOC-62969bc3 section 3.2 lists the agent crate as providing "Base role definitions, task-specific agent generation from role + workflow + knowledge" — which suggests base roles are intended as structural engine infrastructure. The module comment says "The engine defines these roles; plugins provide domain-specific specialisations on top of them."

However, P1 states "no governance pattern is hardcoded in the engine" and this set of eight roles IS a governance pattern — it encodes the hub-spoke orchestration methodology. A project with a different team structure would need to change Rust source to alter available roles.

**Assessment:** The architecture docs appear to intentionally make base roles an engine concern. This is a design decision rather than an implementation error. However, it creates a sealed vocabulary. Flag for team awareness.

**Recommended action (not blocking):** Document the intentionality of this decision in the engine crate. If base roles are structural infrastructure, state this explicitly in the architecture docs. If plugins should be able to declare custom roles, the enum should become an open string type.

---

### 11. daemon (main.rs, config.rs, health.rs, tray.rs)

**Verdict:** PASS

DaemonConfig loads from orqa.toml at project root with sensible defaults. All tunable parameters (min_score, max_semantic, downstream_warn_threshold) are config-driven. The daemon startup sequence matches the architecture description in DOC-62969bc3 section 3.3: file watchers, health HTTP endpoint, LSP subprocess (TCP), system tray. MCP is explicitly NOT managed by the daemon — the module comment in main.rs explains this correctly: MCP uses stdio transport, each LLM client spawns its own orqa-mcp-server process. Consistent with architecture.

---

### 12. daemon/watcher.rs [ARCHITECTURE-GAP — lower severity]

**Verdict:** ARCHITECTURE-GAP — lower severity

WATCH_DIRS = &[".orqa", "plugins"] and RULES_DIR = ".orqa/learning/rules" are hardcoded constants at lines 30 and 40.

DOC-70063f55 section 10.2 states: "The daemon reads [the manifest] at startup/install: registers watch paths..." and the manifest declares watch.paths as the source. The architecture describes dynamically registering watch paths from installed plugin manifests, not from a fixed constant list.

Currently the daemon watches a static set of directories. As enforcement plugins are added that declare their own watch.paths, the daemon will not pick up those paths automatically.

**Severity note:** The current two hardcoded paths (.orqa, plugins) cover most cases for the current phase of development. This is lower severity than the GateCondition issue, but it is an architecture divergence that will need to be addressed when enforcement plugins with custom watch paths are installed.

**Required fix (can defer):** At startup, after scanning installed plugins, extract enforcement.watch.paths from each manifest and register them with the debouncer. The static WATCH_DIRS fallback can remain as baseline coverage.

---

### 13. daemon (mcp.rs, lsp.rs, process.rs)

**Verdict:** PASS

MCP correctly documents that it is NOT managed by the daemon — MCP uses stdio transport, each LLM client spawns its own orqa-mcp-server process. LSP is correctly managed as a TCP subprocess (legitimately persistent). Process management (PID file, single-instance guard, cleanup) is clean. Consistent with DOC-62969bc3 section 3.3.

---

### 14. daemon/prompt.rs

**Verdict:** PASS

Classification vocabulary is loaded from plugin manifests at handler time rather than being hardcoded. This is the correct P1 pattern.

---

## Observations (non-blocking)

### OBS-1: Dual HTTP server implementations

engine/validation/src/daemon.rs contains its own HTTP daemon mode using tiny_http. The main daemon/ binary also runs an HTTP health endpoint. Two separate HTTP server implementations exist in the codebase. This may be intentional (the validation daemon can run standalone) but worth tracking to ensure they do not diverge in behavior or conflict when both run simultaneously.

### OBS-2: gates.rs module comment inaccuracy

gates.rs module comment says "No gate names, conditions, or messages are hardcoded in this module." This is accurate for names and messages, but the GateCondition enum means condition LOGIC is hardcoded. The comment should be updated when the GateCondition issue is fixed.

---

## Blocking Issues

### BLOCKING-1: GateCondition closed enum (engine/workflow/src/gates.rs lines 32-47)

The GateCondition enum is a closed Rust enum with five hardcoded condition variants. This violates P1 (Plugin-Composed Everything) and P4 (Declarative Over Imperative). Condition evaluation logic is imperative Rust code, not declarative string dispatch.

**Required fix:** Replace the enum with a string condition identifier + optional params map, dispatching by string with open fallthrough (matching the correct pattern in transitions.rs).

### BLOCKING-2: Hardcoded verification commands (engine/workflow/src/tracker.rs lines 169-175)

record_command() hardcodes six specific verification command substrings. Inconsistent with the config-driven pattern used for path classification in the same module.

**Required fix:** Add verification_patterns: Vec``<String>`` to TrackerConfig. Default can include the current six strings.

---

## Non-Blocking Issues (flag for team)

- **BaseRole enum (engine/agent/src/types.rs):** Closed Rust enum encoding hub-spoke role taxonomy. Architecture docs appear to intend this as structural infrastructure (not a plugin concern), but if that assumption changes this becomes a blocker. Recommend documenting the intentionality.
- **daemon/watcher.rs hardcoded watch dirs:** Static WATCH_DIRS will not pick up watch.paths from enforcement plugin manifests. Lower severity for current phase; can defer with a known-gap note.
- **Dual HTTP server implementations:** engine/validation/src/daemon.rs and daemon/health.rs. Track to avoid divergence.
