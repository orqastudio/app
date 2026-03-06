---
title: "Roadmap"
category: product
tags: []
created: 2026-03-02
updated: 2026-03-06
---

# Roadmap

**Date:** 2026-03-06

Feature ideas and future work. Remove entries once implemented.

The roadmap enforces a strict **define before build** sequence. Phases 0a through 0e must be completed before any application code is written. This is a product management tool — it should be built like one.

---

## Phase 0a: Tech Stack Research

Research and resolve technical decision points. Each item results in documented findings in `.orqa/research/` with a recommendation.

**Claude Integration** — [`.orqa/research/mvp/claude-integration.md`](/research/mvp/claude-integration)

- [x] Claude integration approach → Agent SDK as primary (spawns official CLI, Max subscription). `tools: []` + custom MCP for control.
- [x] Claude Max integration path → Max subscription via Agent SDK. Cost-effective at 25+ conv/day. API key + other providers on roadmap.
- [x] Tool implementation strategy → Orqa Studio tools as custom MCP server exposed to Agent SDK. Native Rust execution. MCP host for extensibility.
- [x] Streaming architecture → Agent SDK → sidecar (Bun-compiled) → NDJSON stdout → Rust → Channel<T> → Svelte store.

**Tauri v2** — [`.orqa/research/mvp/tauri-v2.md`](/research/mvp/tauri-v2)

- [x] Tauri v2 capability audit → All 8 requirements confirmed supported
- [x] IPC design → invoke() for CRUD, Channel<T> for streaming, events for notifications
- [x] Security model → Scoped capabilities, keyring for API keys, persisted scopes
- [x] Plugin ecosystem → All 9 needed plugins exist and are stable

**Frontend** — [`.orqa/research/mvp/frontend.md`](/research/mvp/frontend)

- [x] Markdown rendering + editing → CodeMirror 6 (editing) + @humanspeak/svelte-markdown (rendering). No WYSIWYG.
- [x] Conversation UI component → Custom build on shadcn-svelte. Vercel AI SDK for patterns only.
- [x] Panel layout system → PaneForge (shadcn-svelte Resizable). Three-zone + nav sub-panel layout.
- [x] Chart/visualization library → LayerChart (shadcn-svelte Chart). Badge + lucide for indicators.

**Persistence** — [`.orqa/research/mvp/persistence.md`](/research/mvp/persistence)

- [x] SQLite schema design → 9 tables + 2 FTS5. One row per content block. rusqlite + tauri-plugin-sql.
- [x] File vs DB boundary → Hybrid: metadata + FTS in DB, content from disk. notify file watcher.
- [x] Session persistence model → Full history (<5 GB/year). FTS5 cross-session search. Rule-based handoff.

**Onboarding** — [`.orqa/research/mvp/onboarding.md`](/research/mvp/onboarding)

- [x] Codebase scanning strategy → Three-tier hybrid: manifest heuristics + hyperpolyglot + Claude on-demand.
- [x] Governance framework format → .claude/ on disk (authoritative) + SQLite metadata (derived cache).
- [x] Progressive disclosure → Conversation-first. Feature gates in SQLite. Value in <1 minute.

## Phase 0b: Architecture Decisions

Promote research findings to formal Architecture Decisions in [`docs/architecture/decisions.md`](/architecture/decisions). Each AD is immutable once recorded. Research origin noted for traceability.

- [x] AD-007: Agent SDK sidecar integration — Bun-compiled TypeScript, stdin/stdout NDJSON, `tauri-plugin-shell` spawn. ← [Claude Integration](/research/mvp/claude-integration)
- [x] AD-008: Max subscription authentication — Primary auth via Agent SDK + Claude Code CLI. API key + other providers on roadmap. ← [Claude Integration](/research/mvp/claude-integration)
- [x] AD-009: Streaming pipeline — Agent SDK → sidecar → NDJSON → Rust → Channel<T> → Svelte. Clarifies AD-002. ← [Claude Integration](/research/mvp/claude-integration)
- [x] AD-010: Tool implementation as MCP — Orqa Studio tools as custom MCP server to Agent SDK. Built-in tools disabled. MCP host for extensibility. ← [Claude Integration](/research/mvp/claude-integration)
- [x] AD-011: Security model — Tauri three-layer (permissions → scopes → capabilities). Keyring for secrets. Persisted scopes. ← [Tauri v2](/research/mvp/tauri-v2)
- [x] AD-012: Tauri plugin selections — 11 plugins (sql, fs, shell, store, autostart, updater, window-state, dialog, notification, keyring, persisted-scope). ← [Tauri v2](/research/mvp/tauri-v2)
- [x] AD-013: Frontend library selections — shadcn-svelte + CodeMirror 6 + PaneForge + LayerChart. Custom conversation UI. ← [Frontend](/research/mvp/frontend)
- [x] AD-014: Persistence architecture — 9 tables + 2 FTS5. One row per content block. Hybrid file/DB. Full session history. ← [Persistence](/research/mvp/persistence)
- [x] AD-015: Governance artifact format — .claude/ on disk (authoritative) + SQLite metadata cache. yaml-front-matter + comrak. ← [Onboarding](/research/mvp/onboarding)
- [x] AD-016: Onboarding strategy — Three-tier scanning. Conversation-first progressive disclosure. Feature gates in SQLite. ← [Onboarding](/research/mvp/onboarding)
- [x] AD-017: Composability principle — Provider-agnostic ProviderEvent protocol. Swappable sidecar providers. ← [Claude Integration](/research/mvp/claude-integration)

## Phase 0c: Product Definition

Define what we're building before designing how it looks. These documents live in `docs/product/`.

- [x] **Glossary / domain model** — 40+ terms across 9 categories. Canonical definitions for all product documentation. [`docs/product/glossary.md`](/product/glossary)
- [x] **User personas** — Three personas: Alex (PM/Tech Lead, primary), Sam (Developer, secondary), Jordan (Solo Technical PM, tertiary). Goals, pain points, workflows, design implications. Comparison matrix. [`docs/product/personas.md`](/product/personas)
- [x] **User journeys** — Six end-to-end workflows: first-time setup, define governance, implementation cycle, review/approve, learning loop, onboard existing project. MVP coverage matrix. [`docs/product/journeys.md`](/product/journeys)
- [x] **Information architecture** — Three-zone + nav sub-panel layout (Activity Bar, Nav Sub-Panel, Explorer, Chat). Navigation model, keyboard shortcuts, state management, empty states. Phase 1 scope defined. [`docs/product/information-architecture.md`](/product/information-architecture)
- [x] **MVP feature specification** — 14 features (F-001 through F-013 + F-001b New Project) with acceptance criteria. Includes Claude-generated handoff summaries (F-013) and New Project workflow (F-001b) in Phase 1. Dogfooding validation checklist. Explicit deferral list with rationale and target phase. [`docs/product/mvp-specification.md`](/product/mvp-specification)

## Phase 0d: UX Design

Design the user interface before building it. These documents live in `docs/ui/`.

- [x] **Wireframing tool research** — PlantUML Salt (primary, wireframes) + D2 (secondary, architecture diagrams). ImagineUI abandoned and not recommended. [`.orqa/research/mvp/wireframing.md`](/research/mvp/wireframing)
- [x] **Design system** — Orqa Studio's own design tokens (colors, typography, spacing, dark/light mode). Per-project theming via extracted design tokens. Brand extension variables. Component library specification. [`docs/ui/design-system.md`](/ui/design-system) ← research: [Design Tokens](/research/mvp/design-tokens), [Branding](/research/mvp/branding), [Brand Identity](/ui/brand-identity)
- [x] **Wireframes: Core layout** — Three-zone + nav sub-panel layout (Activity Bar, Nav Sub-Panel, Explorer, Chat) with toolbar and status bar. Default and Nav Sub-Panel-collapsed states. Zone dimensions and collapse behavior. [`docs/wireframes/core-layout.md`](/wireframes/core-layout) ← informed by: [Information Architecture](/product/information-architecture), [Wireframing](/research/mvp/wireframing)
- [x] **Wireframes: Conversation view** — Active conversation, streaming state, empty/welcome state, error states. Tool call cards collapsed and expanded. All tool types represented. [`docs/wireframes/conversation-view.md`](/wireframes/conversation-view) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/mvp/frontend), [MVP Spec F-003, F-004](/product/mvp-specification)
- [x] **Wireframes: Artifact browser** — Explorer Panel browser with Activity Bar category selection, artifact viewer (rendered), editor (source), empty states. Path scope display for rules. [`docs/wireframes/artifact-browser.md`](/wireframes/artifact-browser) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/mvp/frontend), [MVP Spec F-007, F-008](/product/mvp-specification)
- [x] **Wireframes: Settings / onboarding** — Settings panel (provider, project, appearance, shortcuts). First-run welcome, CLI setup, project open with scan results, new project governance scaffolding. [`docs/wireframes/settings-onboarding.md`](/wireframes/settings-onboarding) ← informed by: [Onboarding](/research/mvp/onboarding), [MVP Spec F-001, F-001b, F-009](/product/mvp-specification)
- [x] **Wireframes: Dashboard** — Scanner dashboard with violation details (Phase 3), metrics dashboard with KPI cards (Phase 5), learning loop IMPL/RETRO cards with promotion workflow (Phase 5). Designed early to validate info architecture. [`docs/wireframes/dashboard.md`](/wireframes/dashboard) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/mvp/frontend)
- [x] **Component inventory** — 21 shadcn-svelte library components, 38 custom application components, 4 custom markdown blocks. Phase-tagged. Third-party library mapping. [`docs/ui/component-inventory.md`](/ui/component-inventory) ← informed by: [Frontend](/research/mvp/frontend), [Wireframing](/research/mvp/wireframing)
- [x] **Interaction patterns** — Streaming token display pipeline, tool call approval flow (Phase 1 read-only, Phase 2 interactive), inline editing, panel resize/collapse, keyboard shortcuts, transitions, focus management, loading/error/empty states. [`docs/ui/interaction-patterns.md`](/ui/interaction-patterns) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/mvp/frontend), [Claude Integration](/research/mvp/claude-integration)
- [x] **Responsive behavior** — Panel collapse priority chain, window width ranges (720-1200px+), overlay mode for narrow windows, toolbar/input/status bar adaptations, PaneForge configuration, testing matrix. [`docs/ui/responsive-behavior.md`](/ui/responsive-behavior) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/mvp/frontend)

## Phase 0e: Technical Design

Design the technical architecture before building it. These documents live in `docs/architecture/`.

- [x] **SQLite schema** — 11 core tables + 2 FTS5 virtual tables. WAL mode, foreign keys, busy timeout. Migration strategy via tauri-plugin-sql. Streaming write pattern. Common query patterns. [`docs/architecture/sqlite-schema.md`](/architecture/sqlite-schema) ← research: [Persistence](/research/mvp/persistence), [Design Tokens](/research/mvp/design-tokens)
- [x] **IPC command catalog** — 26 commands across 8 domains (Project, Session, Message, Streaming, Artifact, Theme, Settings, Sidecar). 10 StreamEvent variants. Typed `invoke<T>` wrapper. [`docs/architecture/ipc-commands.md`](/architecture/ipc-commands) ← research: [Tauri v2](/research/mvp/tauri-v2)
- [x] **Rust module architecture** — 8 top-level modules. Domain types matching SQLite schema. 20 command handlers. Repository pattern with New/Update DTOs. SidecarManager lifecycle. Tool trait and ToolRegistry. Orqa StudioError with 15 variants. [`docs/architecture/rust-modules.md`](/architecture/rust-modules) ← research: [Claude Integration](/research/mvp/claude-integration), [Tauri v2](/research/mvp/tauri-v2), [Persistence](/research/mvp/persistence)
- [x] **Svelte component tree** — Single-route architecture with state-driven views. 7 stores as Svelte 5 class-based singletons. Component-to-command mapping. Data flow diagrams. [`docs/architecture/svelte-components.md`](/architecture/svelte-components) ← research: [Frontend](/research/mvp/frontend), [Design Tokens](/research/mvp/design-tokens); product: [Information Architecture](/product/information-architecture)
- [x] **Streaming pipeline** — End-to-end pipeline with latency annotations. 7 NDJSON message types. requestAnimationFrame-based token buffering. StreamBuffer for SQLite writes. Backpressure analysis (~89KB max). Reconnection strategy. [`docs/architecture/streaming-pipeline.md`](/architecture/streaming-pipeline) ← research: [Claude Integration](/research/mvp/claude-integration); AD: [AD-009](/architecture/decisions)
- [x] **Tool definitions** — 6 tools (Read, Write, Edit, Bash, Glob, Grep) with MCP JSON Schema, Rust implementation, parameter schemas, result formats, UI rendering specs, security constraints. Tool approval matrix. [`docs/architecture/tool-definitions.md`](/architecture/tool-definitions) ← research: [Claude Integration](/research/mvp/claude-integration); AD: [AD-010](/architecture/decisions)
- [x] **MCP host interface** — Dual MCP role (server + host). Built-in 6-tool orqa_ namespace. External server discovery and lifecycle. Three trust levels. Tool aggregation with namespacing. [`docs/architecture/mcp-host.md`](/architecture/mcp-host) ← research: [Claude Integration](/research/mvp/claude-integration); AD: [AD-010](/architecture/decisions)
- [x] **Error taxonomy** — 8 sub-enums with 48 total variants. thiserror derivation. IPC serialization. UI surface mapping for all variants. Three recovery tiers. Logging with tracing crate. [`docs/architecture/error-taxonomy.md`](/architecture/error-taxonomy)
- [x] **Wireframe serving infrastructure** — Salt source storage. SQLite wireframe_cache table. Style variants (light/dark/brand). On-demand generation with per-wireframe mutex. Custom protocol handler. PlantUML binary resolution. [`docs/architecture/wireframe-serving.md`](/architecture/wireframe-serving) ← research: [Wireframing](/research/mvp/wireframing), [Design Tokens](/research/mvp/design-tokens)
- [x] **PlantUML bundling spike** — 4 options evaluated: GraalVM native-image (30-40MB), bundled JRE via jlink (38-50MB), system JRE detection, WASM (not ready). Recommendation: try A, fall back to B, always include C. 6 acceptance criteria. 3-day timebox. [`docs/architecture/plantuml-spike.md`](/architecture/plantuml-spike) ← research: [Wireframing](/research/mvp/wireframing)

---

## Phase 1: Scaffold — COMPLETE

**Prerequisites:** Phases 0a–0e complete and approved.

Phase 1 delivered a working Tauri v2 app with Claude conversations via Agent SDK sidecar, 40+ IPC commands, 91 Svelte components, full CRUD, streaming, and semantic code search (ONNX embeddings + DuckDB).

- [x] Initialize Tauri v2 + Svelte 5 project with configured plugins
- [x] Rust backend: Claude API client with streaming (Agent SDK sidecar)
- [x] Rust backend: Channel<T> streaming to frontend
- [x] Rust backend: SQLite setup with initial schema + migrations
- [x] Rust backend: Session CRUD commands
- [x] Rust backend: API key storage via keyring — *Deferred: Max subscription uses OAuth via Claude CLI, not API keys*
- [x] Frontend: Main layout (four-zone layout per AD-018/AD-019)
- [x] Frontend: Conversation component with streaming token display
- [x] Frontend: Tool call rendering (collapsible cards with input/output)
- [x] Frontend: Session dropdown in Chat Panel header
- [x] Frontend: Settings view (provider config, model selection)
- [x] Integration: Send message → stream response → render in UI
- [x] Integration: Basic tool call display (read-only)
- [x] First working demo: chat with Claude in the desktop app — *Partially complete: echo sidecar works E2E, real Agent SDK sidecar needs Bun build*
- [x] Semantic code search: ONNX embeddings + DuckDB vector search + startup model download

---

> **Core Principle: Native Claude Code Artifacts First**
>
> Every capability in Phase 2 is implemented via native `.claude/` artifacts (hooks, rules, skills, CLAUDE.md) first, so it works in the CLI without Orqa Studio. Orqa Studio then adds visual management, dashboards, and enhanced UX on top. This means Orqa Studio is always additive — it never creates vendor lock-in against the Claude Code CLI.

## Phase 2a: First-Run Setup Wizard

**Prerequisites:** Phase 1 complete.

A version-gated setup wizard that runs on first launch. Detects what's configured (Claude CLI, authentication, sidecar, embedding model), skips completed steps, and guides through missing ones. Re-triggers when new requirements are added in future builds by incrementing a setup version constant.

- [ ] Backend: Setup domain types (SetupStatus, SetupStepStatus, ClaudeCliInfo)
- [ ] Backend: Setup commands (check_claude_cli, check_claude_auth, check_embedding_model, get_setup_status, complete_setup)
- [ ] Backend: Version-gated setup check on app launch
- [ ] Frontend: SetupWizard full-screen overlay component
- [ ] Frontend: ClaudeCliStep — CLI detection + install guidance
- [ ] Frontend: ClaudeAuthStep — Auth detection + login flow
- [ ] Frontend: SidecarStep — Sidecar startup with status
- [ ] Frontend: EmbeddingModelStep — Model download with progress
- [ ] Frontend: SetupComplete — Completion confirmation
- [ ] Frontend: SetupStore — step state, detection results, actions
- [ ] Frontend: Mount wizard in AppLayout when setup incomplete
- [ ] Settings: Provider section shows CLI version, auth status, re-auth button
- [ ] Design doc: [`docs/architecture/setup-wizard.md`](/architecture/setup-wizard)

## Phase 2b: Governance Bootstrap — COMPLETE

**Prerequisites:** Phase 2a complete (sidecar running).

When a user opens a project, Claude (via sidecar) scans existing governance files, analyzes them, and generates recommendations. Output is native Claude Code artifacts (`.claude/rules/*.md`, `.claude/hooks/*.sh`, `.claude/agents/*.md`, etc.). Can also translate from other tool formats (Cursor, Copilot, Continue) into Claude Code artifacts.

- [x] Backend: Governance domain types (GovernanceScanResult, Recommendation, etc.)
- [x] Backend: Governance scanner — filesystem walk to collect .claude/ and other governance files
- [x] Backend: Governance repo — CRUD for analyses and recommendations (SQLite)
- [x] Backend: Governance commands (governance_scan, recommendations_list, recommendation_update, recommendation_apply)
- [x] Backend: Migration 002 — governance_analyses and governance_recommendations tables
- [x] Frontend: GovernanceBootstrapWizard — wizard overlay on project open
- [x] Frontend: GovernanceScanPanel — scan results and coverage indicator
- [x] Frontend: RecommendationList + RecommendationCard — review and approve/reject
- [x] Frontend: GovernanceStore — scan state, analysis state, recommendations
- [x] Frontend: Trigger governance scan on project open
- [x] Frontend: Dashboard governance health badge
- [x] Design doc: [`docs/architecture/governance-bootstrap.md`](/architecture/governance-bootstrap)

## Phase 2c: Artifact Editing

**Prerequisites:** Phase 2b complete.

Edit agents, rules, skills, and hooks directly in the Orqa Studio UI. Orqa Studio becomes the primary interface for managing governance artifacts, with file watcher support so CLI and text editor edits are reflected in real time.

- [ ] Artifact editor component with markdown/YAML editing (CodeMirror 6)
- [ ] Create new artifacts from templates
- [ ] File watcher for external changes (CLI or text editor edits)
- [ ] Validation and linting for artifact formats

## Phase 2d: Self-Learning Loop

**Prerequisites:** Phase 2c complete.

Implements the learning loop as native Claude Code hooks and rules first (works in CLI), then adds Orqa Studio-only dashboards for visibility and management.

**Native artifacts:**
- [ ] Hooks that capture lessons after sessions (post-session hook writes to `.orqa/lessons/`)
- [ ] Rules enforcing lesson checking before implementation
- [ ] CLAUDE.md section describing the promotion pipeline (lesson → rule → scanner → enforcement)

**Orqa Studio enhancements:**
- [ ] Lesson dashboard with recurrence trends (LayerChart)
- [ ] Browse/edit lessons UI
- [ ] Automated promotion suggestions (when recurrence >= threshold)
- [ ] Session analytics (pass/fail rates, coverage trends)

## Phase 2e: Enforcement & Continuity

**Prerequisites:** Phase 2d complete.

Implements rule injection and violation detection as native hooks (works in CLI), then adds Orqa Studio-only real-time streaming analysis and session handoff UI.

**Native artifacts:**
- [ ] Hooks that inject relevant rules into conversations based on file context
- [ ] Hooks that detect violations and log them

**Orqa Studio enhancements:**
- [ ] Real-time violation detection during streaming (pattern matching on streamed tokens)
- [ ] Visual compliance dashboard
- [ ] Session handoff and continuity (cross-session search, handoff summaries)

## Phase 2f: Rule Enforcement Engine

**Prerequisites:** Phase 2e complete.

Implements the rule enforcement engine as a first-class Orqa Studio feature. Rule files carry YAML frontmatter declaring enforcement patterns. The engine reads frontmatter, evaluates patterns against agent tool calls, records violations, and surfaces them in the UI. CLI-compatible hookify files are auto-generated from the same frontmatter.

- [ ] Backend: Enforcement domain types (EnforcementEntry, Violation, ViolationVerdict)
- [ ] Backend: Rule frontmatter parser (yaml-front-matter + comrak) — extracts enforcement entries from `.claude/rules/*.md`
- [ ] Backend: Enforcement engine — compile entries, evaluate tool calls, produce block/warn verdicts
- [ ] Backend: Violation persistence — `violations` SQLite table with rule ID, entry ID, matched text, session ID, timestamp
- [ ] Backend: Enforcement commands (list_violations, get_violation, dismiss_violation, list_enforcement_entries)
- [ ] Backend: Hookify generator — derives CLI-compatible `.local.md` files from rule frontmatter
- [ ] Frontend: EnforcementPanel — sidebar showing active rules and violation groups
- [ ] Frontend: ViolationBadge — inline red/amber indicator on ToolCallCard
- [ ] Frontend: GovernanceScanResults — scan output with enforcement coverage summary
- [ ] Frontend: EnforcementStore — violation state, enforcement entry list, actions
- [ ] Design docs: [`docs/architecture/enforcement.md`](/architecture/enforcement), [`docs/ui/enforcement-panel.md`](/ui/enforcement-panel)

## Phase 2g: Lesson Management

**Prerequisites:** Phase 2f complete.

Implements the lesson management UI. Lessons are individual markdown files in `.orqa/lessons/` with YAML frontmatter tracking metadata (category, tags, recurrence, promotion status). SQLite caches metadata for fast queries. The UI shows lessons, tracks recurrence, and highlights promotion candidates.

- [ ] Backend: Lesson domain types (Lesson, LessonMetadata, PromotionCandidate)
- [ ] Backend: Lesson storage — `.orqa/lessons/*.md` files with YAML frontmatter
- [ ] Backend: Lesson metadata cache — `lessons` SQLite table (derived from frontmatter)
- [ ] Backend: Lesson commands (list_lessons, get_lesson, create_lesson, update_lesson, list_promotion_candidates)
- [ ] Backend: Promotion workflow — convert lesson to rule enforcement entry when recurrence threshold met
- [ ] Frontend: LessonList — navigation section with recurrence badges
- [ ] Frontend: LessonViewer — individual lesson display with metadata
- [ ] Frontend: Promotion candidate highlights — surface lessons ready to promote
- [ ] Frontend: LessonStore — lesson state, promotion candidates, actions
- [ ] Design docs: [`docs/architecture/lessons.md`](/architecture/lessons), [`docs/ui/lesson-dashboard.md`](/ui/lesson-dashboard)

## Phase 2h: Sub-Agent Support

**Prerequisites:** Phase 2f complete.

Implements the `spawn_agent` tool so the orchestrator agent can delegate tasks to specialized sub-agents within an Orqa Studio session. The agent registry reads `.claude/agents/*.md`. Sub-agent tool calls are aggregated and not surfaced individually in the conversation view.

- [ ] Backend: Agent registry — reads `.claude/agents/*.md`, indexes capabilities
- [ ] Backend: `spawn_agent` tool — spawns a sub-agent with the given role and instructions
- [ ] Backend: Explore mode — lightweight codebase exploration agent (no tool approval required)
- [ ] Backend: Output aggregation — child tool calls collected, not surfaced individually
- [ ] Backend: Turn limits — configurable max turns per sub-agent invocation
- [ ] Frontend: Sub-agent indicator in conversation view — shows when a sub-agent is active
- [ ] Frontend: Sub-agent result display — summary card with expandable detail
- [ ] Design doc: [`docs/architecture/sub-agents.md`](/architecture/sub-agents)

## Future: CI/CD, Versioning & Auto-Update

Automated build pipeline, semantic versioning with pre-release builds, and configurable in-app auto-update. Uses tauri-plugin-updater with GitHub Releases as the distribution channel.

**CI/CD Pipeline:**
- [ ] GitHub Actions workflow: PR checks (`make check` on all platforms)
- [ ] GitHub Actions workflow: build artifacts on merge to main (pre-release)
- [ ] GitHub Actions workflow: build release artifacts on tag push (stable)
- [ ] Platform matrix: Windows (x64), macOS (x64, arm64), Linux (x64)
- [ ] Artifact signing: Tauri updater signature keys
- [ ] Caching strategy: Rust target, node_modules, bun cache

**Versioning:**
- [ ] Semantic versioning: MAJOR.MINOR.PATCH + pre-release suffix
- [ ] Version source of truth: tauri.conf.json + Cargo.toml + package.json sync
- [ ] Conventional commits + auto-changelog generation
- [ ] Git tag workflow: v0.1.0 (stable), v0.2.0-beta.1 (pre-release)
- [ ] Version bump automation (make release-patch, release-minor, release-major)

**Auto-Update:**
- [ ] Backend: Update channel domain types (PreRelease, Minor, MajorOnly)
- [ ] Backend: Update check command — compare current version against GitHub Releases
- [ ] Backend: Channel filtering — parse semver, filter by user preference
- [ ] Backend: Update download + install via tauri-plugin-updater
- [ ] Frontend: Update channel selector in Settings (pre-release / minor / major only)
- [ ] Frontend: Update notification badge + changelog preview dialog
- [ ] Frontend: Update progress indicator during download/install
- [ ] SQLite: user_preferences for update_channel, update_check_interval
- [ ] Design doc: docs/architecture/auto-update.md

## Phase 3: File Tools & MCP Host

← research: [Claude Integration](/research/mvp/claude-integration) (tool implementation), [Tauri v2](/research/mvp/tauri-v2) (fs plugin, security scopes); AD: [AD-010](/architecture/decisions), [AD-011](/architecture/decisions); design: [`docs/architecture/mcp-host.md`](/architecture/mcp-host)

**File Tools:**
- [ ] Implement file tools (Read, Write, Edit, Glob, Grep) in Rust backend
- [ ] Tool call approval flow (approve/deny/modify before execution)
- [ ] Project file tree panel in UI
- [ ] File viewer/editor panel (markdown rendering + code highlighting) ← research: [Frontend](/research/mvp/frontend) (CodeMirror, markdown rendering)
- [ ] Git status integration (show modified files, branch info)

**Semantic Search:**

Semantic search is embedded natively in the app (ONNX embeddings + DuckDB in `src-tauri/src/search/`, sidecar exposes `search_regex`, `search_semantic`, `code_research` as built-in tools). ChunkHound via `.mcp.json` is a permanent CLI/dev-tool enhancement — both paths are first-class.

- [ ] Index management UI — trigger re-index, show index status, configure inclusion/exclusion patterns

**MCP Host — External Server Support:**

Orqa Studio connects to user-installed external MCP servers and aggregates their tools for the sidecar. Architecture fully specified in [`docs/architecture/mcp-host.md`](/architecture/mcp-host).

- [ ] Backend: MCP host module (`src-tauri/src/mcp_host/`) — JSON-RPC protocol handler, connection state machine
- [ ] Backend: stdio transport — spawn external MCP server processes via `tauri-plugin-shell`, stdin/stdout JSON-RPC framing, process lifecycle (lazy spawn, crash detection, auto-restart, graceful shutdown)
- [ ] Backend: SSE transport — HTTP client for remote MCP servers, reconnection with exponential backoff, health check pings
- [ ] Backend: Config loader — read/merge `.claude/mcp-servers.json` (project) + `%APPDATA%/orqa-studio/mcp-servers.json` (user), env var expansion, trust level resolution
- [ ] Backend: Tool aggregator — merge built-in + external tool lists, namespace external tools (`mcp__{server}__{tool}`), route tool calls to correct server
- [ ] Backend: Tauri commands — `mcp_server_list`, `mcp_server_add`, `mcp_server_remove`, `mcp_server_test`, `mcp_server_toggle`
- [ ] Sidecar: Accept aggregated tool list from Rust, register external tools with Agent SDK
- [ ] Frontend: MCP Servers section in Settings — server list (name, transport, status, tool count, trust level), enable/disable toggle
- [ ] Frontend: Add Server dialog — transport type, command/URL, env vars, trust level, "Test Connection" button
- [ ] Frontend: Server detail view — tool list, connection log, restart button
- [ ] Frontend: First-use approval dialog for project-level servers
- [ ] Security: Three trust levels (builtin/user/project), env var filtering, project-level approval gating

## Phase 4: Process Visibility

← research: [Onboarding](/research/mvp/onboarding) (governance format), [Frontend](/research/mvp/frontend) (LayerChart for dashboard); AD: [AD-015](/architecture/decisions)

- [ ] Scanner runner and dashboard (pass/fail history, violation details)
- [ ] Metrics dashboard with KPI cards (LayerChart)
- [ ] Agent activity panel (which agent is working, what tools it's using)
- [ ] Agent enforcement dashboard (track agent utilization per session, warn if orchestrator writes code directly, enforce skill loading before task execution)
- [ ] Documentation panel (browse, render, edit project docs)

## Phase 5: Discovery & Research

← product: [Personas](/product/personas) (Alex: PM/Tech Lead), [Journeys](/product/journeys); research: [Persistence](/research/mvp/persistence) (FTS5 search)

Research and discovery as a managed artifact within Orqa Studio, giving the PM persona tooling for the define-before-build workflow.

- [ ] Research artifact type (structured objects, not just markdown — queryable, filterable)
- [ ] Decision traceability graph (research → AD → feature → implementation)
- [ ] Research-to-AD promotion workflow
- [ ] Discovery dashboard (open questions, pending decisions, implementation readiness)
- [ ] Phase gate management (define phases with prerequisites, track completion)
- [ ] Conversational research workflow (Claude-assisted investigation producing structured artifacts)

## Phase 6: Idea & Feedback Capture

← product: [Journeys](/product/journeys) (define governance, learning loop); research: [Persistence](/research/mvp/persistence) (FTS5 search); pillars: Self-Learning Loop, Process Governance

The top of the funnel. Ideas, feedback, and observations captured during conversations or entered manually flow into the artifact pipeline — surfacing as research topics, informing plans, and ultimately driving implementation. Closes the loop between "I noticed something" and "we built something about it."

**Capture:**
- [ ] Idea artifact type in `.orqa/ideas/` — lightweight markdown with YAML frontmatter (source, category, priority, status, linked-session)
- [ ] Quick-capture from conversation — highlight text or use a slash command to create an idea from a conversation insight
- [ ] Quick-capture from anywhere — global shortcut or status bar button to log a thought without leaving the current context
- [ ] Feedback tagging — tag ideas as `enhancement`, `question`, `observation`, `user-feedback`
- [ ] Session-linked ideas — automatically backlink to the session where the idea originated

**Triage & Flow:**
- [ ] Idea inbox — dashboard view showing uncategorised ideas, sortable by recency, source, and priority
- [ ] Idea-to-research promotion — when an idea needs investigation, promote it to a research document in `.orqa/research/` with backlink
- [ ] Idea-to-plan promotion — when an idea is ready for implementation, promote it directly to a plan in `.orqa/plans/` with backlink
- [ ] Duplicate detection — surface similar existing ideas, research, or plans when a new idea is created (FTS5 + semantic search)
- [ ] Idea retirement — archive ideas that are resolved, out of scope, or superseded, with a reason

**Traceability:**
- [ ] Idea frontmatter: `source` (conversation, manual, feedback), `promoted-to` (research ref, plan ref), `session-id`, `tags`
- [ ] Ideas visible in the traceability graph: Idea → Research → Plan → Implementation
- [ ] Idea recurrence tracking — similar ideas from different sessions increase priority automatically

## Phase 7: Implementation Breakdown & Work Management

← product: [Journeys](/product/journeys) (implementation cycle, review/approve); research: [Persistence](/research/mvp/persistence); pillars: Process Governance

The bottom of the funnel. Plans break down into epics, epics into backlog items, backlog items into tasks. Each level is trackable, prioritisable, and assignable. This is the execution layer that turns approved plans into delivered features.

**Hierarchy:**
- [ ] Epic — a major body of work derived from a plan phase. Lives in `.orqa/epics/` with frontmatter linking to the parent plan
- [ ] Backlog Item — a user-facing deliverable within an epic. Sized, prioritised, and acceptance-criteria'd
- [ ] Task — an agent-assignable unit of work within a backlog item. Maps to a worktree branch

**Plan-to-Backlog Flow:**
- [ ] Plan phase breakdown — when a plan is approved, generate epics from its phases with one click
- [ ] Epic-to-backlog breakdown — decompose an epic into sized backlog items with Claude assistance
- [ ] Backlog-to-task breakdown — decompose a backlog item into agent-assignable tasks
- [ ] Traceability: Plan → Epic → Backlog Item → Task → Commit → Verification

**Bug Tracking:**

Bugs are first-class backlog items, not sub-types of ideas or tasks. They are typically single-task items that go straight from report to fix. The key challenge is prioritising bugs against feature work — the prioritisation framework (above) handles this by scoring bugs on the same dimensions as everything else, so a high-impact urgent bug naturally outranks medium-priority features without needing a subjective severity label.

- [ ] Bug artifact type in `.orqa/bugs/` — markdown with YAML frontmatter (status, component, reproduction-steps, linked-session, linked-task, impact, urgency)
- [ ] Bug creation from conversation — when a user or agent discovers a defect, create a bug with session context
- [ ] Bug creation from test failures — failed tests auto-generate bug reports with stack trace and reproduction context
- [ ] Bugs appear in the unified backlog alongside feature items — not in a separate silo
- [ ] Priority derived from the same scoring dimensions as everything else — a high-impact, urgent bug naturally outranks a low-impact feature without needing a separate severity label
- [ ] Bug-to-task — a bug becomes a single task assigned to an agent, with the reproduction steps and linked session as context
- [ ] Bug triage view — filtered view of unresolved bugs sorted by composite priority score, with linked sessions and reproduction steps

**Prioritisation Framework:**

The automated system needs to know where any item — bug, feature, or idea — fits in the roadmap and implementation hierarchy. This requires a user-defined scoring model that captures the project's values and constraints, so the system can rank items consistently without requiring human triage on every decision.

- [ ] Priority dimensions — user configures which dimensions matter and their relative weights. Defaults:
  - **Impact** — how many users/workflows does this affect? How broken is the experience? (1-5)
  - **Effort** — how much work to implement? (1-5, inverted — low effort scores higher)
  - **Urgency** — time sensitivity, deadlines, blocking other work (1-5)
  - **Product Alignment** — does this serve the product vision and pillars? (0=off-roadmap, 3=one pillar, 5=core to both pillars)
- [ ] No separate severity field — severity is an emergent property of the other dimensions. A high-impact, high-urgency bug with strong product alignment IS critical. A low-impact, low-urgency bug IS cosmetic. Explicit severity is a subjective duplicate that creates disagreements; derived priority from objective dimensions does not.
- [ ] Dimension weights — user sets relative importance per dimension (e.g., impact×3, effort×2, urgency×2, alignment×2). Stored in `.orqa/project.json`
- [ ] Composite priority score — weighted sum produces a single comparable number across all item types. A high-impact bug naturally rises above a medium-priority feature without needing a separate severity label
- [ ] Priority bands — score ranges map to named bands (`P0-Critical`, `P1-High`, `P2-Medium`, `P3-Low`, `P4-Backlog`) for human-readable grouping
- [ ] Auto-scoring — when a new item is created, the system suggests dimension scores based on frontmatter fields (tags, pillar, scope, component) and presents the suggested priority for user confirmation
- [ ] Manual override — user can always override the calculated score. Overrides are tracked so the model can learn which dimensions the user values differently than the defaults
- [ ] Priority recalculation — when weights change, all items are re-scored. User reviews items that shifted bands
- [ ] Custom dimensions — users can add project-specific dimensions (e.g., "customer-facing", "security-related", "tech-debt") with their own scales and weights
- [ ] Priority decay — optional: items that sit unworked lose urgency score over time, or gain it (configurable — some items become more urgent the longer they wait, others become less relevant)
- [ ] Priority views — backlog sortable by composite score, filterable by band. "What should I work on next?" answered by the top of the sorted list

**Backlog Management:**
- [ ] Unified backlog view — bugs and feature items together, filterable and sortable by type, epic, status, priority band, assignee
- [ ] Status workflow — `draft` → `ready` → `in-progress` → `review` → `done` (configurable per project)
- [ ] Sprint/iteration planning — optional timeboxing. Drag items into iterations. Burndown visibility
- [ ] Dependency tracking — items can depend on other items, surfacing blocked work

**Agent Integration:**
- [ ] Task-to-agent assignment — assign tasks to specific agents (backend-engineer, frontend-engineer, etc.)
- [ ] Auto-generate worktree branch from task — `git worktree add` with conventional naming
- [ ] Task context injection — when an agent starts a task, the task description, acceptance criteria, and linked plan context are injected into the conversation
- [ ] Task completion detection — agent reports done → triggers verification gates (code-reviewer, qa-tester, ux-reviewer)
- [ ] Task progress tracking — link commits and conversations to tasks for audit trail

**Reporting:**
- [ ] Progress dashboard — plan completion %, epic progress, velocity trends (if iterations enabled)
- [ ] Blocked items view — surface items waiting on dependencies, review, or external input
- [ ] Pillar alignment report — which pillar each in-progress item serves, ensuring balanced investment

## Future: Provider Ecosystem

The provider-agnostic sidecar interface supports additional providers without changing the Rust core or Svelte UI. Each provider is a new implementation behind the same `ProviderEvent` protocol. ← AD: [AD-017](/architecture/decisions); research: [Claude Integration](/research/mvp/claude-integration)

- [ ] API key provider (Anthropic TypeScript SDK — direct HTTP, pay-per-token)
- [ ] Cloud provider routing (Amazon Bedrock, Google Vertex AI, Azure AI Foundry)
- [ ] Alternative model providers (OpenAI, Google Gemini, open-weight models)
- [ ] Local model support (Ollama, llama.cpp — for offline/air-gapped use)

## Future: Project Type System & Tooling Profiles

Orqa Studio's product discipline (define → research → design → build → govern) applies to any kind of project, not just software. The project setup wizard should determine what the end product is and install the appropriate agents, tools, and environment accordingly.

**Project Type Detection & Setup:**
- [ ] Project type selector during setup — Software, Web Application, Documentation, Research, Design, Custom
- [ ] Project type determines which agents are installed (e.g., software projects get `backend-engineer`, `frontend-engineer`, `test-engineer`; non-software projects get domain-appropriate agents)
- [ ] Project type determines which dev tools are installed (e.g., software projects get ChunkHound for semantic code search; others don't)
- [ ] Project type stored in `.orqa/project.json` and drives conditional behavior throughout the app
- [ ] Custom project types — users can define their own type with a custom agent/tool manifest

**Software Project Tooling (default for dogfooding):**
- [ ] ChunkHound / semantic search auto-configured for code-based projects
- [ ] Dev environment toolbar — start/stop local dev server, build, test, lint from the app
- [ ] Terminal integration — embedded terminal panel for running commands without leaving the app
- [ ] Git integration panel — branch management, diff viewer, commit history

**Web Application Tooling:**
- [ ] Embedded browser preview — render the web app output alongside the conversation (iframe or webview)
- [ ] Hot-reload integration — preview updates live as the agent makes changes
- [ ] Responsive preview — toggle viewport sizes to verify responsive behavior
- [ ] Network inspector — monitor API calls made by the previewed app
- [ ] Dogfood detection — when the project being managed IS Orqa Studio itself, preview tooling is disabled (you can't preview yourself inside yourself). Dogfood-specific enhanced caution (no-watch dev server, session state persistence, sidecar self-edit warnings) is implemented as part of the agent governance overhaul — see `docs/process/agent-governance-plan.md` Phase 1

**Asset & Content Tooling:**
- [ ] Asset viewers — image, video, audio, 3D model preview panels based on project type
- [ ] Document preview — PDF, slides, or rich text rendering for documentation projects
- [ ] Markdown preview — live-rendered markdown alongside source for content projects

**Environment Management:**
- [ ] Local dev environment launcher — configure and start Docker containers, local servers, databases from the app
- [ ] Environment profiles — save/restore different environment configurations per project
- [ ] Health checks — verify the local environment is running and accessible before starting a session

## Future: Onboarding Flow Review

Review and polish the end-to-end onboarding experience — from first app launch through project creation/initialization to first conversation. Ensure the flows are intuitive, well-documented, and handle edge cases gracefully.

- [ ] Audit first-run setup wizard — verify each step is necessary, clear, and skippable when already configured
- [ ] New Project flow — "Create From Scratch" should scaffold a sensible project structure (README, .gitignore, .orqa/) with optional templates (e.g., Rust, Node, Python starter)
- [ ] Initialize Existing Folder flow — improve scan results presentation, show what Orqa will add vs. what already exists, preview `.orqa/` directory structure before creation
- [ ] Open Project validation — graceful handling of corrupted `.orqa/`, missing files, version mismatches between app and project config
- [ ] Onboarding analytics — track where users drop off or get stuck (locally, not telemetry)
- [ ] Guided first conversation — after project setup, suggest a first task or provide a walkthrough of the conversation interface
- [ ] Re-onboarding — when the app updates with new features, surface a "What's New" flow highlighting changes relevant to the user's project

## Future: Multi-Window & Multi-Project

Multiple projects open simultaneously, each in its own window with independent state. Requires system tray support, per-window project context, and per-window sidecar management.

- [ ] System tray integration — `tray-icon` Tauri plugin. Close Window minimizes to tray instead of quitting. Tray icon shows running status. Right-click menu for quick access.
- [ ] Multi-window support — Open additional projects in new Tauri windows. Each window has its own project store, conversation state, and sidecar process.
- [ ] Window management — Track open windows and their projects. "Window" menu listing all open project windows. Switch between windows.
- [ ] Per-window sidecar lifecycle — Each window spawns and owns its own sidecar process. Sidecar cleanup on window close.
- [ ] Shared settings — App-level settings (provider config, appearance) shared across windows. Project-level settings isolated per window.

## Future: Multi-User Collaborative Access

A small team (PM, Tech Lead, Developer) sharing a single Orqa Studio instance with visibility into each other's sessions and governance work. Schema includes nullable `user_id` and `last_edited_by` columns from Phase 1 to avoid migration-heavy changes later. ← research: [Persistence](/research/persistence) (multi-user considerations); product: [Personas](/product/personas)

- [ ] User identity and authentication (local accounts or SSO)
- [ ] Session visibility controls (own, team, all)
- [ ] Artifact edit locking or conflict resolution
- [ ] Shared vs. personal settings
- [ ] Server component for concurrent multi-machine access
- [ ] Connection pooling — current `Mutex<Connection>` is fine for single-user but becomes a bottleneck under concurrent access. Swap to `r2d2`/`deadpool-sqlite` pool (2-4 read + 1 write). Code-level refactor, no schema changes.
- [ ] Evaluate SQLite vs. PostgreSQL — SQLite is single-writer; true multi-machine concurrent access may require a server-backed database

## Future: Wireframe Browser & Interactive UX Flows

Wireframes are generated as styled images during Phase 0d and stored in a local image cache. This future work makes them a first-class browsable, interactive artifact within Orqa Studio. ← research: [Wireframing](/research/mvp/wireframing), [Design Tokens](/research/mvp/design-tokens); product: [Journeys](/product/journeys)

- [ ] Wireframe browser view — Browse all wireframes for a project, organized by UX flow / journey
- [ ] Custom markdown block for wireframe images — Renders cached wireframes with style-awareness (serves light/dark/brand variant based on active theme)
- [ ] Interactive UX flow navigation — Custom markdown block that links wireframe images into clickable flows, allowing users to navigate through screens as part of an interactive review process
- [ ] On-demand wireframe regeneration — When design tokens change (new brand colors, theme switch), regenerate wireframe variants from source definitions
- [ ] Wireframe-to-component traceability — Link wireframe screens to the Svelte components that implement them


## Future: Design Tool Integration

Rather than building comprehensive design tooling internally, integrate with 3rd-party design tools where designers already work. This enables a **Designer persona** to complete the end-to-end team (PM/Tech Lead + Developer + Designer) and avoids reinventing design tooling. ← research: [Design Tokens](/research/mvp/design-tokens); product: [Personas](/product/personas)

- [ ] Figma integration — Import design tokens (colors, typography, spacing) from Figma files. Use Figma MCP server for two-way communication. Extract component specifications from Figma designs.
- [ ] Design token sync — Bidirectional sync between project design tokens and external design tools. Changes in Figma propagate to Orqa Studio's per-project theme; governance-defined tokens can be pushed back. ← research: [Design Tokens](/research/mvp/design-tokens)
- [ ] Design-to-wireframe pipeline — Import high-fidelity designs from Figma as wireframe references. Link designs to user journeys and UX flows within Orqa Studio.
- [ ] Code-to-Figma backfill — Analyze existing frontend implementation (components, styles, layout) and generate corresponding Figma components that accurately represent them. Use those components to reconstruct wireframes as Figma designs, enabling teams who prototyped in code to backfill a proper design system. This bridges the gap to Figma's dev tools pipeline for full design-to-development integration. Potentially unique in the market — no existing tool automates code→Figma component generation at this level.
- [ ] Designer persona — Extend the persona model with a Designer role who defines visual standards, reviews UI compliance, and manages the design system through Orqa Studio's governance framework. ← product: [Personas](/product/personas)
- [ ] Replace automated wireframing — If Figma integration matures, automated wireframe generation becomes optional. Designers create wireframes in their native tool; Orqa Studio indexes and organizes them.


## Future: Developer Experience (Dogfooding)

DX improvements discovered while dogfooding Orqa Studio with itself.

- [ ] Build splash window — Small branded Tauri window that appears during `make dev` compilation and disappears once the app opens. Shows concise build status by default with an expandable accordion for detailed build output (cargo compile progress, warnings, etc.). Helps the developer know the app is building rather than staring at a blank screen.
- [ ] Custom system prompt templates — Pre-built prompt templates for common scenarios (dogfooding, greenfield, legacy codebase) that can be selected in project settings.
- [ ] Project-local database — Move SQLite DB from `app_data_dir` to `.orqa/orqa.db` so session history and audit data travels with the project and can be committed to git. Requires `.gitignore` entries for WAL/SHM journal files (`*.db-wal`, `*.db-shm`). Consider whether all tables belong in the portable DB or if some (e.g. app preferences) should remain app-local.

## Phase 2i: Composability Gate — NEXT (blocks dogfooding)

**Prerequisites:** Phase 2h complete. **Blocks:** Dogfooding.

**Plan:** [`.orqa/plans/composability-gate.md`](/.orqa/plans/composability-gate)

Composability is not just a coding standard — it is a platform principle. The app must practice what it preaches before it can credibly enforce composability on projects it manages. This phase audits, refactors, and architecturally hardens the entire stack, then builds the composable learning loop that adapts to any project.

**Phase A: Deep Composability Audit** — Every layer: AI provider integration, Rust backend (every module), frontend stores, components, CSS/design system, configuration. Produces scorecard + prioritized refactoring plan.

**Phase B: Refactoring to Composability:**
- [ ] Extract domain services from `stream_commands.rs` (~1000 lines → ≤200 lines)
- [ ] Decompose `ConversationView.svelte` into composable units
- [ ] Resolve `conversationStore` → `sessionStore` coupling
- [ ] UI component architecture review — shared components used everywhere, variant props, no inline states
- [ ] Provider abstraction hardening — protocol cleaned of Claude-specific leaks, pluggable auth, provider selection in project config

**Phase C: Composable Learning Loop:**
- [ ] Base + Project layering — base skills/agents/rules/scanner ship with app, project-specific layer on top
- [ ] `.orqa/project.json` schema extended for composability config (base_skills, project_skills, base_agents, project_agents, scanner_profiles)
- [ ] Portable `composability` skill (language-agnostic, no `orqa-` prefix) — ships with app for all projects
- [ ] Agents composable per-project — same role adapts to different tech stacks
- [ ] Rules layered — base always apply, project rules add on top
- [ ] Scanner pluggable — base scans + project-defined custom scans

**Phase D: Initialization Composability Assessment:**
- [ ] Governance scan includes composability analysis (function size distribution, coupling, purity, feature isolation)
- [ ] User-facing assessment report with score, strengths, gaps, and options (refactor plan / accept / skip)
- [ ] Greenfield projects get composability by default
- [ ] No-code PM scenario — composability principles applied automatically, user never sees the complexity

**Verification:** Orqa Studio's own composability score ≥85/100. Composability scanner runs on itself as part of `make check`.

## Future: Code Quality Audit

- [ ] Abstraction pattern audit — Review the codebase for areas where iterative development has introduced overcomplicated patterns, duplicate logic, or unnecessary layers. Identify opportunities for reusable abstractions (e.g. the generic `parse_frontmatter<T>` pattern applied to YAML parsing). Produce a refactoring plan with prioritised simplification targets that reduce maintenance burden without losing functionality.
- [ ] Coding standards compliance audit — Systematic review of the entire codebase against `docs/development/coding-standards.md`. Verify all Rust code follows error propagation rules, function size limits, naming conventions, and clippy compliance. Verify all Svelte code uses Svelte 5 runes only, strict TypeScript, component purity, and shared component patterns. Document violations and produce a remediation plan.
- [ ] Enforcement artifact review — Audit existing rules (`.claude/rules/`), hooks (`.claude/hooks/`), and skills (`.claude/skills/`) for completeness and effectiveness. Identify coding standards that lack automated enforcement. Create missing enforcement hooks and rules to close gaps. Ensure pre-commit hooks catch all documented standards.
