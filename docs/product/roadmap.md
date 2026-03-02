# Roadmap

**Date:** 2026-03-02

Feature ideas and future work. Remove entries once implemented.

The roadmap enforces a strict **define before build** sequence. Phases 0a through 0e must be completed before any application code is written. This is a product management tool — it should be built like one.

---

## Phase 0a: Tech Stack Research

Research and resolve technical decision points. Each item results in documented findings in `docs/research/` with a recommendation.

**Claude Integration** — [`docs/research/claude-integration.md`](/research/claude-integration)

- [x] Claude integration approach → Agent SDK as primary (spawns official CLI, Max subscription). `tools: []` + custom MCP for control.
- [x] Claude Max integration path → Max subscription via Agent SDK. Cost-effective at 25+ conv/day. API key + other providers on roadmap.
- [x] Tool implementation strategy → Forge tools as custom MCP server exposed to Agent SDK. Native Rust execution. MCP host for extensibility.
- [x] Streaming architecture → Agent SDK → sidecar (Bun-compiled) → NDJSON stdout → Rust → Channel<T> → Svelte store.

**Tauri v2** — [`docs/research/tauri-v2.md`](/research/tauri-v2)

- [x] Tauri v2 capability audit → All 8 requirements confirmed supported
- [x] IPC design → invoke() for CRUD, Channel<T> for streaming, events for notifications
- [x] Security model → Scoped capabilities, keyring for API keys, persisted scopes
- [x] Plugin ecosystem → All 9 needed plugins exist and are stable

**Frontend** — [`docs/research/frontend.md`](/research/frontend)

- [x] Markdown rendering + editing → CodeMirror 6 (editing) + @humanspeak/svelte-markdown (rendering). No WYSIWYG.
- [x] Conversation UI component → Custom build on shadcn-svelte. Vercel AI SDK for patterns only.
- [x] Panel layout system → PaneForge (shadcn-svelte Resizable). Three-pane layout.
- [x] Chart/visualization library → LayerChart (shadcn-svelte Chart). Badge + lucide for indicators.

**Persistence** — [`docs/research/persistence.md`](/research/persistence)

- [x] SQLite schema design → 9 tables + 2 FTS5. One row per content block. rusqlite + tauri-plugin-sql.
- [x] File vs DB boundary → Hybrid: metadata + FTS in DB, content from disk. notify file watcher.
- [x] Session persistence model → Full history (<5 GB/year). FTS5 cross-session search. Rule-based handoff.

**Onboarding** — [`docs/research/onboarding.md`](/research/onboarding)

- [x] Codebase scanning strategy → Three-tier hybrid: manifest heuristics + hyperpolyglot + Claude on-demand.
- [x] Governance framework format → .claude/ on disk (authoritative) + SQLite metadata (derived cache).
- [x] Progressive disclosure → Conversation-first. Feature gates in SQLite. Value in <1 minute.

## Phase 0b: Architecture Decisions

Promote research findings to formal Architecture Decisions in [`docs/architecture/decisions.md`](/architecture/decisions). Each AD is immutable once recorded. Research origin noted for traceability.

- [x] AD-007: Agent SDK sidecar integration — Bun-compiled TypeScript, stdin/stdout NDJSON, `tauri-plugin-shell` spawn. ← [Claude Integration](/research/claude-integration)
- [x] AD-008: Max subscription authentication — Primary auth via Agent SDK + Claude Code CLI. API key + other providers on roadmap. ← [Claude Integration](/research/claude-integration)
- [x] AD-009: Streaming pipeline — Agent SDK → sidecar → NDJSON → Rust → Channel<T> → Svelte. Clarifies AD-002. ← [Claude Integration](/research/claude-integration)
- [x] AD-010: Tool implementation as MCP — Forge tools as custom MCP server to Agent SDK. Built-in tools disabled. MCP host for extensibility. ← [Claude Integration](/research/claude-integration)
- [x] AD-011: Security model — Tauri three-layer (permissions → scopes → capabilities). Keyring for secrets. Persisted scopes. ← [Tauri v2](/research/tauri-v2)
- [x] AD-012: Tauri plugin selections — 11 plugins (sql, fs, shell, store, autostart, updater, window-state, dialog, notification, keyring, persisted-scope). ← [Tauri v2](/research/tauri-v2)
- [x] AD-013: Frontend library selections — shadcn-svelte + CodeMirror 6 + PaneForge + LayerChart. Custom conversation UI. ← [Frontend](/research/frontend)
- [x] AD-014: Persistence architecture — 9 tables + 2 FTS5. One row per content block. Hybrid file/DB. Full session history. ← [Persistence](/research/persistence)
- [x] AD-015: Governance artifact format — .claude/ on disk (authoritative) + SQLite metadata cache. yaml-front-matter + comrak. ← [Onboarding](/research/onboarding)
- [x] AD-016: Onboarding strategy — Three-tier scanning. Conversation-first progressive disclosure. Feature gates in SQLite. ← [Onboarding](/research/onboarding)
- [x] AD-017: Composability principle — Provider-agnostic ProviderEvent protocol. Swappable sidecar providers. ← [Claude Integration](/research/claude-integration)

## Phase 0c: Product Definition

Define what we're building before designing how it looks. These documents live in `docs/product/`.

- [x] **Glossary / domain model** — 40+ terms across 9 categories. Canonical definitions for all product documentation. [`docs/product/glossary.md`](/product/glossary)
- [x] **User personas** — Three personas: Alex (PM/Tech Lead, primary), Sam (Developer, secondary), Jordan (Solo Technical PM, tertiary). Goals, pain points, workflows, design implications. Comparison matrix. [`docs/product/personas.md`](/product/personas)
- [x] **User journeys** — Six end-to-end workflows: first-time setup, define governance, implementation cycle, review/approve, learning loop, onboard existing project. MVP coverage matrix. [`docs/product/journeys.md`](/product/journeys)
- [x] **Information architecture** — Three-pane layout (sidebar, primary, detail). Navigation model, keyboard shortcuts, state management, empty states. Phase 1 scope defined. [`docs/product/information-architecture.md`](/product/information-architecture)
- [x] **MVP feature specification** — 14 features (F-001 through F-013 + F-001b New Project) with acceptance criteria. Includes Claude-generated handoff summaries (F-013) and New Project workflow (F-001b) in Phase 1. Dogfooding validation checklist. Explicit deferral list with rationale and target phase. [`docs/product/mvp-specification.md`](/product/mvp-specification)

## Phase 0d: UX Design

Design the user interface before building it. These documents live in `docs/ui/`.

- [x] **Wireframing tool research** — PlantUML Salt (primary, wireframes) + D2 (secondary, architecture diagrams). ImagineUI abandoned and not recommended. [`docs/research/wireframing.md`](/research/wireframing)
- [x] **Design system** — Forge's own design tokens (colors, typography, spacing, dark/light mode). Per-project theming via extracted design tokens. Brand extension variables. Component library specification. [`docs/ui/design-system.md`](/ui/design-system) ← research: [Design Tokens](/research/design-tokens), [Branding](/research/branding), [Brand Identity](/ui/brand-identity)
- [x] **Wireframes: Core layout** — Four-zone layout (Activity Bar, Explorer, Sessions, Chat) with toolbar and status bar. Default and Sessions-collapsed states. Zone dimensions and collapse behavior. [`docs/ui/wireframes/core-layout.md`](/ui/wireframes/core-layout) ← informed by: [Information Architecture](/product/information-architecture), [Wireframing](/research/wireframing)
- [x] **Wireframes: Conversation view** — Active conversation, streaming state, empty/welcome state, error states. Tool call cards collapsed and expanded. All tool types represented. [`docs/ui/wireframes/conversation-view.md`](/ui/wireframes/conversation-view) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/frontend), [MVP Spec F-003, F-004](/product/mvp-specification)
- [x] **Wireframes: Artifact browser** — Explorer Panel browser with Activity Bar category selection, artifact viewer (rendered), editor (source), empty states. Path scope display for rules. [`docs/ui/wireframes/artifact-browser.md`](/ui/wireframes/artifact-browser) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/frontend), [MVP Spec F-007, F-008](/product/mvp-specification)
- [x] **Wireframes: Settings / onboarding** — Settings panel (provider, project, appearance, shortcuts). First-run welcome, CLI setup, project open with scan results, new project governance scaffolding. [`docs/ui/wireframes/settings-onboarding.md`](/ui/wireframes/settings-onboarding) ← informed by: [Onboarding](/research/onboarding), [MVP Spec F-001, F-001b, F-009](/product/mvp-specification)
- [x] **Wireframes: Dashboard** — Scanner dashboard with violation details (Phase 3), metrics dashboard with KPI cards (Phase 5), learning loop IMPL/RETRO cards with promotion workflow (Phase 5). Designed early to validate info architecture. [`docs/ui/wireframes/dashboard.md`](/ui/wireframes/dashboard) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/frontend)
- [x] **Component inventory** — 21 shadcn-svelte library components, 38 custom application components, 4 custom markdown blocks. Phase-tagged. Third-party library mapping. [`docs/ui/component-inventory.md`](/ui/component-inventory) ← informed by: [Frontend](/research/frontend), [Wireframing](/research/wireframing)
- [x] **Interaction patterns** — Streaming token display pipeline, tool call approval flow (Phase 1 read-only, Phase 2 interactive), inline editing, panel resize/collapse, keyboard shortcuts, transitions, focus management, loading/error/empty states. [`docs/ui/interaction-patterns.md`](/ui/interaction-patterns) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/frontend), [Claude Integration](/research/claude-integration)
- [x] **Responsive behavior** — Panel collapse priority chain, window width ranges (720-1200px+), overlay mode for narrow windows, toolbar/input/status bar adaptations, PaneForge configuration, testing matrix. [`docs/ui/responsive-behavior.md`](/ui/responsive-behavior) ← informed by: [Information Architecture](/product/information-architecture), [Frontend](/research/frontend)

## Phase 0e: Technical Design

Design the technical architecture before building it. These documents live in `docs/architecture/`.

- [x] **SQLite schema** — 11 core tables + 2 FTS5 virtual tables. WAL mode, foreign keys, busy timeout. Migration strategy via tauri-plugin-sql. Streaming write pattern. Common query patterns. [`docs/architecture/sqlite-schema.md`](/architecture/sqlite-schema) ← research: [Persistence](/research/persistence), [Design Tokens](/research/design-tokens)
- [x] **IPC command catalog** — 26 commands across 8 domains (Project, Session, Message, Streaming, Artifact, Theme, Settings, Sidecar). 10 StreamEvent variants. Typed `forgeInvoke<T>` wrapper. [`docs/architecture/ipc-commands.md`](/architecture/ipc-commands) ← research: [Tauri v2](/research/tauri-v2)
- [x] **Rust module architecture** — 8 top-level modules. Domain types matching SQLite schema. 20 command handlers. Repository pattern with New/Update DTOs. SidecarManager lifecycle. Tool trait and ToolRegistry. ForgeError with 15 variants. [`docs/architecture/rust-modules.md`](/architecture/rust-modules) ← research: [Claude Integration](/research/claude-integration), [Tauri v2](/research/tauri-v2), [Persistence](/research/persistence)
- [x] **Svelte component tree** — Single-route architecture with state-driven views. 7 stores as Svelte 5 class-based singletons. Component-to-command mapping. Data flow diagrams. [`docs/architecture/svelte-components.md`](/architecture/svelte-components) ← research: [Frontend](/research/frontend), [Design Tokens](/research/design-tokens); product: [Information Architecture](/product/information-architecture)
- [x] **Streaming pipeline** — End-to-end pipeline with latency annotations. 7 NDJSON message types. requestAnimationFrame-based token buffering. StreamBuffer for SQLite writes. Backpressure analysis (~89KB max). Reconnection strategy. [`docs/architecture/streaming-pipeline.md`](/architecture/streaming-pipeline) ← research: [Claude Integration](/research/claude-integration); AD: [AD-009](/architecture/decisions)
- [x] **Tool definitions** — 6 tools (Read, Write, Edit, Bash, Glob, Grep) with MCP JSON Schema, Rust implementation, parameter schemas, result formats, UI rendering specs, security constraints. Tool approval matrix. [`docs/architecture/tool-definitions.md`](/architecture/tool-definitions) ← research: [Claude Integration](/research/claude-integration); AD: [AD-010](/architecture/decisions)
- [x] **MCP host interface** — Dual MCP role (server + host). Built-in 6-tool forge_ namespace. External server discovery and lifecycle. Three trust levels. Tool aggregation with namespacing. [`docs/architecture/mcp-host.md`](/architecture/mcp-host) ← research: [Claude Integration](/research/claude-integration); AD: [AD-010](/architecture/decisions)
- [x] **Error taxonomy** — 8 sub-enums with 48 total variants. thiserror derivation. IPC serialization. UI surface mapping for all variants. Three recovery tiers. Logging with tracing crate. [`docs/architecture/error-taxonomy.md`](/architecture/error-taxonomy)
- [x] **Wireframe serving infrastructure** — Salt source storage. SQLite wireframe_cache table. Style variants (light/dark/brand). On-demand generation with per-wireframe mutex. Custom protocol handler. PlantUML binary resolution. [`docs/architecture/wireframe-serving.md`](/architecture/wireframe-serving) ← research: [Wireframing](/research/wireframing), [Design Tokens](/research/design-tokens)
- [x] **PlantUML bundling spike** — 4 options evaluated: GraalVM native-image (30-40MB), bundled JRE via jlink (38-50MB), system JRE detection, WASM (not ready). Recommendation: try A, fall back to B, always include C. 6 acceptance criteria. 3-day timebox. [`docs/architecture/plantuml-spike.md`](/architecture/plantuml-spike) ← research: [Wireframing](/research/wireframing)

---

## Phase 1: Scaffold

**Prerequisites:** Phases 0a–0e complete and approved.

Build the minimum viable application that can run a conversation with Claude.

- [ ] Initialize Tauri v2 + Svelte 5 project with configured plugins ← AD: [AD-012](/architecture/decisions); research: [Tauri v2](/research/tauri-v2)
- [ ] Rust backend: Claude API client with streaming (reqwest + SSE) ← AD: [AD-007, AD-009](/architecture/decisions); research: [Claude Integration](/research/claude-integration)
- [ ] Rust backend: Channel<T> streaming to frontend ← AD: [AD-009](/architecture/decisions); research: [Tauri v2](/research/tauri-v2)
- [ ] Rust backend: SQLite setup with initial schema + migrations ← AD: [AD-014](/architecture/decisions); research: [Persistence](/research/persistence)
- [ ] Rust backend: Session CRUD commands ← AD: [AD-014](/architecture/decisions); research: [Persistence](/research/persistence)
- [ ] Rust backend: API key storage via keyring ← AD: [AD-011](/architecture/decisions); research: [Tauri v2](/research/tauri-v2)
- [ ] Frontend: Main layout (conversation panel + collapsible side panel) ← AD: [AD-013](/architecture/decisions); research: [Frontend](/research/frontend); product: [Information Architecture](/product/information-architecture)
- [ ] Frontend: Conversation component with streaming token display ← AD: [AD-013](/architecture/decisions); research: [Frontend](/research/frontend); spec: [F-003](/product/mvp-specification)
- [ ] Frontend: Tool call rendering (collapsible cards with input/output) ← research: [Frontend](/research/frontend); spec: [F-004](/product/mvp-specification)
- [ ] Frontend: Session history sidebar ← research: [Frontend](/research/frontend); spec: [F-005](/product/mvp-specification); product: [Information Architecture](/product/information-architecture)
- [ ] Frontend: Settings view (API key entry, model selection) ← spec: [F-009](/product/mvp-specification); product: [Information Architecture](/product/information-architecture)
- [ ] Integration: Send message → stream response → render in UI ← spec: [F-003](/product/mvp-specification)
- [ ] Integration: Basic tool call display (read-only, no approval yet) ← spec: [F-004](/product/mvp-specification)
- [ ] First working demo: chat with Claude in the desktop app ← spec: [Dogfooding Validation Checklist](/product/mvp-specification)

## Dogfood Milestone (after Phase 1)

Once Phase 1 delivers a working chat UI with Claude integration, Forge begins managing its own development. This is the transition point from the temporary Alvarez-derived CLI process to Forge-as-primary-tool. ← spec: [Dogfooding Validation Checklist](/product/mvp-specification)

**Minimum viable dogfood:**
- [ ] Can browse and edit process artifacts (agents, rules, skills) in the UI ← spec: [F-007, F-008](/product/mvp-specification)
- [ ] Can run a conversation with Claude and see streaming responses ← spec: [F-003](/product/mvp-specification)
- [ ] Can view session history ← spec: [F-005](/product/mvp-specification)
- [ ] Can capture notes/lessons through the interface

Features that are painful or missing when dogfooding become the highest-priority work items.

## Phase 1.5: Discovery & Research Process Management

**Prerequisites:** Dogfood Milestone complete.

The define-before-build discovery process (research → architecture decisions → product definition → UX design → technical design) is itself a repeatable, structured workflow — but currently lives in flat markdown files. Making the discovery process a first-class managed artifact within Forge gives the PM persona a dedicated UX for driving the end-to-end journey from question to decision to design to implementation. This phase comes immediately after MVP because it provides tooling to aid dogfooding of all subsequent phases — every feature added in Phase 2+ benefits from managed discovery. ← product: [Personas](/product/personas) (Alex: PM/Tech Lead), [Journeys](/product/journeys), [Roadmap](/product/roadmap) (Phase 0a–0e pattern)

**Note:** Current research documents (`docs/research/*.md`) use YAML frontmatter with structured metadata to enable future migration into Forge's research artifact system. See the frontmatter convention in [`docs/research/README.md`](/research/).

- [ ] Research artifact type — First-class `research` artifact alongside agents/rules/skills. Each research doc is a structured object: open questions, findings, verdict, status (open/complete). Not just markdown — queryable, filterable, linkable. Migrate existing `docs/research/*.md` files using their YAML frontmatter.
- [ ] Decision traceability graph — Visual graph showing the flow from research findings → architecture decisions → product features → technical designs → implementation. Navigate upstream ("why was this decided?") and downstream ("what depends on this?").
- [ ] Research-to-AD promotion workflow — When research produces a recommendation, a structured workflow promotes it to a formal architecture decision. Track which research informed which AD, and surface when research is updated after an AD was recorded.
- [ ] Discovery dashboard — For a project, show: open research questions, pending decisions, unresolved design items, implementation readiness. The PM can see at a glance where the process is blocked or incomplete.
- [ ] Phase gate management — Define phases with prerequisites, track completion, enforce sequencing. Automate the "is Phase X complete?" check rather than manually reviewing checklists.
- [ ] Conversational research workflow — Use Claude to conduct research interactively: ask questions, evaluate options, document findings, draft recommendations. The conversation produces structured research artifacts, not just chat history.
- [ ] Template-driven discovery — Project templates that include a discovery phase structure. "Starting a new Tauri app" could include pre-defined research questions (which plugins? which frontend framework? what persistence?) with suggested evaluation criteria.

## Phase 2: File System Integration

← research: [Claude Integration](/research/claude-integration) (tool implementation), [Tauri v2](/research/tauri-v2) (fs plugin, security scopes); AD: [AD-010](/architecture/decisions), [AD-011](/architecture/decisions)

- [ ] Implement file tools (Read, Write, Edit, Glob, Grep) in Rust backend
- [ ] Tool call approval flow (approve/deny/modify before execution)
- [ ] Project file tree panel in UI
- [ ] File viewer/editor panel (markdown rendering + code highlighting) ← research: [Frontend](/research/frontend) (CodeMirror, markdown rendering)
- [ ] Git status integration (show modified files, branch info)

## Phase 3: Process Layer

← research: [Onboarding](/research/onboarding) (governance format), [Frontend](/research/frontend) (LayerChart for dashboard); AD: [AD-015](/architecture/decisions)

- [ ] Documentation panel (browse, render, edit project docs)
- [ ] Scanner runner (execute scanners, parse results)
- [ ] Scanner dashboard (pass/fail history, violation details)
- [ ] Agent activity panel (which agent is working, what tools it's using)

## Phase 4: Governance Backfill

← research: [Onboarding](/research/onboarding) (scanning strategy, progressive disclosure); AD: [AD-015](/architecture/decisions), [AD-016](/architecture/decisions)

- [ ] Codebase scanner (detect languages, frameworks, structure)
- [ ] Conversational onboarding flow (ask questions, generate governance artifacts)
- [ ] Agent definition generator
- [ ] Rule generator from conversation
- [ ] Architecture decision capture from conversation

## Phase 5: Learning Loops

← research: [Persistence](/research/persistence) (session persistence, FTS5 search); product: [Journeys](/product/journeys) (Journey 5: Learning Loop)

- [ ] Retrospective UI (IMPL/RETRO cards with promotion workflow)
- [ ] Metrics dashboard (KPIs rendered as charts) ← research: [Frontend](/research/frontend) (LayerChart)
- [ ] Session continuity (handoff notes, cross-session search) ← research: [Persistence](/research/persistence); spec: [F-013](/product/mvp-specification)
- [ ] Bug investigation workflow (screenshot + annotation + doc comparison)
- [ ] Cross-project learning — Global lesson store (app-level, not per-project). Promote lessons/rules from project-local to global scope. New projects inherit relevant global patterns during onboarding. Tag-based relevance matching (language, framework, domain).

## Future: Provider Ecosystem

The provider-agnostic sidecar interface supports additional providers without changing the Rust core or Svelte UI. Each provider is a new implementation behind the same `ProviderEvent` protocol. ← AD: [AD-017](/architecture/decisions); research: [Claude Integration](/research/claude-integration)

- [ ] API key provider (Anthropic TypeScript SDK — direct HTTP, pay-per-token)
- [ ] Cloud provider routing (Amazon Bedrock, Google Vertex AI, Azure AI Foundry)
- [ ] Alternative model providers (OpenAI, Google Gemini, open-weight models)
- [ ] Local model support (Ollama, llama.cpp — for offline/air-gapped use)

## Future: Multi-User Collaborative Access

A small team (PM, Tech Lead, Developer) sharing a single Forge instance with visibility into each other's sessions and governance work. Schema includes nullable `user_id` and `last_edited_by` columns from Phase 1 to avoid migration-heavy changes later. ← research: [Persistence](/research/persistence) (multi-user considerations); product: [Personas](/product/personas)

- [ ] User identity and authentication (local accounts or SSO)
- [ ] Session visibility controls (own, team, all)
- [ ] Artifact edit locking or conflict resolution
- [ ] Shared vs. personal settings
- [ ] Server component for concurrent multi-machine access
- [ ] Connection pooling — current `Mutex<Connection>` is fine for single-user but becomes a bottleneck under concurrent access. Swap to `r2d2`/`deadpool-sqlite` pool (2-4 read + 1 write). Code-level refactor, no schema changes.
- [ ] Evaluate SQLite vs. PostgreSQL — SQLite is single-writer; true multi-machine concurrent access may require a server-backed database

## Future: Wireframe Browser & Interactive UX Flows

Wireframes are generated as styled images during Phase 0d and stored in a local image cache. This future work makes them a first-class browsable, interactive artifact within Forge. ← research: [Wireframing](/research/wireframing), [Design Tokens](/research/design-tokens); product: [Journeys](/product/journeys)

- [ ] Wireframe browser view — Browse all wireframes for a project, organized by UX flow / journey
- [ ] Custom markdown block for wireframe images — Renders cached wireframes with style-awareness (serves light/dark/brand variant based on active theme)
- [ ] Interactive UX flow navigation — Custom markdown block that links wireframe images into clickable flows, allowing users to navigate through screens as part of an interactive review process
- [ ] On-demand wireframe regeneration — When design tokens change (new brand colors, theme switch), regenerate wireframe variants from source definitions
- [ ] Wireframe-to-component traceability — Link wireframe screens to the Svelte components that implement them


## Future: Design Tool Integration

Rather than building comprehensive design tooling internally, integrate with 3rd-party design tools where designers already work. This enables a **Designer persona** to complete the end-to-end team (PM/Tech Lead + Developer + Designer) and avoids reinventing design tooling. ← research: [Design Tokens](/research/design-tokens); product: [Personas](/product/personas)

- [ ] Figma integration — Import design tokens (colors, typography, spacing) from Figma files. Use Figma MCP server for two-way communication. Extract component specifications from Figma designs.
- [ ] Design token sync — Bidirectional sync between project design tokens and external design tools. Changes in Figma propagate to Forge's per-project theme; governance-defined tokens can be pushed back. ← research: [Design Tokens](/research/design-tokens)
- [ ] Design-to-wireframe pipeline — Import high-fidelity designs from Figma as wireframe references. Link designs to user journeys and UX flows within Forge.
- [ ] Code-to-Figma backfill — Analyze existing frontend implementation (components, styles, layout) and generate corresponding Figma components that accurately represent them. Use those components to reconstruct wireframes as Figma designs, enabling teams who prototyped in code to backfill a proper design system. This bridges the gap to Figma's dev tools pipeline for full design-to-development integration. Potentially unique in the market — no existing tool automates code→Figma component generation at this level.
- [ ] Designer persona — Extend the persona model with a Designer role who defines visual standards, reviews UI compliance, and manages the design system through Forge's governance framework. ← product: [Personas](/product/personas)
- [ ] Replace automated wireframing — If Figma integration matures, automated wireframe generation becomes optional. Designers create wireframes in their native tool; Forge indexes and organizes them.
