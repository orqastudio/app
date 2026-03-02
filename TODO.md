# Forge TODO

**Last Updated:** 2026-03-02

Current phase: **0e (Technical Design) — COMPLETE.** Phases 0a–0e all done. Ready for Phase 1 (Scaffold) pending review and approval.

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

## Phase 1: Scaffold

**Prerequisites:** Phases 0a–0e complete and approved.

- [ ] Initialize Tauri v2 + Svelte 5 project with plugins
- [ ] Rust: Claude API client with streaming
- [ ] Rust: Channel<T> streaming to frontend
- [ ] Rust: SQLite setup with schema + migrations
- [ ] Rust: Session CRUD commands
- [ ] Rust: API key storage via keyring
- [ ] Frontend: Main layout (three-zone + nav sub-panel per AD-019)
- [ ] Frontend: Conversation with streaming tokens
- [ ] Frontend: Tool call rendering (collapsible cards)
- [ ] Frontend: Session dropdown in Chat Panel header
- [ ] Frontend: Settings (API key, model selection)
- [ ] Integration: Send message → stream → render
- [ ] First working demo: chat with Claude in the app

---

## Phase 1.5: Discovery & Research Process Management

**Prerequisites:** Dogfood Milestone complete.

First post-MVP phase. Makes the discovery process a first-class managed artifact within Forge, providing tooling to aid dogfooding of all subsequent phases. See `docs/product/roadmap.md` for details.

- [ ] Research artifact type (structured objects, not just markdown)
- [ ] Decision traceability graph (research → AD → feature → implementation)
- [ ] Research-to-AD promotion workflow
- [ ] Discovery dashboard (open questions, pending decisions, readiness)
- [ ] Phase gate management
- [ ] Conversational research workflow
- [ ] Template-driven discovery

---

## Phase 2–5

See `docs/product/roadmap.md` for Phases 1.5–5 (Discovery & Research, File System Integration, Process Layer, Governance Backfill, Learning Loops). These phases are defined at a high level and will be refined as earlier phases complete.
