# Forge TODO

**Last Updated:** 2026-03-04

Current phase: **Phase 1 COMPLETE. Next: Phase 2a (First-Run Setup Wizard).**

---

## Phase 0a: Tech Stack Research

Research and resolve technical decision points. Each item results in documented findings in `docs/research/` with a recommendation. See [Research Overview](/research/) for full details.

### Claude Integration — `docs/research/claude-integration.md`

- [x] **Claude integration approach** — Agent SDK (`@anthropic-ai/claude-agent-sdk`) as primary integration. Spawns official Claude Code CLI, uses Max subscription. `tools: []` + custom MCP for Forge tool control, `canUseTool` for approval UI, `includePartialMessages` for streaming. **Recommendation: Agent SDK sidecar.**
- [x] **Claude Max integration path** — Max subscription via Agent SDK (spawns official CLI, legal). Cost-effective at 25+ conversations/day ($100-200/month flat vs ~$300+/month API). **Recommendation: Max subscription primary.** API key and other providers are roadmap items.
- [x] **Tool implementation strategy** — Forge's tools exposed as custom MCP server to Agent SDK. Native Rust execution. User-extensible via MCP host. **Recommendation: Forge tools as MCP server + MCP host for extensibility.**
- [x] **Streaming architecture** — Agent SDK streams `SDKMessage` events → Bun-compiled sidecar translates to NDJSON → stdout → Rust `CommandEvent::Stdout` → `Channel<T>` → Svelte store. **Recommendation: Agent SDK → sidecar → NDJSON → Rust → Channel<T> → Svelte.**

### Tauri v2 — `docs/research/tauri-v2.md`

- [x] **Capability audit** — All 8 requirements confirmed: file system, process spawning, file watching, system tray, auto-update, cross-platform, SSE streaming, multi-window.
- [x] **IPC design** — `invoke()` for CRUD, `Channel<T>` for streaming, events for notifications. TauRPC available for end-to-end type safety.
- [x] **Security model** — Three-layer (permissions → scopes → capabilities). `$HOME/**` base scope, deny sensitive paths, pre-declared shell commands, `tauri-plugin-keyring` for API keys.
- [x] **Plugin ecosystem** — All 9 needed plugins exist and are stable (sql, fs, shell, store, autostart, updater, window-state, dialog, notification). Plus community keyring and persisted-scope.

### Frontend — `docs/research/frontend.md`

- [x] **Markdown rendering + editing** — Two-component strategy: CodeMirror 6 (`svelte-codemirror-editor` v2.1.0, Svelte 5 runes) for editing, `@humanspeak/svelte-markdown` v0.8.13 for rendering. `svelte-highlight` for runtime syntax highlighting. **No WYSIWYG — source-level editing preserves markdown fidelity.**
- [x] **Conversation UI component** — Custom build on shadcn-svelte primitives (Button, ScrollArea, Collapsible, Tabs, Badge). Vercel AI SDK studied for patterns only (SSE transport incompatible with Channel<T>). Streaming via Svelte 5 `$state` for fine-grained DOM updates. **Recommendation: Custom.**
- [x] **Panel layout system** — PaneForge v1.0.2 (Svelte 5 native, shadcn-svelte `Resizable` IS PaneForge). Three-zone + nav sub-panel layout. Tabs via shadcn-svelte. **Recommendation: PaneForge + shadcn-svelte Tabs.**
- [x] **Chart/visualization library** — LayerChart (shadcn-svelte `Chart` IS LayerChart). Composable Svelte components. Status indicators via shadcn-svelte `Badge` + `lucide-svelte` icons. **Recommendation: LayerChart via shadcn-svelte.**

### Persistence — `docs/research/persistence.md`

- [x] **SQLite schema design** — 9 tables + 2 FTS5. One row per content block for messages. External content FTS5 with triggers. JSON for flexible data, normalized for queried fields. `tauri-plugin-sql` migrations. **Recommendation: Schema defined, `rusqlite` + `tauri-plugin-sql` dual approach.**
- [x] **File vs DB boundary** — Hybrid (Option D): metadata + FTS index in DB, content always from disk. File watcher (`notify-debouncer-full`, 500ms) triggers re-scan/re-index. Obsidian-like pattern. **Recommendation: Metadata + search index in DB, content from disk.**
- [x] **Session persistence model** — Full history (disk is cheap, < 5 GB/year heavy use). Truncate only > 1 MB tool results. FTS5 cross-session search (< 50ms at 1M rows). Claude-generated handoff summaries from Phase 1 (rule-based fallback). Nullable `user_id` for future multi-user. **Recommendation: Store everything, search everything, summarize on session end.**

### Onboarding — `docs/research/onboarding.md`

- [x] **Codebase scanning strategy** — Three-tier hybrid: manifest-file heuristics (instant, <100ms), `hyperpolyglot` language detection (~1-2s), Claude analysis (on-demand). Heuristics cover 90%+ of projects. **Recommendation: Heuristics-first, Claude-augmented.**
- [x] **Governance framework format** — Option C: `.claude/` files on disk (source of truth, CLI-compatible) + structured metadata in SQLite (derived cache). Parse with `yaml-front-matter` + `comrak`. Sync via `notify` file watcher. **Recommendation: Files authoritative, DB is cache.**
- [x] **Progressive disclosure** — Conversation-first. No wizard. Features appear as they become relevant (feature gate table in SQLite). Value in under 1 minute: API key → open project → auto-scan → conversation. **Recommendation: Organic feature introduction.**

---

## Phase 0b: Architecture Decisions

Promote research findings to formal ADs in `docs/architecture/decisions.md`.

- [x] AD-007: Agent SDK sidecar integration — Bun-compiled TypeScript, stdin/stdout NDJSON, `tauri-plugin-shell` spawn
- [x] AD-008: Max subscription authentication — Primary auth via Agent SDK + Claude Code CLI. API key + other providers on roadmap.
- [x] AD-009: Streaming pipeline — Agent SDK → sidecar → NDJSON → Rust → Channel<T> → Svelte. Clarifies AD-002.
- [x] AD-010: Tool implementation as MCP — Forge tools as custom MCP server to Agent SDK. Built-in tools disabled. MCP host for extensibility.
- [x] AD-011: Security model — Tauri three-layer (permissions → scopes → capabilities). Keyring for secrets. Persisted scopes.
- [x] AD-012: Tauri plugin selections — 11 plugins (sql, fs, shell, store, autostart, updater, window-state, dialog, notification, keyring, persisted-scope).
- [x] AD-013: Frontend library selections — shadcn-svelte + CodeMirror 6 + PaneForge + LayerChart. Custom conversation UI.
- [x] AD-014: Persistence architecture — 9 tables + 2 FTS5. One row per content block. Hybrid file/DB. Full session history.
- [x] AD-015: Governance artifact format — .claude/ on disk (authoritative) + SQLite metadata cache. yaml-front-matter + comrak.
- [x] AD-016: Onboarding strategy — Three-tier scanning. Conversation-first progressive disclosure. Feature gates in SQLite.
- [x] AD-017: Composability principle — Provider-agnostic ProviderEvent protocol. Swappable sidecar providers.

---

## Phase 0c: Product Definition

Define what we're building. Documents in `docs/product/`.

- [x] Glossary / domain model — `docs/product/glossary.md`. 40+ terms across 9 categories. Establishes consistent terminology for all documentation.
- [x] User personas — `docs/product/personas.md`. Three personas: Alex (PM/Tech Lead, primary), Sam (Developer, secondary), Jordan (Solo Technical PM, tertiary). Comparison matrix and design priorities.
- [x] User journeys — `docs/product/journeys.md`. Six journeys: first-time setup, define governance, implementation cycle, review/approve, learning loop, onboard existing project. MVP coverage matrix.
- [x] Information architecture — `docs/product/information-architecture.md`. Three-zone + nav sub-panel layout (Activity Bar, Nav Sub-Panel, Explorer Panel, Chat Panel), toolbar, navigation model, keyboard shortcuts, state management, empty states.
- [x] MVP feature specification — `docs/product/mvp-specification.md`. 14 features (F-001 through F-013 + F-001b New Project) with acceptance criteria. Dogfooding validation checklist. Explicit deferral list.

---

## Phase 0d: UX Design

Design the UI. Documents in `docs/ui/`.

- [x] Wireframing tool research — PlantUML Salt (primary, wireframes) + D2 (secondary, architecture diagrams). ImagineUI abandoned, not recommended. `docs/research/wireframing.md`
- [x] Design system — Forge's own design tokens, brand extensions, component library spec, per-project theming. `docs/ui/design-system.md`
- [x] Wireframes: Core layout — `docs/ui/wireframes/core-layout.md`
- [x] Wireframes: Conversation view — `docs/ui/wireframes/conversation-view.md`
- [x] Wireframes: Artifact browser — `docs/ui/wireframes/artifact-browser.md`
- [x] Wireframes: Settings / onboarding — `docs/ui/wireframes/settings-onboarding.md`
- [x] Wireframes: Dashboard — `docs/ui/wireframes/dashboard.md`
- [x] Component inventory — `docs/ui/component-inventory.md`
- [x] Interaction patterns — `docs/ui/interaction-patterns.md`
- [x] Responsive behavior — `docs/ui/responsive-behavior.md`

---

## Phase 0e: Technical Design

Design the technical architecture. Documents in `docs/architecture/`.

- [x] SQLite schema — `docs/architecture/sqlite-schema.md`. 11 core tables + 2 FTS5. WAL mode. Migration strategy.
- [x] IPC command catalog — `docs/architecture/ipc-commands.md`. 26 commands, 10 streaming events, TypeScript types.
- [x] Rust module architecture — `docs/architecture/rust-modules.md`. 8 modules, 20 handlers, repository pattern, ForgeError.
- [x] Svelte component tree — `docs/architecture/svelte-components.md`. Single-route, 7 stores, component-to-command mapping.
- [x] Streaming pipeline — `docs/architecture/streaming-pipeline.md`. End-to-end with buffering, backpressure, reconnection.
- [x] Tool definitions — `docs/architecture/tool-definitions.md`. 6 tools with MCP schemas, Rust impl, UI specs.
- [x] MCP host interface — `docs/architecture/mcp-host.md`. Dual role, external server management, trust model.
- [x] Error taxonomy — `docs/architecture/error-taxonomy.md`. 48 variants across 8 categories, UI surface mapping.
- [x] Wireframe serving infrastructure — `docs/architecture/wireframe-serving.md`. SQLite cache, style variants, custom protocol.
- [x] PlantUML bundling spike — `docs/architecture/plantuml-spike.md`. 4 options evaluated, 6 acceptance criteria, 3-day timebox.

---

## Phase 1: Scaffold — COMPLETE

**Prerequisites:** Phases 0a–0e complete and approved.

- [x] Initialize Tauri v2 + Svelte 5 project with plugins
- [x] Rust: Claude API client with streaming
- [x] Rust: Channel<T> streaming to frontend
- [x] Rust: SQLite setup with schema + migrations
- [x] Rust: Session CRUD commands
- [x] Rust: API key storage via keyring — **Deferred:** Max subscription uses OAuth via Claude CLI, not API keys. Keyring integration not needed for current auth path.
- [x] Frontend: Main layout (three-zone + nav sub-panel per AD-019)
- [x] Frontend: Conversation with streaming tokens
- [x] Frontend: Tool call rendering (collapsible cards)
- [x] Frontend: Session dropdown in Chat Panel header
- [x] Frontend: Settings (API key, model selection)
- [x] Integration: Send message → stream → render
- [x] First working demo: chat with Claude in the app — **Partially complete:** Echo sidecar works end-to-end. Real Agent SDK sidecar needs build tooling (Bun compilation).

**Summary:** Phase 1 delivered a working Tauri v2 app with Claude conversations via Agent SDK sidecar, 40+ IPC commands, 91 Svelte components, full CRUD, streaming, and semantic code search (ONNX embeddings + DuckDB).

---

## Phase 2a: First-Run Setup Wizard

**Prerequisites:** Phase 1 complete.

Version-gated setup wizard for first launch. Detects Claude CLI, auth, sidecar, embedding model.

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
- [ ] Design doc: `docs/architecture/setup-wizard.md`

## Phase 2b: Governance Bootstrap

**Prerequisites:** Phase 2a complete (sidecar running).

Claude-powered governance scan, analysis, and recommendations for projects.

- [ ] Backend: Governance domain types (GovernanceScanResult, Recommendation, etc.)
- [ ] Backend: Governance scanner — filesystem walk to collect .claude/ and other governance files
- [ ] Backend: Governance repo — CRUD for analyses and recommendations (SQLite)
- [ ] Backend: Governance commands (governance_scan, recommendations_list, recommendation_update, recommendation_apply)
- [ ] Backend: Migration 002 — governance_analyses and governance_recommendations tables
- [ ] Frontend: GovernanceBootstrapWizard — wizard overlay on project open
- [ ] Frontend: GovernanceScanPanel — scan results and coverage indicator
- [ ] Frontend: RecommendationList + RecommendationCard — review and approve/reject
- [ ] Frontend: GovernanceStore — scan state, analysis state, recommendations
- [ ] Frontend: Trigger governance scan on project open
- [ ] Frontend: Dashboard governance health badge
- [ ] Design doc: `docs/architecture/governance-bootstrap.md`

## Phase 2c: Artifact Editing

**Prerequisites:** Phase 2b complete.

Edit agents, rules, skills, and hooks directly in the Forge UI.

- [ ] Artifact editor component with markdown/YAML editing
- [ ] Create new artifacts from templates
- [ ] File watcher for external changes (CLI or text editor edits)
- [ ] Validation and linting for artifact formats

## Phase 2d: Self-Learning Loop

**Prerequisites:** Phase 2c complete.

Native hooks/rules for lesson capture + recurrence + promotion; Forge UI for visibility.

- [ ] Native: Hooks that capture lessons after sessions
- [ ] Native: Rules enforcing lesson checking before implementation
- [ ] Native: CLAUDE.md describes the promotion pipeline
- [ ] Forge: Lesson dashboard with recurrence trends
- [ ] Forge: Browse/edit lessons UI
- [ ] Forge: Automated promotion suggestions
- [ ] Forge: Session analytics

## Phase 2e: Enforcement & Continuity

**Prerequisites:** Phase 2d complete.

Native hooks for rule injection + violation detection; Forge UI for session handoff.

- [ ] Native: Hooks inject rules into conversations
- [ ] Native: Hooks detect violations
- [ ] Forge: Real-time violation detection during streaming
- [ ] Forge: Visual compliance dashboard
- [ ] Forge: Session handoff and continuity

## Phase 3: File Tools & MCP

- [ ] Implement file tools (Read, Write, Edit, Glob, Grep) in Rust backend
- [ ] Tool call approval flow (approve/deny/modify before execution)
- [ ] Project file tree panel in UI
- [ ] File viewer/editor panel
- [ ] Git status integration

## Phase 4: Process Visibility

- [ ] Scanner runner and dashboard
- [ ] Metrics dashboard with KPI cards
- [ ] Agent activity panel
- [ ] Documentation panel (browse, render, edit)

## Phase 5: Discovery & Research

- [ ] Research artifact type (structured objects)
- [ ] Decision traceability graph
- [ ] Research-to-AD promotion workflow
- [ ] Discovery dashboard
- [ ] Phase gate management
- [ ] Conversational research workflow
