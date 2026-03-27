# Phase 2: Engine Extraction & App Gap Analysis

## Question

Compare the current app backend (128 files, ~22,400 lines) and frontend (123 components) against ARCHITECTURE.md Section 3.1 ("The Engine is a standalone Rust crate"). Classify every backend module, identify frontend hardcoding violations, and propose the engine crate's public API.

---

## Part 1: Backend Module Classification

### Current State Summary

ARCHITECTURE.md states: "The engine is a **standalone Rust crate** -- an independent process, not embedded in any consumer." Currently:

- **2 small crates extracted**: `orqa-validation` (graph, integrity, metrics, auto-fix) and `orqa-search` (chunking, embedding, DuckDB search)
- **~12,400 lines of business logic remain embedded** in the Tauri backend (`app/src-tauri/src/domain/`)
- **2 access protocol servers exist** (`orqa-mcp-server`, `orqa-lsp-server`) but they depend on the extracted crates directly, not on a unified engine crate
- The app binary serves triple duty: Tauri desktop app, MCP server spawner (`--mcp`), and LSP server spawner (`--lsp`)

### Module-by-Module Classification

#### ENGINE -- Must move to standalone engine crate

These modules contain business logic that the engine should provide. They are currently embedded in the Tauri backend and unavailable to CLI, MCP, or LSP consumers without going through Tauri.

| Module | File | Lines | Engine Subsystem | Notes |
|--------|------|-------|-----------------|-------|
| Artifact types | `domain/artifact.rs` | 687 | Core types | `Artifact`, `ArtifactType` enum, ID generation, frontmatter parsing. Foundation type used everywhere. |
| Artifact filesystem | `domain/artifact_fs.rs` | 301 | Core I/O | Read/write/scan/delete artifacts on disk. Every consumer needs this. |
| Artifact reader | `domain/artifact_reader.rs` | 874 | Navigation | Config-driven navigation tree scanner. Builds `NavigationNode` trees from `project.json`. |
| Enforcement types | `domain/enforcement.rs` | 113 | Enforcement | `EventType`, `RuleAction`, `Verdict`, `EnforcementRule` type definitions. |
| Enforcement engine | `domain/enforcement_engine.rs` | 783 | Enforcement | Regex-based rule evaluation for file/bash/scan/lint events. Core enforcement capability. |
| Enforcement parser | `domain/enforcement_parser.rs` | 340 | Enforcement | Parses YAML frontmatter from `.md` rule files. |
| Enforcement violation | `domain/enforcement_violation.rs` | 24 | Enforcement | `EnforcementViolation` struct. |
| Status transitions | `domain/status_transitions.rs` | 880 | Workflow | Config-driven engine with 5 named conditions. Core state machine evaluation. |
| Process gates | `domain/process_gates.rs` | 548 | Workflow | 5 gates evaluating session process state. Workflow guard capability. |
| Process state | `domain/process_state.rs` | 287 | Workflow | Session state tracking for gate evaluation. |
| Workflow tracker | `domain/workflow_tracker.rs` | 406 | Workflow | Session activity tracking (files read/written, searches, docs). |
| Knowledge injector | `domain/knowledge_injector.rs` | 477 | Prompt pipeline | ONNX-based cosine similarity matching for knowledge injection. |
| System prompt builder | `domain/system_prompt.rs` | 270 | Prompt pipeline | Constructs system prompts from rules + knowledge + CLAUDE.md. |
| Governance scanner | `domain/governance_scanner.rs` | 422 | Enforcement | Scans 6 governance areas in `.orqa/`. |
| Governance types | `domain/governance.rs` | 43 | Enforcement | `GovernanceScanResult`, `GovernanceArea` structs. |
| Hook manager | `hooks/manager.rs` | 415 | Plugin system | Git hook dispatcher generation from plugin registry. |
| Platform config | `domain/platform_config.rs` | 251 | Core types | Compile-time `core.json` loading, relationship/type definitions, inverse map building. |
| Project types | `domain/project.rs` | 119 | Core types | `Project`, `DetectedStack`, `ScanResult` structs. |
| Project scanner | `domain/project_scanner.rs` | 396 | Core I/O | Recursive walk, language/framework/package manager detection. |
| Project settings | `domain/project_settings.rs` | 251 | Core types | Partially re-exports from `orqa_validation`, adds local types. |
| Config loader | `domain/config_loader.rs` | 94 | Core I/O | Reads `.orqa/project.json`. |
| Paths | `domain/paths.rs` | 242 | Core I/O | Config-driven path resolution from `project.json`. |
| Health snapshot | `domain/health_snapshot.rs` | 48 | Metrics | `HealthSnapshot` struct with 14 metrics. |
| Lessons | `domain/lessons.rs` | 252 | Core types | `Lesson` struct, parse/render. |
| Message types | `domain/message.rs` | 192 | Core types | `Message`, `MessageRole`, `ContentType`, `StreamStatus`. |
| Session types | `domain/session.rs` | 128 | Core types | `Session`, `SessionSummary`, `SessionStatus`. |
| Session title | `domain/session_title.rs` | 116 | Core logic | Auto-title generation via sidecar summary. |
| Settings types | `domain/settings.rs` | 171 | Core types | `ResolvedTheme`, `SidecarStatus`, `SidecarState`. |
| Provider events | `domain/provider_event.rs` | 288 | Streaming | `StreamEvent` enum with 16 variants. |
| Stream loop | `domain/stream_loop.rs` | 1,042 | Streaming | Core streaming infrastructure, tool approval, enforcement integration. |
| Tool executor | `domain/tool_executor.rs` | 1,140 | Streaming | 11 tool implementations + enforcement hooks. |
| Time utils | `domain/time_utils.rs` | 272 | Utilities | Calendar utilities (no chrono dependency). |
| CLI tool runner | `cli_tools/runner.rs` | 299 | Plugin system | Runs plugin-registered CLI tools. |

**Total ENGINE lines: ~10,155** (34 modules)

#### ENGINE (Already Extracted)

| Crate | Subsystem | Lines (approx) | Status |
|-------|-----------|----------------|--------|
| `orqa-validation` | Graph, integrity, metrics, auto-fix, content, hooks, platform schema | ~3,000+ | Extracted, standalone daemon binary |
| `orqa-search` | Chunking, embedding, DuckDB store, search engine | ~1,500+ | Extracted, standalone daemon binary |

#### BRIDGE -- Thin wrappers over extracted crates (stay in app, but shrink)

| Module | File | Lines | Notes |
|--------|------|-------|-------|
| Graph bridge | `domain/artifact_graph.rs` | 196 | Wrapper over `orqa_validation`. Stays as Tauri command delegation. |
| Integrity bridge | `domain/integrity_engine.rs` | 101 | Re-exports from `orqa_validation`. Stays as Tauri command delegation. |
| Search re-export | `search/mod.rs` | 11 | Re-exports `orqa_search`. Stays. |

**Total BRIDGE lines: ~308**

#### TAURI GLUE -- Stays in the app

These modules are app-specific: Tauri command wrappers, IPC, window management, app lifecycle, database init.

| Module | File(s) | Lines | Notes |
|--------|---------|-------|-------|
| Entry point | `main.rs` | 52 | CLI flag parsing, app/MCP/LSP dispatch |
| Library root | `lib.rs` | 268 | App state construction, 58 command registration, plugin registration |
| App state | `state.rs` | 127 | `AppState` with 8 sub-structs |
| Error types | `error.rs` | 231 | `OrqaError` enum |
| Database init | `db.rs` | 245 | SQLite init, WAL mode, migrations |
| Logging | `logging.rs` | 149 | Two-tier tracing subscriber + Tauri event emission |
| Startup tracking | `startup.rs` | 94 | Tracks long-running init tasks |
| File watcher | `watcher.rs` | 248 | Watches `.orqa/` for changes, emits events |
| 19 command modules | `commands/*.rs` | ~2,500 | Tauri IPC command handlers |
| Sidecar manager | `sidecar/manager.rs` | 333 | Child process lifecycle management |
| Sidecar protocol | `sidecar/protocol.rs` | 160 | NDJSON wire format |
| Sidecar types | `sidecar/types.rs` | 558 | Request/response enums |
| Server spawners | `servers/*.rs` | 363 | IPC socket, MCP/LSP binary spawning |

**Total TAURI GLUE lines: ~5,328**

#### PLUGIN INFRASTRUCTURE -- Moves to engine (plugin system is an engine capability)

| Module | File | Lines | Notes |
|--------|------|-------|-------|
| Collision detection | `plugins/collision.rs` | 188 | Relationship collision detection between plugins |
| Discovery | `plugins/discovery.rs` | 70 | Plugin scanning from `project.json` |
| Installer | `plugins/installer.rs` | 317 | Local + GitHub plugin installation |
| Lockfile | `plugins/lockfile.rs` | 92 | Plugin lockfile management |
| Manifest | `plugins/manifest.rs` | 145 | `orqa-plugin.json` reading and validation |
| Registry | `plugins/registry.rs` | 159 | Official + community plugin registry fetching |

**Total PLUGIN INFRASTRUCTURE lines: 971** (should move to engine -- plugin system is listed as an engine capability in ARCHITECTURE.md)

#### DATA ACCESS -- Assessment

| Module | File | Lines | Destination |
|--------|------|-------|-------------|
| Enforcement rules repo | `repo/enforcement_rules_repo.rs` | 129 | **Engine** -- reads rule files from disk |
| Lesson repo | `repo/lesson_repo.rs` | 371 | **Engine** -- file-based lesson storage |
| Project settings repo | `repo/project_settings_repo.rs` | 124 | **Engine** -- file-based project.json I/O |
| Health snapshot repo | `repo/health_snapshot_repo.rs` | 220 | **App** -- SQLite persistence, app-specific |
| Message repo | `repo/message_repo.rs` | 436 | **App** -- SQLite + FTS5, app-specific |
| Project repo | `repo/project_repo.rs` | 289 | **App** -- SQLite, multi-project management is app-specific |
| Session repo | `repo/session_repo.rs` | 444 | **App** -- SQLite, session management is app-specific |
| Settings repo | `repo/settings_repo.rs` | 173 | **App** -- SQLite key-value, app preferences |
| Theme repo | `repo/theme_repo.rs` | 268 | **App** -- SQLite, theming is app-specific |
| Violations repo | `repo/violations_repo.rs` | 145 | **App** -- SQLite, violation history is app-specific |

**File-based repos to engine: ~624 lines** (enforcement rules, lessons, project settings)
**SQLite repos stay in app: ~1,975 lines**

#### SETUP -- Stays in app (Claude CLI detection is app-specific)

| Module | File | Lines | Notes |
|--------|------|-------|-------|
| Claude CLI setup | `domain/setup.rs` | 428 | CLI detection, version parsing, auth checking. App-specific. |

---

### Extraction Summary

| Destination | Modules | Lines |
|-------------|---------|-------|
| **Engine crate (new)** | 34 domain + 6 plugin + 3 file-based repos | ~11,750 |
| **Engine crate (already extracted)** | `orqa-validation` + `orqa-search` | ~4,500+ |
| **Stays in app (Tauri glue)** | Commands, sidecar, servers, DB, state, logging | ~5,328 |
| **Stays in app (SQLite repos)** | 7 SQLite repositories | ~1,975 |
| **Stays in app (setup)** | Claude CLI setup | 428 |
| **Bridges (shrink)** | 3 thin wrappers | ~308 |

**The engine extraction gap is ~11,750 lines of business logic that should not be in the Tauri backend.**

---

## Part 2: Frontend Hardcoding Analysis

Every instance where the frontend hardcodes governance patterns instead of receiving them from the engine via Tauri commands or plugin-provided configuration.

### HIGH Severity (direct P1 violations)

| # | Component | What Is Hardcoded | Where It Should Come From | Impact |
|---|-----------|-------------------|--------------------------|--------|
| 1 | `MarkdownLink.svelte` | `ARTIFACT_ID_RE` regex with prefixes: EPIC, TASK, MS, RES, DEC, AD, REQ, RISK, etc. | Engine's composed schema -- artifact types with their `id_prefix` field | Adding a plugin with a new artifact type will produce IDs that the frontend cannot detect or render as links. Fundamental P1 violation. |
| 2 | `StatusBar.svelte` | `sidecarPluginName = "@orqastudio/plugin-claude"` | Plugin registry's `activeSidecarKey` | Assumes a specific sidecar plugin. If a different sidecar is installed (e.g., OpenAI, Ollama), the status bar will not track it. |

### MEDIUM Severity (should be data-driven)

| # | Component | What Is Hardcoded | Where It Should Come From |
|---|-----------|-------------------|--------------------------|
| 3 | `model-options.ts` | `CLAUDE_MODELS` array (auto, opus, sonnet, haiku) | Sidecar plugin should advertise available models |
| 4 | `ModelSettings.svelte` | Same model options duplicated | Should share with `model-options.ts` or come from sidecar |
| 5 | `ProjectScanningSettings.svelte` | Same model options duplicated again | Third copy of the same list |
| 6 | `FrontmatterHeader.svelte` | `SKIP_FIELDS`, `CHIP_FIELDS`, `LINK_FIELDS`, `BOOLEAN_FIELDS`, `FIELD_ORDER`, priority color classes (P0-P3) | Artifact schema definitions from the composed schema should describe field rendering hints |
| 7 | `ArtifactViewer.svelte` | Fallback `stages` array `[draft, in_progress, review, done]` | Project config -- should never fall back to hardcoded stages |
| 8 | `ArtifactLanding.svelte` | `categoryConfig` mapping category keys to `{icon, label, description}` for process/delivery/discovery/governance/principles | Navigation tree config from plugin composition |
| 9 | `DynamicArtifactTable.svelte` | `PRIORITY_ORDER` (P0=0..P3=3), `STATUS_ORDER` (draft=0..blocked=6) | Project status config (status machine defines order), priority definitions from schema |
| 10 | `LessonVelocityWidget.svelte` | Stage definitions with colors (identified, active, promoted, resolved) | Lesson lifecycle config from the learning workflow plugin |
| 11 | `ImprovementTrendsWidget.svelte` | Hardcoded governance types: `rule`, `lesson`, `decision` queried by `.byType()` | Composed schema should define which types are "governance" types |
| 12 | `DecisionQueueWidget.svelte` | `actionLabel` function maps artifact type to action label (decision -> "Decide", task -> "Assign") | Workflow plugin should define action labels per artifact type per status |
| 13 | `EmbeddingModelStep.svelte` | Model name "all-MiniLM-L6-v2" displayed in setup | Search engine config should advertise its model name |

### LOW Severity (cosmetic or reasonable defaults)

| # | Component | What Is Hardcoded | Assessment |
|---|-----------|-------------------|------------|
| 14 | `category-colors.ts` | Lesson category color map (process, technical, team, etc.) | Cosmetic. Could come from theme config but low priority. |
| 15 | `tool-display.ts` | Tool icons and labels for tool calls | Cosmetic. Tool display is app UI concern. |
| 16 | `TraceabilityPanel.svelte` | `iconForType` mapping (epic->flag, task->check-square) | Cosmetic. Could come from artifact type definitions. |
| 17 | `SettingsCategoryNav.svelte` | App and project settings categories | Settings structure is core app feature, not governance. |
| 18 | `ShortcutsSettings.svelte` | 5 keyboard shortcuts | App feature, not governance. |
| 19 | `ProjectSetupWizard.svelte` | Default excluded paths | Reasonable defaults. |
| 20 | `GraphHealthPanel.svelte` | Health metric severity thresholds | Could be configurable but reasonable as defaults. |
| 21 | `SetupComplete.svelte` | 4-step checklist labels | Setup flow is core app feature. |
| 22 | `SetupWizard.svelte` | Step IDs for routing | Setup flow is core app feature. |
| 23 | `ToolCallCard.svelte` | Regex for enforcement block detection | Pattern matching on known engine output format. |
| 24 | `ExplorerRouter.svelte` | `CORE_VIEWS` record | Core view routing is legitimate app concern. |
| 25 | `ArtifactLink.svelte` | `DEFAULT_ARTIFACT_LINK_COLORS` fallback | Has override mechanism via settings. Fallback is acceptable. |

### Hardcoding Pattern Summary

| Severity | Count | Pattern |
|----------|-------|---------|
| HIGH | 2 | Artifact type prefixes, sidecar plugin name -- will break with new plugins |
| MEDIUM | 11 | Model lists, field rendering, status stages, lifecycle stages -- will not adapt to plugin composition |
| LOW | 12 | Cosmetic, reasonable defaults, or core app concerns |
| **Total** | **25** | |

### Key Insight: The Triplicated Model List

The model options list (`auto, claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5`) appears in THREE places:
1. `model-options.ts` (used by `ModelSelector.svelte` in conversations)
2. `ModelSettings.svelte` (app settings)
3. `ProjectScanningSettings.svelte` (project settings)

This should be a single list provided by the active sidecar plugin, which knows what models it supports.

---

## Part 3: Engine Crate Design

### Proposed Crate: `orqa-engine`

The engine crate unifies all business logic under a single API. It consumes `orqa-validation` and `orqa-search` as internal dependencies (or absorbs them). All consumers (app, CLI, MCP server, LSP server) depend only on `orqa-engine`.

### Dependency Graph (Target)

```
                    +-------------------+
                    |   orqa-engine     |
                    |                   |
                    | (absorbs or       |
                    |  depends on       |
                    |  orqa-validation   |
                    |  and orqa-search)  |
                    +--------+----------+
                             |
            +----------------+----------------+
            |                |                |
     +------+------+  +-----+------+  +------+------+
     |  Tauri app  |  |    CLI     |  | MCP server  |
     |  (UI glue)  |  |            |  | LSP server  |
     +-------------+  +------------+  +-------------+
```

### Public API Surface

The engine crate should expose these capability modules:

#### 1. Graph Engine (`orqa_engine::graph`)

```rust
// Already in orqa-validation, promoted to engine API
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph>;
pub fn validate(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck>;
pub fn compute_health(graph: &ArtifactGraph) -> GraphHealth;
pub fn auto_fix(graph: &ArtifactGraph, checks: &[IntegrityCheck], path: &Path) -> Result<Vec<AppliedFix>>;
pub fn compute_traceability(graph: &ArtifactGraph, id: &str) -> TraceabilityResult;
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats;
```

#### 2. Workflow Engine (`orqa_engine::workflow`)

```rust
// Currently in domain/status_transitions.rs (880 lines)
pub fn evaluate_transitions(artifact: &Artifact, relationships: &[ArtifactRef], rules: &[StatusRule]) -> Vec<ProposedTransition>;

// Currently in domain/process_gates.rs (548 lines)
pub fn evaluate_process_gates(process_state: &SessionProcessState) -> Vec<GateResult>;

// Currently in domain/workflow_tracker.rs (406 lines)
pub struct WorkflowTracker { ... }
impl WorkflowTracker {
    pub fn add_file_read(&mut self, path: &str);
    pub fn add_file_write(&mut self, path: &str);
    pub fn add_search(&mut self, query: &str, results: usize);
    pub fn add_doc_consulted(&mut self, path: &str);
    pub fn add_verification(&mut self, kind: &str, passed: bool);
}
```

#### 3. Enforcement Engine (`orqa_engine::enforcement`)

```rust
// Currently in domain/enforcement_engine.rs (783 lines)
pub struct EnforcementEngine { ... }
impl EnforcementEngine {
    pub fn load(project_root: &Path) -> Result<Self>;
    pub fn evaluate_file(&self, path: &str, content: &str) -> Vec<Verdict>;
    pub fn evaluate_bash(&self, command: &str) -> Vec<Verdict>;
    pub fn evaluate_scan(&self, diagnostics: &str) -> Vec<Verdict>;
    pub fn evaluate_lint(&self, output: &str) -> Vec<Verdict>;
}

// Currently in domain/enforcement_parser.rs (340 lines)
pub fn parse_enforcement_rule(content: &str) -> Result<EnforcementRule>;

// Currently in domain/governance_scanner.rs (422 lines)
pub fn scan_governance(project_root: &Path) -> GovernanceScanResult;
```

#### 4. Prompt Pipeline (`orqa_engine::prompt`)

```rust
// Currently in domain/system_prompt.rs (270 lines)
pub fn build_system_prompt(project_root: &Path, knowledge_matches: &[KnowledgeMatch]) -> String;

// Currently in domain/knowledge_injector.rs (477 lines)
pub struct KnowledgeInjector { ... }
impl KnowledgeInjector {
    pub fn load(knowledge_dir: &Path) -> Result<Self>;
    pub fn match_prompt(&self, prompt: &str, threshold: f32, max_results: usize) -> Vec<KnowledgeMatch>;
}
```

#### 5. Search Engine (`orqa_engine::search`)

```rust
// Already in orqa-search, promoted to engine API
pub struct SearchEngine { ... }
impl SearchEngine {
    pub fn new(db_path: &Path) -> Result<Self>;
    pub fn index(&mut self, project_path: &Path, exclude: &[&str]) -> Result<IndexStatus>;
    pub fn search_semantic(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>;
    pub fn search_regex(&self, pattern: &str, scope: Option<&str>, limit: usize) -> Result<Vec<SearchResult>>;
}
```

#### 6. Plugin System (`orqa_engine::plugin`)

```rust
// Currently in plugins/ (971 lines total)
pub fn install_from_path(source: &Path, target: &Path) -> Result<InstallResult>;
pub fn install_from_github(repo: &str, target: &Path) -> Result<InstallResult>;
pub fn uninstall(plugin_dir: &Path, lockfile_path: &Path) -> Result<()>;
pub fn detect_collisions(incoming: &[RelationshipDef], existing: &[RelationshipDef]) -> Vec<KeyCollision>;
pub fn read_manifest(plugin_dir: &Path) -> Result<PluginManifest>;
pub fn scan_plugins(project_root: &Path) -> Vec<DiscoveredPlugin>;
pub fn generate_hook_dispatchers(project_root: &Path, registry: &HookRegistry) -> Result<HookGenerationResult>;
```

#### 7. Artifact I/O (`orqa_engine::artifact`)

```rust
// Currently in domain/artifact.rs + artifact_fs.rs + artifact_reader.rs (~1,862 lines)
pub fn parse_artifact(content: &str) -> Result<Artifact>;
pub fn generate_id(type_key: &str) -> String;
pub fn write_artifact(path: &Path, artifact: &Artifact) -> Result<()>;
pub fn read_artifact(path: &Path) -> Result<Artifact>;
pub fn scan_directory(dir: &Path) -> Result<Vec<Artifact>>;
pub fn scan_navigation_tree(project_root: &Path) -> Result<Vec<NavigationNode>>;
```

#### 8. Platform Schema (`orqa_engine::platform`)

```rust
// Currently in domain/platform_config.rs (251 lines) + orqa-validation/platform.rs
pub fn platform_config() -> &'static PlatformConfig;
pub fn build_inverse_map(rels: &[RelationshipDef]) -> HashMap<String, String>;
pub fn build_merged_inverse_map(project_rels: &[ProjectRelationshipConfig]) -> HashMap<String, String>;
pub fn keys_for_semantic(semantic: &str) -> Vec<String>;
```

#### 9. Core Types (`orqa_engine::types`)

```rust
// Consolidated from across domain modules
pub struct Artifact { ... }
pub struct Lesson { ... }
pub struct Message { ... }
pub struct Session { ... }
pub struct Project { ... }
pub struct ProjectSettings { ... }
pub struct DetectedStack { ... }
pub enum ArtifactType { ... }
pub enum StreamEvent { ... }
// ... all type definitions that consumers need
```

### What Stays in Each Consumer

| Consumer | What It Owns |
|----------|-------------|
| **Tauri app** | SQLite repos (messages, sessions, projects, settings, themes, health snapshots, violations). Sidecar management. Window management. IPC command wrappers. File watcher. Logging to frontend events. Setup wizard. |
| **CLI** | Argument parsing. Terminal output formatting. Interactive prompts. |
| **MCP server** | JSON-RPC protocol handling. Tool registration. Stdio transport. |
| **LSP server** | LSP protocol handling. Diagnostic publishing. `tower-lsp` integration. |

### Migration Strategy

The extraction should happen in phases to avoid breaking the working app:

**Phase 1: Create `orqa-engine` crate**
- Create `libs/engine/` with `Cargo.toml`
- Depend on `orqa-validation` and `orqa-search`
- Move type definitions first (zero-risk, no logic changes)

**Phase 2: Move pure business logic**
- Enforcement engine + parser (783 + 340 = 1,123 lines)
- Status transitions (880 lines)
- Process gates + state + tracker (548 + 287 + 406 = 1,241 lines)
- Platform config (251 lines)
- These have no Tauri dependencies.

**Phase 3: Move I/O-dependent logic**
- Artifact I/O (artifact.rs + artifact_fs.rs + artifact_reader.rs = 1,862 lines)
- Knowledge injector (477 lines)
- System prompt builder (270 lines)
- Governance scanner (422 lines)
- Config loader + paths (94 + 242 = 336 lines)
- Project scanner (396 lines)

**Phase 4: Move plugin system**
- All 6 plugin modules (971 lines)
- Hook manager (415 lines)
- CLI tool runner (299 lines)

**Phase 5: Move streaming infrastructure**
- Stream loop (1,042 lines) -- requires abstracting sidecar communication
- Tool executor (1,140 lines) -- requires abstracting enforcement hooks
- These are the most tightly coupled to Tauri app state.

**Phase 6: Absorb or re-export existing crates**
- `orqa-validation` becomes `orqa_engine::graph` + `orqa_engine::validation`
- `orqa-search` becomes `orqa_engine::search`
- MCP and LSP servers depend on `orqa-engine` instead of individual crates

### Interface Between Engine and App

After extraction, the Tauri app's relationship to the engine is:

```
App calls engine functions directly (same process, library dependency)
  OR
App calls engine via daemon (separate process, HTTP/IPC)
```

ARCHITECTURE.md says the engine is "an independent process." This suggests the daemon model: the engine runs as a background process, and all consumers communicate with it over IPC. The current `orqa-validation` and `orqa-search` crates already have `bin/server.rs` daemon entry points, supporting this direction.

The Tauri app would then:
1. Start the engine daemon on app launch
2. Make requests to the engine daemon for all business logic
3. Own only UI state (SQLite repos for sessions, messages, settings) and sidecar management

This matches how `orqa-mcp-server` and `orqa-lsp-server` already work -- they are separate binaries that could consume the engine daemon.

---

## Recommendations

### Immediate (pre-extraction, low effort)

1. **Fix HIGH frontend hardcoding** -- `MarkdownLink.svelte` artifact ID regex and `StatusBar.svelte` sidecar plugin name. These will break when new plugins are installed.
2. **Deduplicate model options** -- Consolidate the three copies of the model list into one source.

### Short-term (engine extraction Phase 1-2)

3. **Create `libs/engine/` crate** and begin moving type definitions and pure business logic.
4. **Establish engine public API** as the single interface all consumers use.
5. **Fix MEDIUM frontend hardcoding** -- field rendering hints, status stages, and lifecycle stages should come from the engine's composed schema.

### Medium-term (engine extraction Phase 3-5)

6. **Move I/O-dependent logic** into the engine crate.
7. **Abstract sidecar communication** so the stream loop and tool executor can live in the engine.
8. **Move plugin system** into the engine (ARCHITECTURE.md lists it as an engine capability).

### Long-term (engine independence)

9. **Unify daemon** -- single `orqa-engine` daemon process that MCP, LSP, CLI, and app all consume.
10. **Make the app a pure UI shell** -- owns only rendering, SQLite session/message history, and sidecar management.

---

## Open Questions

1. **Stream loop abstraction**: The stream loop (`domain/stream_loop.rs`, 1,042 lines) is deeply coupled to the sidecar protocol and Tauri event emission. Moving it to the engine requires abstracting both the sidecar communication and the event delivery mechanism (traits or channels). This is the hardest extraction challenge and may require significant refactoring.

2. **Daemon vs library**: ARCHITECTURE.md says "independent process," but the simplest extraction path is a library crate that consumers link against. A daemon adds IPC overhead and operational complexity. The decision impacts Phase 6 significantly. Both `orqa-validation` and `orqa-search` already support both modes (lib + bin), suggesting the engine should too.

3. **SQLite split**: Some repositories straddle the boundary. Health snapshots are computed by engine logic (graph health metrics) but stored in app-specific SQLite. The engine may need a storage abstraction (trait-based) so the app can provide its SQLite backend while the CLI uses file-based storage.

4. **Tool executor scope**: The tool executor implements 11 tools (read_file, write_file, bash, etc.) that are specific to the sidecar-driven conversation flow. In the connector model (Claude Code), these tools are provided by the host tool, not the engine. The tool executor may be app-specific rather than engine-level, or it may need to be optional/pluggable.
