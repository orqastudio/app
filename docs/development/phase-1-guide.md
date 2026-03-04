# Phase 1 Implementation Guide

> Single-page reference for implementation agents during the MVP scaffold phase.

---

## Overview

Phase 1 delivers the Forge MVP: a dogfood-ready desktop app that replaces the CLI-based bootstrap process for Forge's own development. After this phase, Forge can manage its own governance artifacts, run conversations with Claude, persist sessions, and display tool calls. Every feature below serves the dogfooding milestone defined in [MVP Specification](/product/mvp-specification).

**Prerequisites:** Phases 0a-0e (research, architecture decisions, product definition, UX design, technical design) are all complete and approved.

---

## Feature Matrix

| Feature ID | Name | Sub-Phase | Governing Docs | Key Files (Rust) | Key Files (Frontend) | Agent |
|------------|------|-----------|-----------------|-------------------|----------------------|-------|
| F-011 | SQLite Infrastructure | 1 | [sqlite-schema](/architecture/sqlite-schema) | `lib.rs`, `state.rs`, `error.rs`, `repo/` | -- | `backend-engineer` |
| F-006 | Three-Zone + Nav Sub-Panel Layout | 2 | [svelte-components](/architecture/svelte-components), [core-layout wireframe](/wireframes/core-layout) | -- | `AppLayout`, `ActivityBar`, `NavSubPanel`, `StatusBar`, `Toolbar` | `frontend-engineer` |
| F-001 | Project Open & Codebase Scan | 3 | [ipc-commands](/architecture/ipc-commands) (project_open, project_scan), [sqlite-schema](/architecture/sqlite-schema) (projects) | `commands/project_commands.rs`, `scanner/`, `domain/project.rs`, `repo/project_repo.rs` | `ProjectDashboard`, `ProjectSwitcher`, `project.svelte.ts`, `commands/project.ts` | `backend-engineer` + `frontend-engineer` |
| F-001b | New Project Workflow | 3 | [ipc-commands](/architecture/ipc-commands) (project_create), [mvp-specification](/product/mvp-specification) F-001b | `commands/project_commands.rs`, `scanner/` | `ProjectSwitcher`, `WelcomeScreen` | `backend-engineer` + `frontend-engineer` |
| F-002 | Agent SDK Sidecar Lifecycle | 4 | [rust-modules](/architecture/rust-modules) (sidecar/), [streaming-pipeline](/architecture/streaming-pipeline) | `sidecar/manager.rs`, `sidecar/protocol.rs`, `sidecar/types.rs` | `sidecar.svelte.ts`, `commands/sidecar.ts` | `backend-engineer` |
| F-012 | MCP Tool Server | 5 | [tool-definitions](/architecture/tool-definitions), [mcp-host](/architecture/mcp-host) | `tools/mcp_server.rs`, `tools/read.rs`, `tools/write.rs`, `tools/edit.rs`, `tools/bash.rs`, `tools/glob.rs`, `tools/grep.rs`, `tools/security.rs` | -- | `backend-engineer` |
| F-005 | Session Persistence | 6 | [ipc-commands](/architecture/ipc-commands) (session_*), [sqlite-schema](/architecture/sqlite-schema) (sessions, messages) | `commands/session_commands.rs`, `commands/message_commands.rs`, `repo/session_repo.rs`, `repo/message_repo.rs`, `domain/session.rs`, `domain/message.rs` | `session.svelte.ts`, `commands/session.ts`, `SessionDropdown`, `SessionHeader` | `backend-engineer` + `frontend-engineer` |
| F-003 | Conversation Streaming | 7 | [ipc-commands](/architecture/ipc-commands) (stream_send_message), [streaming-pipeline](/architecture/streaming-pipeline), [conversation-view wireframe](/wireframes/conversation-view) | `sidecar/stream.rs`, `commands/message_commands.rs` | `ConversationView`, `MessageBubble`, `UserMessage`, `AssistantMessage`, `MessageInput`, `StreamingIndicator`, `conversation.svelte.ts`, `commands/conversation.ts` | `backend-engineer` + `frontend-engineer` |
| F-004 | Tool Call Display | 8 | [tool-definitions](/architecture/tool-definitions), [conversation-view wireframe](/wireframes/conversation-view) | -- (uses existing stream events) | `ToolCallCard`, `ToolCallInput`, `ToolCallOutput`, `DiffView` | `frontend-engineer` |
| F-007 | Artifact Browser | 9 | [ipc-commands](/architecture/ipc-commands) (artifact_*), [artifact-browser wireframe](/wireframes/artifact-browser) | `commands/artifact_commands.rs`, `repo/artifact_repo.rs`, `watcher/artifact_watcher.rs` | `ArtifactBrowser`, `ArtifactListItem`, `ArtifactViewer`, `FrontmatterDisplay`, `artifact.svelte.ts`, `commands/artifact.ts`, `DocTreeNav`, `ArtifactListNav` | `backend-engineer` + `frontend-engineer` |
| F-008 | Artifact Editor | 10 | [ipc-commands](/architecture/ipc-commands) (artifact_update, artifact_create) | `commands/artifact_commands.rs` | `ArtifactEditor`, `MarkdownEditor` (CodeMirror 6) | `frontend-engineer` |
| F-009 | Settings View | 11 | [ipc-commands](/architecture/ipc-commands) (settings_*), [settings wireframe](/wireframes/settings-onboarding) | `commands/settings_commands.rs`, `repo/settings_repo.rs` | `SettingsView`, `ProviderSettings`, `ProjectSettings`, `AppearanceSettings`, `ThemeToggle` | `frontend-engineer` + `backend-engineer` |
| F-010 | Status Bar | 11 | [svelte-components](/architecture/svelte-components) | -- | `StatusBar` | `frontend-engineer` |
| F-013 | Session Handoff Summaries | 12 | [ipc-commands](/architecture/ipc-commands) (session_end), [streaming-pipeline](/architecture/streaming-pipeline) | `commands/session_commands.rs`, `sidecar/` | `SessionDropdown` (handoff preview) | `backend-engineer` |
| F-014 | Auto-Session on Plan Mode | 13 | [mvp-specification](/product/mvp-specification) F-014 | `commands/session_commands.rs`, `sidecar/stream.rs` | `session.svelte.ts`, `conversation.svelte.ts` | `backend-engineer` + `frontend-engineer` |

---

## Sub-Phase Breakdown

### Sub-Phase 1: SQLite Infrastructure (F-011)

- **Goal:** Database foundation with schema, migrations, and repository layer.
- **Depends on:** Nothing (first sub-phase).
- **Creates:**
  - `src-tauri/migrations/001_initial_schema.sql` -- 9 tables + 2 FTS5 + indexes + triggers
  - `src-tauri/migrations/002_add_themes.sql` -- project_themes + project_theme_overrides
  - `src-tauri/src/error.rs` -- ForgeError enum with thiserror
  - `src-tauri/src/state.rs` -- AppState struct
  - `src-tauri/src/domain/mod.rs` and all domain type files
  - `src-tauri/src/repo/mod.rs` and all repository files
  - `src-tauri/src/lib.rs` -- app builder with plugin registration and migration setup
  - `src-tauri/src/main.rs` -- entry point
- **Acceptance Criteria:**
  - Database file created in Tauri app data directory
  - Migrations applied via tauri-plugin-sql on startup
  - Phase 1 tables exist: projects, sessions, messages, message_blocks, artifacts, feature_gates
  - FTS5 virtual tables for message search
  - WAL mode enabled
  - All repo CRUD operations pass unit tests
- **Governing Docs:** [SQLite Schema](/architecture/sqlite-schema), [Rust Modules](/architecture/rust-modules) sections 3-5
- **Agent:** `backend-engineer`

### Sub-Phase 2: Application Shell (F-006)

- **Goal:** Three-zone + nav sub-panel layout with Activity Bar, resizable panels, and status bar.
- **Depends on:** Sub-Phase 1 (settings persistence for panel state).
- **Creates:**
  - `src/routes/+layout.svelte` -- ThemeProvider, global keybindings, Sonner
  - `src/routes/+page.svelte` -- single-page container
  - `src/lib/components/layout/AppLayout.svelte`
  - `src/lib/components/layout/ActivityBar.svelte` + `ActivityBarItem.svelte`
  - `src/lib/components/layout/NavSubPanel.svelte`
  - `src/lib/components/layout/Toolbar.svelte`
  - `src/lib/components/layout/StatusBar.svelte`
  - `src/lib/components/layout/WelcomeScreen.svelte`
  - `src/lib/stores/navigation.svelte.ts`
  - `src/lib/stores/theme.svelte.ts`
  - `src/app.css` -- Tailwind directives, shadcn CSS variables, Forge semantic tokens
- **Acceptance Criteria:**
  - Activity Bar: fixed 48px icon rail with Project Dashboard, Docs (default active), Agents, Rules, Skills, Hooks, separator, Settings (bottom)
  - Active icon has 2px left border indicator + highlighted background
  - Keyboard shortcuts: `Ctrl+0` through `Ctrl+5`, `Ctrl+,`, `Ctrl+B`
  - Nav Sub-Panel: 200px default, min 160px, max 280px, collapsible
  - PaneForge manages three resizable zones
  - Panel sizes and collapse state persist across restarts
  - Functional at 900x600px minimum; auto-collapse below 1200px
- **Governing Docs:** [Svelte Components](/architecture/svelte-components), [Core Layout Wireframe](/wireframes/core-layout), [Information Architecture](/product/information-architecture)
- **Agent:** `frontend-engineer`

### Sub-Phase 3: Project Open & New Project (F-001, F-001b)

- **Goal:** Open existing directories and create new projects with governance scaffolding.
- **Depends on:** Sub-Phase 1 (project repo), Sub-Phase 2 (shell for rendering).
- **Creates:**
  - `src-tauri/src/commands/project_commands.rs` -- project_open, project_create, project_list, project_get, project_get_active, project_scan
  - `src-tauri/src/scanner/mod.rs`, `tier1.rs`, `tier2.rs`, `theme_extractor.rs`
  - `src/lib/stores/project.svelte.ts`
  - `src/lib/commands/project.ts`
  - `src/lib/components/navigation/ProjectDashboard.svelte`
  - `src/lib/components/navigation/ProjectSwitcher.svelte`
- **Acceptance Criteria:**
  - F-001: Open via native file dialog; Tier 1 scan < 100ms; Tier 2 scan < 5s for 10k files; `.claude/` artifacts indexed; last-opened remembered; empty project shows empty state
  - F-001b: New Project action available; directory created with `.claude/` skeleton (CLAUDE.md, agents/, rules/, skills/, hooks/, docs/); git init offered; project registered; discovery conversation offered (not a wizard); skip option with defaults
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (Project Commands), [MVP Specification](/product/mvp-specification) F-001 + F-001b
- **Agent:** `backend-engineer` (commands, scanner), `frontend-engineer` (project UI)

### Sub-Phase 4: Sidecar Lifecycle (F-002)

- **Goal:** Spawn, monitor, and communicate with the Agent SDK sidecar process.
- **Depends on:** Sub-Phase 1 (state management), Sub-Phase 3 (project context for sidecar config).
- **Creates:**
  - `src-tauri/src/sidecar/mod.rs`
  - `src-tauri/src/sidecar/manager.rs` -- SidecarManager: spawn, kill, restart, health check
  - `src-tauri/src/sidecar/protocol.rs` -- NDJSON line framing
  - `src-tauri/src/sidecar/types.rs` -- SidecarStatus, SidecarRequest, SidecarResponse
  - `src-tauri/src/commands/sidecar_commands.rs` -- sidecar_status, sidecar_restart
  - `src/lib/stores/sidecar.svelte.ts`
  - `src/lib/commands/sidecar.ts`
  - Bundled sidecar binary (Bun-compiled TypeScript)
- **Acceptance Criteria:**
  - Sidecar binary bundled with app (~18-25 MB)
  - Spawns via tauri-plugin-shell on first conversation request
  - Communicates via stdin/stdout NDJSON
  - Health check detects unexpected exit
  - Missing Claude Code CLI shows clear error with installation guidance
  - Status bar shows sidecar state: not started / starting / connected / error
  - Clean termination on app close
- **Governing Docs:** [Rust Modules](/architecture/rust-modules) section 6, [Streaming Pipeline](/architecture/streaming-pipeline)
- **Agent:** `backend-engineer`

### Sub-Phase 5: MCP Tool Server (F-012)

- **Goal:** Expose Forge's tools as an MCP server for the sidecar.
- **Depends on:** Sub-Phase 4 (sidecar to connect to).
- **Creates:**
  - `src-tauri/src/tools/mod.rs` -- ToolRegistry, Tool trait
  - `src-tauri/src/tools/mcp_server.rs` -- MCP JSON-RPC protocol
  - `src-tauri/src/tools/read.rs`, `write.rs`, `edit.rs`, `bash.rs`, `glob.rs`, `grep.rs`
  - `src-tauri/src/tools/security.rs` -- path validation, scope enforcement
- **Acceptance Criteria:**
  - MCP server registered as `mcpServers: { "forge": ... }` in sidecar config
  - Agent SDK built-in tools disabled (`tools: []`)
  - All 6 tools available: Read, Write, Edit, Bash, Glob, Grep
  - Tools execute natively in Rust
  - Security scopes enforced (no access outside project root + home)
- **Governing Docs:** [Tool Definitions](/architecture/tool-definitions), [MCP Host](/architecture/mcp-host), [Rust Modules](/architecture/rust-modules) section 7
- **Agent:** `backend-engineer`

### Sub-Phase 6: Session Persistence (F-005)

- **Goal:** Automatic session creation, persistence, listing, and switching.
- **Depends on:** Sub-Phase 1 (session/message repos), Sub-Phase 2 (session dropdown location), Sub-Phase 3 (project context).
- **Creates:**
  - `src-tauri/src/commands/session_commands.rs` -- session_create, session_list, session_get, session_update_title, session_end, session_delete
  - `src-tauri/src/commands/message_commands.rs` -- message_list, message_search
  - `src/lib/stores/session.svelte.ts`
  - `src/lib/commands/session.ts`
  - `src/lib/components/navigation/SessionDropdown.svelte`
  - `src/lib/components/conversation/SessionHeader.svelte`
- **Acceptance Criteria:**
  - Sessions auto-created on first message
  - Metadata stored in SQLite: title, created_at, updated_at, message_count, model (stores "auto" when auto model selection active)
  - Messages stored one row per content block
  - Auto-titled from first user message (50 chars)
  - Session list in Chat Panel header dropdown, ordered most recent
  - Clicking session loads full history
  - New Session via `Ctrl+N`
  - Persist across restarts; last-active restored on launch
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (Session + Message Commands), [SQLite Schema](/architecture/sqlite-schema) (sessions, messages tables)
- **Agent:** `backend-engineer` (commands, repos), `frontend-engineer` (session UI)

### Sub-Phase 7: Conversation Streaming (F-003)

- **Goal:** Send messages and receive streaming AI responses with markdown rendering.
- **Depends on:** Sub-Phase 4 (sidecar), Sub-Phase 5 (MCP tools), Sub-Phase 6 (session to stream into).
- **Creates:**
  - `src-tauri/src/sidecar/stream.rs` -- StreamHandler: NDJSON parser, Channel<T> forwarder, DB buffer
  - `src/lib/stores/conversation.svelte.ts`
  - `src/lib/commands/conversation.ts`
  - `src/lib/components/conversation/ConversationView.svelte`
  - `src/lib/components/conversation/MessageBubble.svelte`
  - `src/lib/components/conversation/UserMessage.svelte`
  - `src/lib/components/conversation/AssistantMessage.svelte`
  - `src/lib/components/conversation/SystemMessage.svelte`
  - `src/lib/components/conversation/MessageInput.svelte`
  - `src/lib/components/conversation/StreamingIndicator.svelte`
  - `src/lib/components/content/MarkdownRenderer.svelte`
  - `src/lib/components/content/CodeBlock.svelte`
- **Acceptance Criteria:**
  - Enter sends, Shift+Enter inserts newline
  - Message sent to sidecar via stdin NDJSON
  - Tokens stream character by character via Channel<T>
  - First token < 2s after send
  - Markdown rendered: headings, bold, italic, code blocks, lists
  - Code blocks have syntax highlighting (svelte-highlight)
  - User/assistant messages visually distinct
  - Auto-scroll during streaming; user can scroll up without snap-back
  - Errors display as error blocks in conversation
  - Model selector includes "Auto (recommended)" default option
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (stream_send_message, StreamEvent), [Streaming Pipeline](/architecture/streaming-pipeline), [Conversation View Wireframe](/wireframes/conversation-view)
- **Agent:** `backend-engineer` (StreamHandler), `frontend-engineer` (conversation UI)

### Sub-Phase 8: Tool Call Display (F-004)

- **Goal:** Render tool calls as collapsible cards inline in conversation.
- **Depends on:** Sub-Phase 7 (conversation streaming provides tool events).
- **Creates:**
  - `src/lib/components/tool/ToolCallCard.svelte`
  - `src/lib/components/tool/ToolCallInput.svelte`
  - `src/lib/components/tool/ToolCallOutput.svelte`
  - `src/lib/components/tool/DiffView.svelte`
- **Acceptance Criteria:**
  - Tool calls appear inline at invocation point
  - Card shows: tool name, input summary (truncated), status badge
  - Collapsed by default; expanding reveals full input + output
  - File tool calls show file path prominently
  - Edit/Write calls show diff view (green additions, red deletions)
  - Command calls show command + output in monospace
  - Error results are visually distinct
  - Read-only in Phase 1 (no approval/denial controls)
- **Governing Docs:** [Tool Definitions](/architecture/tool-definitions), [Conversation View Wireframe](/wireframes/conversation-view)
- **Agent:** `frontend-engineer`

### Sub-Phase 9: Artifact Browser (F-007)

- **Goal:** Browse governance artifacts in the Explorer Panel with file watcher updates.
- **Depends on:** Sub-Phase 2 (Explorer Panel), Sub-Phase 3 (project with indexed artifacts).
- **Creates:**
  - `src-tauri/src/commands/artifact_commands.rs` -- artifact_list, artifact_get, artifact_get_by_path, artifact_create, artifact_update, artifact_delete
  - `src-tauri/src/watcher/artifact_watcher.rs` -- notify-debouncer-full, 500ms debounce
  - `src/lib/stores/artifact.svelte.ts`
  - `src/lib/commands/artifact.ts`
  - `src/lib/components/artifact/ArtifactBrowser.svelte`
  - `src/lib/components/artifact/ArtifactListItem.svelte`
  - `src/lib/components/artifact/ArtifactViewer.svelte`
  - `src/lib/components/content/FrontmatterDisplay.svelte`
  - `src/lib/components/navigation/DocTreeNav.svelte`
  - `src/lib/components/navigation/ArtifactListNav.svelte`
- **Acceptance Criteria:**
  - Explorer shows artifact browser when category active in Activity Bar
  - Activity Bar icons select category (no tab bar in Explorer)
  - List entries show filename + brief description (frontmatter or first paragraph)
  - Hooks category shows both lifecycle hooks and hookify rules with subtype indicator and filter
  - Clicking artifact opens viewer in Explorer (conversation stays in Chat Panel)
  - Markdown rendered with proper formatting; YAML frontmatter as structured metadata
  - File watcher updates within 500ms of disk change
  - Empty categories show meaningful empty state
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (Artifact Commands), [Artifact Browser Wireframe](/wireframes/artifact-browser)
- **Agent:** `backend-engineer` (commands, watcher), `frontend-engineer` (artifact UI)

### Sub-Phase 10: Artifact Editor (F-008)

- **Goal:** Edit governance artifacts via CodeMirror 6 source editing.
- **Depends on:** Sub-Phase 9 (artifact viewer to add edit mode to).
- **Creates:**
  - `src/lib/components/artifact/ArtifactEditor.svelte`
  - `src/lib/components/content/MarkdownEditor.svelte` -- CodeMirror 6 integration
- **Acceptance Criteria:**
  - "Edit" toggle (`Ctrl+E`) switches to CodeMirror 6 editor
  - Markdown syntax highlighting in editor
  - YAML frontmatter editable
  - `Ctrl+S` saves to disk immediately
  - Rendered view reflects saved changes
  - Unsaved changes indicator in title
  - Close with unsaved changes prompts: save, discard, cancel
  - "New" button creates template file and opens in edit mode
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (artifact_update, artifact_create), [Artifact Browser Wireframe](/wireframes/artifact-browser)
- **Agent:** `frontend-engineer`

### Sub-Phase 11: Settings & Status Bar (F-009, F-010)

- **Goal:** Settings view in Explorer Panel and status bar showing system state.
- **Depends on:** Sub-Phase 2 (shell), Sub-Phase 4 (sidecar status for display).
- **Creates:**
  - `src-tauri/src/commands/settings_commands.rs` -- settings_get, settings_set, settings_get_all
  - `src-tauri/src/commands/theme_commands.rs` -- theme_get_project, theme_set_override, theme_clear_overrides
  - `src/lib/components/settings/SettingsView.svelte`
  - `src/lib/components/settings/ProviderSettings.svelte`
  - `src/lib/components/settings/ProjectSettings.svelte`
  - `src/lib/components/settings/AppearanceSettings.svelte`
  - `src/lib/components/settings/ThemeToggle.svelte`
  - `src/lib/components/settings/ShortcutsReference.svelte`
  - `src/lib/commands/settings.ts`
- **Acceptance Criteria:**
  - F-009: Settings via Activity Bar icon or `Ctrl+,`; opens in Explorer Panel; Provider section shows sidecar/CLI status; Project section shows root path, detected stack, file watcher status; Appearance has theme toggle (light/dark/system) + font size; persisted via tauri-plugin-store; theme changes apply immediately
  - F-010: Status bar spans full width; sidecar status with icon + text; active model name (or "Auto -> resolved model"); token usage per session; color-coded indicators (green/yellow/red)
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (Settings + Theme Commands), [Settings Wireframe](/wireframes/settings-onboarding)
- **Agent:** `frontend-engineer` (settings UI, status bar), `backend-engineer` (settings + theme commands)

### Sub-Phase 12: Session Handoff Summaries (F-013)

- **Goal:** Claude-powered summaries on session end for cross-session continuity.
- **Depends on:** Sub-Phase 6 (session lifecycle), Sub-Phase 4 (sidecar for summary generation).
- **Creates:**
  - Handoff summary generation logic in `sidecar/stream.rs` or `sidecar/manager.rs`
  - Rule-based fallback summary function
  - Session dropdown handoff preview rendering
- **Acceptance Criteria:**
  - Handoff summary generated when session ends (new session started or explicit close)
  - Summary prompt covers: accomplished, in progress, decisions, blockers, files modified
  - Stored in sessions.summary and sessions.handoff_notes; kept under 500 words
  - New sessions for same project include previous handoff notes in system prompt
  - Rule-based fallback if sidecar unavailable (last user message, modified files, tool call count)
  - Handoff notes visible in session list (expandable preview)
  - Summary generation does not block next session (async)
- **Governing Docs:** [IPC Commands](/architecture/ipc-commands) (session_end), [MVP Specification](/product/mvp-specification) F-013
- **Agent:** `backend-engineer`

### Sub-Phase 13: Auto-Session on Plan Mode (F-014)

- **Goal:** Automatically create new sessions when plan mode triggers.
- **Depends on:** Sub-Phase 6 (session creation), Sub-Phase 7 (conversation streaming for detection).
- **Creates:**
  - Plan mode detection logic in sidecar or stream handler
  - Auto-session creation with `[Plan] <topic>` title
  - Settings toggle for auto-session behavior
- **Acceptance Criteria:**
  - Plan-mode trigger detected (explicit user request or autonomous Claude plan mode)
  - New session created with `[Plan] <topic>` title
  - Previous session preserved with full history
  - Chat Panel switches to new session immediately
  - Session dropdown shows new session
  - User can rename auto-generated title
  - Behavior configurable: users can disable in Settings > Project
  - When disabled, plan mode starts in current session
- **Governing Docs:** [MVP Specification](/product/mvp-specification) F-014
- **Agent:** `backend-engineer` + `frontend-engineer`

---

## Verification Checklist

Run after each sub-phase and before merging to main.

### Rust Backend

```bash
cargo build                          # Compiles without errors
cargo clippy -- -D warnings          # Zero clippy warnings
cargo fmt --check                    # Formatting matches rustfmt
cargo test                           # All tests pass
```

### Frontend

```bash
npm run build                        # Builds without errors
npm run check                        # svelte-check + tsc pass
npm run lint                         # ESLint passes
npm run test                         # Vitest passes
```

### End-to-End Completeness (per feature)

- [ ] Rust `#[tauri::command]` function exists and is registered in the invoke handler
- [ ] Input/output Rust types derive `Serialize`, `Deserialize`
- [ ] Matching TypeScript interfaces exist in `src/lib/types/`
- [ ] Svelte component calls `invoke()` via typed wrapper with correct command name
- [ ] Store manages state lifecycle (loading, loaded, error)
- [ ] Types are consistent across Rust structs and TypeScript interfaces
- [ ] No `unwrap()`, `expect()`, or `panic!()` in production Rust code
- [ ] No `any` types in TypeScript
- [ ] No Svelte 4 patterns (`$:`, `export let`, `let:`)
- [ ] No TODO/FIXME comments in committed code
- [ ] No stub/placeholder data in production code
- [ ] No `invoke()` calls inside `$lib/components/` (component purity)

### Dogfooding Validation (final gate after all sub-phases)

Per [MVP Specification](/product/mvp-specification) dogfooding checklist:

- [ ] Can browse governance artifacts (agents, rules, skills, hooks, docs)
- [ ] Can edit governance artifacts (save persists to disk and UI)
- [ ] Can run a conversation (send message, streaming response, tool calls displayed)
- [ ] Can review tool calls (expand card, see input/output)
- [ ] Can manage sessions (new, switch via dropdown, history)
- [ ] Can detect project context (open Forge on itself, see detected stack + governance)
- [ ] Persistence works (close, reopen, last session + project restored)

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | Phase 1 builds the session persistence and handoff summary infrastructure (F-005, F-013) that enables knowledge accumulation across sessions -- the foundation the learning loop requires. |
| Process Governance | Phase 1 delivers the artifact browser and editor (F-007, F-008) that make governance artifacts visible and editable, and the conversation interface (F-003) through which governed agents operate. |

---

## Related Documents

- [MVP Specification](/product/mvp-specification) -- Feature acceptance criteria
- [IPC Commands](/architecture/ipc-commands) -- Full command catalog
- [Rust Modules](/architecture/rust-modules) -- Module tree and domain types
- [Svelte Components](/architecture/svelte-components) -- Component tree and store design
- [SQLite Schema](/architecture/sqlite-schema) -- Table definitions and migrations
- [Streaming Pipeline](/architecture/streaming-pipeline) -- End-to-end streaming architecture
- [Tool Definitions](/architecture/tool-definitions) -- MCP tool specs
- [Roadmap](/product/roadmap) -- Phase 1 in the context of all phases
