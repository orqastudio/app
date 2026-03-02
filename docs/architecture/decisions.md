# Architecture Decisions

**Date:** 2026-03-02

Architecture decisions for Forge. Each decision is numbered AD-NNN and is immutable once recorded (can only be superseded by a new AD).

## Decision Log

| AD | Title | Date | Status |
|----|-------|------|--------|
| AD-001 | Thick backend architecture | 2026-03-02 | Active |
| AD-002 | IPC boundary design | 2026-03-02 | Active (streaming clarified by AD-009) |
| AD-003 | Error propagation via Result types | 2026-03-02 | Active |
| AD-004 | Svelte 5 runes only | 2026-03-02 | Active |
| AD-005 | SQLite for all structured persistence | 2026-03-02 | Active |
| AD-006 | Component purity (pages fetch, components display) | 2026-03-02 | Active |
| AD-007 | Agent SDK sidecar integration | 2026-03-02 | Active |
| AD-008 | Max subscription authentication | 2026-03-02 | Active |
| AD-009 | Streaming pipeline | 2026-03-02 | Active |
| AD-010 | Tool implementation as MCP | 2026-03-02 | Active |
| AD-011 | Security model | 2026-03-02 | Active |
| AD-012 | Tauri plugin selections | 2026-03-02 | Active |
| AD-013 | Frontend library selections | 2026-03-02 | Active |
| AD-014 | Persistence architecture | 2026-03-02 | Active |
| AD-015 | Governance artifact format | 2026-03-02 | Active |
| AD-016 | Onboarding strategy | 2026-03-02 | Active |
| AD-017 | Composability principle | 2026-03-02 | Active |
| AD-018 | Four-zone VS Code-style layout | 2026-03-02 | Active (supersedes layout aspects of AD-013) |

---

## AD-001: Thick Backend Architecture

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Rust owns the domain model — sessions, artifacts, governance state, agent orchestration. Svelte is a view layer that renders what Rust tells it.

**Rationale:** The domain logic (session management, artifact parsing, Claude API integration, file system tools) is complex and benefits from Rust's type system, performance, and reliability. Keeping it in the backend means the frontend stays simple and testable.

**Consequences:** All business logic lives in `src-tauri/src/domain/`. The frontend never makes decisions about data flow or state management beyond UI concerns.

---

## AD-002: IPC Boundary Design

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Tauri `invoke()` commands are the only mechanism for frontend-backend communication. All commands are defined as `#[tauri::command]` functions in Rust.

**Rationale:** A single, typed communication channel makes the boundary auditable, testable, and secure. No HTTP servers, no WebSocket servers, no shared memory.

**Consequences:** Every feature that crosses the boundary needs a Rust command + TypeScript type. Streaming data uses Tauri events (`emit`/`listen`).

---

## AD-003: Error Propagation via Result Types

**Date:** 2026-03-02 | **Status:** Active

**Decision:** All Rust functions return `Result<T, E>` using `thiserror` for error types. No `unwrap()`, `expect()`, or `panic!()` in production code.

**Rationale:** A desktop app must never crash. Every error path must be handled gracefully and communicated to the user through the UI.

**Consequences:** Every command handler returns `Result<T, String>` (or a custom error type). The frontend must handle both success and error responses.

---

## AD-004: Svelte 5 Runes Only

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Use Svelte 5 runes exclusively (`$state`, `$derived`, `$effect`, `$props`). No Svelte 4 patterns.

**Rationale:** Runes provide a cleaner, more predictable reactivity model. Mixing old and new patterns creates confusion and inconsistency.

**Consequences:** All components use `$props()` instead of `export let`, `$derived()` instead of `$:`, and `{#snippet}` instead of `<slot>`.

---

## AD-005: SQLite for All Structured Persistence

**Date:** 2026-03-02 | **Status:** Active

**Decision:** SQLite is the sole persistence layer for structured data (sessions, messages, metrics, project config). File-based artifacts (docs, rules, agents) are read from disk.

**Rationale:** SQLite is embedded, requires no external process, supports full-text search, and handles concurrent reads well. Perfect for a desktop app.

**Consequences:** Schema managed via numbered migrations. Repository pattern in Rust for all database access. In-memory SQLite for testing.

---

## AD-006: Component Purity

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Pages and containers fetch data (call `invoke()`). Display components receive data via props only. No `invoke()` calls inside `$lib/components/`.

**Rationale:** Separating data fetching from rendering makes components testable, reusable, and predictable. A component that fetches its own data is harder to test and reason about.

**Consequences:** Route pages (`+page.svelte`) and container components handle data loading. All components in `$lib/components/` are pure — they render what they receive.

---

## AD-007: Agent SDK Sidecar Integration

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Forge integrates with Claude via a Bun-compiled TypeScript sidecar running the official Agent SDK (`@anthropic-ai/claude-agent-sdk`). The sidecar communicates with the Rust backend via stdin/stdout newline-delimited JSON (NDJSON). The Rust backend spawns the sidecar via `tauri-plugin-shell` `spawn()`.

**Rationale:** The Agent SDK spawns the official Claude Code CLI binary, which is the only legal path to Max subscription usage. Bun compile produces ~18-25 MB standalone binaries per platform — far smaller than Electron (~150+ MB). stdin/stdout NDJSON is the same pattern used by LSP (Language Server Protocol) and adds negligible latency (~0.1-0.5ms per event vs Claude's 30-100ms/token generation). The Agent SDK provides `tools: []` to disable built-in tools, `canUseTool` for approval UI delegation, `includePartialMessages` for token-level streaming, and custom MCP servers for routing tool execution to Forge.

**Consequences:** The build pipeline must cross-compile the sidecar for each target platform (`bun build --compile --target=<platform>`). Process lifecycle management (start, restart on crash, kill on exit) must be implemented in Rust. The Claude Code CLI must be installed on the user's machine for the Agent SDK path to function.

**Research:** [Claude Integration Research](/research/claude-integration)

---

## AD-008: Max Subscription Authentication

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Forge's primary authentication path is Claude Max subscription via the Agent SDK. The Agent SDK spawns the official Claude Code CLI, which authenticates via OAuth 2.0 with PKCE against claude.ai. API key authentication and other providers (Bedrock, Vertex, OpenAI) are deferred to the roadmap as future sidecar providers.

**Rationale:** Max subscription ($100-200/month flat) is cost-effective at 25+ conversations/day vs API billing (~$304/month at 75 conversations/day with Sonnet). Third-party OAuth token usage is banned and enforced with server-side fingerprinting since January 2026 — the Agent SDK is the only legal path. The provider-agnostic sidecar interface (AD-017) ensures future providers can be added without changing the Rust core or Svelte UI.

**Consequences:** Users must have Claude Code CLI installed and authenticated (`claude login`). Forge must detect whether authentication is available and surface clear guidance if not. Rate limits from Max subscription (5-hour rolling windows, weekly caps) must be surfaced in the UI.

**Research:** [Claude Integration Research](/research/claude-integration)

---

## AD-009: Streaming Pipeline

**Date:** 2026-03-02 | **Status:** Active

**Decision:** The streaming pipeline flows: Agent SDK (SSE from Claude API) → TypeScript sidecar (translates to `ProviderEvent` NDJSON) → stdout → Rust (`CommandEvent::Stdout`, parse JSON, deserialize) → `Channel<T>` (Tauri IPC) → Svelte store (`$state`, fine-grained reactivity) → component re-render.

**Clarifies AD-002:** AD-002 states "streaming data uses Tauri events (emit/listen)". This is refined: streaming uses `Channel<T>` (faster than events, ordered delivery, type-safe on Rust side) for high-throughput data like Claude token streams. Events (`emit`/`listen`) remain appropriate for low-frequency notifications (file changes, app lifecycle).

**Rationale:** `Channel<T>` is Tauri's recommended streaming mechanism — faster than events, with ordered delivery and index-based sequencing. The sidecar hop adds ~0.1-0.5ms per event, which is negligible compared to Claude's 30-100ms per token.

**Consequences:** The Rust backend must define a `ProviderEvent` enum that covers all streaming event types (text deltas, thinking deltas, tool use events, completion, errors). The Svelte store must accumulate tokens efficiently using `$state` for fine-grained DOM updates.

**Research:** [Claude Integration Research](/research/claude-integration), [Tauri v2 Research](/research/tauri-v2)

---

## AD-010: Tool Implementation as MCP

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Forge's core tools (Read, Write, Edit, Bash, Glob, Grep) are implemented natively in Rust and exposed to the Agent SDK as a custom MCP server via `createSdkMcpServer()`. The Agent SDK's built-in tools are disabled (`tools: []`). Forge also acts as an MCP host for user-provided MCP servers (extensibility).

**Rationale:** Native Rust tools give Forge full control over tool execution, permission model, and UI rendering. Exposing them as MCP servers to the Agent SDK is the SDK's documented extension mechanism. Disabling built-in tools ensures Claude cannot read/write files without Forge's knowledge. The MCP host capability connects Forge to the 10,000+ MCP server ecosystem.

**Consequences:** Each core tool must be implemented as both a Rust function (for execution) and an MCP tool definition (for the Agent SDK). The `canUseTool` callback routes approval requests to Forge's UI before execution. Tool results flow back to the Agent SDK, which sends them to Claude for the next conversation turn.

**Research:** [Claude Integration Research](/research/claude-integration)

---

## AD-011: Security Model

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Tauri's three-layer security model (permissions → scopes → capabilities) configured with `$HOME/**` base file system scope, sensitive path denials (`.ssh`, `.gnupg`), pre-declared shell commands with argument validators, and `tauri-plugin-keyring` for API key storage in the OS keychain.

**Rationale:** Tauri's capability system compiles permissions into the binary at build time. Deny always takes precedence over allow. The keyring plugin stores secrets in macOS Keychain, Windows Credential Manager, or Linux Secret Service — never in plaintext files or the app store.

**Consequences:** Capabilities defined in `src-tauri/capabilities/default.json`. Runtime scope expansion via `app_handle.fs_scope().allow_directory()` for user-selected project directories. `tauri-plugin-persisted-scope` remembers permissions across restarts. Shell commands (git, sh) must be pre-declared with regex argument validators.

**Research:** [Tauri v2 Research](/research/tauri-v2)

---

## AD-012: Tauri Plugin Selections

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Use the following Tauri plugins, all official and stable:

| Plugin | Version | Purpose |
|--------|---------|---------|
| `tauri-plugin-sql` | 2.3.1 | SQLite persistence (with `--features sqlite`) |
| `tauri-plugin-fs` | 2.4.5 | File system access (with `watch` feature) |
| `tauri-plugin-shell` | 2.3.4 | Git/shell commands + sidecar management |
| `tauri-plugin-store` | 2.4.2 | App preferences and UI state (NOT for secrets) |
| `tauri-plugin-autostart` | 2.5.1 | Optional launch at system startup |
| `tauri-plugin-updater` | 2.9.0 | Auto-update via GitHub Releases |
| `tauri-plugin-window-state` | 2.4.1 | Persist window size/position |
| `tauri-plugin-dialog` | 2.4.2 | File/folder selection dialogs |
| `tauri-plugin-notification` | 2.3.3 | System notifications |
| `tauri-plugin-keyring` | community | API key storage in OS keychain |
| `tauri-plugin-persisted-scope` | official | Remember file system permissions |

**Rationale:** All plugins exist in the official `tauri-apps/plugins-workspace` or are well-maintained community packages. Each addresses a specific Forge requirement confirmed during Tauri v2 research.

**Consequences:** Plugin versions must be tracked for security updates. `tauri-plugin-sql` migrations managed via `include_str!()` referencing `.sql` files. Store plugin is for preferences only, never for secrets or relational data.

**Research:** [Tauri v2 Research](/research/tauri-v2)

---

## AD-013: Frontend Library Selections

**Date:** 2026-03-02 | **Status:** Active

**Decision:**

| Concern | Library | Rationale |
|---------|---------|-----------|
| UI primitives | shadcn-svelte | Svelte 5 native, composable, accessible, unstyled base |
| Markdown editing | CodeMirror 6 (`svelte-codemirror-editor` v2.1.0) | Svelte 5 runes support, virtual scrolling, extension ecosystem |
| Markdown rendering | `@humanspeak/svelte-markdown` v0.8.13 | Svelte 5 runes, strict TypeScript, caching |
| Syntax highlighting | `svelte-highlight` v7.9.0 | highlight.js wrapper, Svelte 5 compatible |
| Panel layout | PaneForge v1.0.2 (shadcn-svelte `Resizable`) | Svelte 5 native, same ecosystem as shadcn-svelte |
| Charts | LayerChart (shadcn-svelte `Chart`) | Composable Svelte components, covers time series and bar charts |
| Icons | `lucide-svelte` | Consistent with shadcn-svelte ecosystem |
| Conversation UI | Custom (on shadcn-svelte primitives) | No existing library handles streaming + tool cards + approval flows |

**Rationale:** shadcn-svelte has already made most of these choices (`Resizable` = PaneForge, `Chart` = LayerChart). Aligning with the ecosystem ensures design consistency. No WYSIWYG markdown editor — source-level editing preserves formatting fidelity for governance artifacts committed to git.

**Consequences:** All UI components build on shadcn-svelte primitives. Custom conversation UI must handle streaming token accumulation, tool call rendering, and approval flows.

**Research:** [Frontend Research](/research/frontend)

---

## AD-014: Persistence Architecture

**Date:** 2026-03-02 | **Status:** Active

**Decision:** 9 core tables (`projects`, `sessions`, `messages`, `artifacts`, `scanner_results`, `metrics`, `tasks`, `lessons`, `settings`) plus 2 FTS5 virtual tables (`messages_fts`, `artifacts_fts`). Messages stored as one row per content block (not per API message). `rusqlite` for Rust backend operations, `tauri-plugin-sql` for frontend queries and migrations. WAL mode for concurrent access. Governance artifacts (.claude/ files) are metadata-indexed in SQLite but content is always read from disk. Claude-generated handoff summaries on session end from Phase 1. Nullable `user_id` and `last_edited_by` columns included from Phase 1 to support future multi-user expansion without schema migration.

**Rationale:** One row per content block enables granular FTS indexing, efficient streaming updates, tool-specific queries, and natural UI component mapping. The hybrid file/DB approach (Obsidian pattern) preserves full CLI compatibility — `.claude/` files are authoritative, SQLite is a derived cache. `notify-debouncer-full` file watcher (500ms debounce) keeps the index in sync. Full session history stored (< 5 GB/year at heavy use). FTS5 cross-session search performs under 50ms at 1M messages. Handoff summaries address the #1 pain point (context loss between sessions) — negligible cost via sidecar. Multi-user columns are nullable and unused in Phase 1, avoiding a migration-heavy schema change when collaborative access is implemented.

**Consequences:** Schema managed via numbered SQL migration files in `src-tauri/migrations/`. `.claude/` directory watched with `notify` crate. `forge.db` added to `.gitignore`. SHA-256 hashes used for change detection to avoid re-indexing unchanged files. Sidecar must support a "generate summary" request type for handoff note generation.

**Research:** [Persistence Research](/research/persistence), [Onboarding Research](/research/onboarding)

---

## AD-015: Governance Artifact Format

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Forge reads and writes governance artifacts in the exact `.claude/` format (markdown with YAML frontmatter for agents/skills, pure markdown for rules, JSON for settings). Forge-specific metadata (compliance status, usage counts, parsed timestamps) lives only in SQLite — files are never modified to add Forge metadata.

**Rationale:** Full compatibility with Claude Code CLI. Users can switch between Forge and CLI seamlessly. Markdown is the natural format for AI instructions. Parsing via `yaml-front-matter` (frontmatter extraction) + `comrak` (markdown body parsing/rendering, used by crates.io and docs.rs).

**Consequences:** Files are always authoritative — if a file changes on disk, the SQLite cache is updated to match. No Forge-specific annotations in `.claude/` files. The DB stores enriched metadata (word count, heading structure, tool lists, compliance status) that does not exist in the files.

**Research:** [Onboarding Research](/research/onboarding)

---

## AD-016: Onboarding Strategy

**Date:** 2026-03-02 | **Status:** Active

**Decision:** Three-tier codebase scanning: Tier 1 (manifest-file heuristics, <100ms, automatic), Tier 2 (`hyperpolyglot` language detection, ~1-2s, automatic), Tier 3 (Claude analysis, on-demand). Conversation-first progressive disclosure: features appear as they become relevant, controlled by a `feature_gates` table in SQLite. No wizard. Value in under 1 minute.

**Rationale:** The target user is a PM/Tech Lead, not a developer configuring an IDE. Manifest-file heuristics cover 90%+ of projects instantly. Progressive disclosure avoids cognitive overload — the conversation IS the primary interface, and governance features reveal themselves through natural interaction (Linear, Notion, Obsidian precedent).

**Consequences:** Navigation grows organically (Conversation only → + Artifacts → + Sessions → full nav). Empty states must be actionable, not decorative. Feature gates stored in SQLite with `first_used_at` and `visible` flags. The first-run flow is: API key → open project → auto-scan → conversation.

**Research:** [Onboarding Research](/research/onboarding)

---

## AD-017: Composability Principle

**Date:** 2026-03-02 | **Status:** Active

**Decision:** External integrations (AI providers, MCP servers) connect through provider-agnostic interfaces. The Rust core speaks a neutral `ProviderEvent` protocol; provider-specific logic lives in swappable sidecar processes. Phase 1 implements one provider (Agent SDK for Max subscription). Future providers implement the same interface without changing the core.

**Rationale:** Extends the composability design principle from the Alvarez project. Decoupling the AI provider from the core application means: (1) switching providers requires only a new sidecar implementation, (2) supporting multiple providers simultaneously is architecturally possible, (3) the Rust core and Svelte UI are tested independently of any provider, (4) if Anthropic releases a Rust SDK, the sidecar can be replaced with a native implementation.

**Consequences:** The `ProviderEvent` enum must be stable and provider-neutral. The sidecar protocol (stdin/stdout NDJSON) is the contract — any process that speaks it can be a provider. Provider selection and configuration surfaces in Forge's settings UI.

**Research:** [Claude Integration Research](/research/claude-integration), [Product Governance](/product/governance)

---

## AD-018: Four-Zone VS Code-Style Layout

**Date:** 2026-03-02 | **Status:** Active (supersedes layout aspects of AD-013)

**Decision:** Replace the three-pane layout (Sidebar | Primary/Conversation | Detail) with a four-zone VS Code-style layout: Activity Bar (48px fixed icon rail) | Explorer Panel (flexible, artifact-centric) | Sessions Panel (240px, collapsible) | Chat Panel (flexible, conversation). The Activity Bar replaces the old five-tab bar in the detail panel with direct icon navigation for Docs, Agents, Rules, Skills, Hooks (artifact categories), plus Scanners, Metrics, Learning (dashboards, Phase 3-5), and Settings. PaneForge manages the three resizable zones; the Activity Bar sits outside PaneForge as a fixed CSS flex element.

**Rationale:** The original three-pane layout placed conversation at center, implying it was the primary content. Forge is an artifact-centric product management tool — docs, agents, rules, skills, and hooks are the deliverables. The VS Code-style layout makes artifacts the focal point (Explorer Panel), with conversation supporting from the right (Chat Panel). This pattern is familiar to developers, scales to accommodate future dashboard views without tab bar proliferation, and provides clear visual hierarchy: artifacts are work, conversation is collaboration.

**Supersedes:** Layout aspects of AD-013 (panel arrangement and tab navigation). AD-013's library selections (PaneForge, shadcn-svelte, CodeMirror 6, LayerChart, lucide-svelte) remain active.

**Consequences:** The `ArtifactBrowser` component no longer has internal category tabs — it receives the active category as a prop from the Activity Bar state. New components: `ActivityBar.svelte`, `ActivityBarItem.svelte`, `SessionsPanel.svelte`. The `NavigationStore` is restructured around `activeActivity` (Activity Bar selection) and `explorerView` (what the Explorer Panel shows). Keyboard shortcuts `Ctrl+1` through `Ctrl+5` switch artifact categories directly. The detail panel toggle (`Ctrl+\`) is removed; the Sessions Panel toggle (`Ctrl+B`) replaces the old sidebar toggle.

**Research:** [Frontend Research](/research/frontend), [Information Architecture](/product/information-architecture)
