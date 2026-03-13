---
id: DOC-042
title: MVP Feature Specification
description: Feature specification for the minimum viable product release of OrqaStudio.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

Exactly what Phase 1 delivers, with acceptance criteria for each feature. Scoped to the minimum needed for dogfooding — OrqaStudio™ must be able to manage its own development after this phase. In the current implementation, OrqaStudio uses the Claude Code CLI as the AI provider; it supplements the CLI with a visual governance management layer.

**Guiding principle:** Ship the smallest thing that lets us supplement the CLI-based bootstrap process with OrqaStudio's visual governance UI. Every feature below serves the dogfooding milestone. Features that would be nice but aren't needed for dogfooding are listed under "Explicitly Deferred."

**Universal acceptance criterion:** All governance artifacts created or modified by OrqaStudio are stored under `.orqa/` as standard markdown files and are immediately usable by any compatible tool. For Claude Code CLI compatibility, OrqaStudio optionally creates `.claude/` symlinks pointing to `.orqa/` paths. No proprietary formats, no migration steps.

---

## MVP Features

### F-001: Project Open & Codebase Scan

**Description:** User can open a project directory. OrqaStudio scans the codebase and populates project metadata.

**Acceptance Criteria:**
- [ ] User can open a directory via native file dialog (tauri-plugin-dialog)
- [ ] Tier 1 scan (manifest heuristics) completes in < 100ms and detects: languages, frameworks, package manager, project name
- [ ] Tier 1 scan also detects design system tokens: `tailwind.config.*` theme colors/fonts, CSS custom properties in `:root`, component library conventions
- [ ] Tier 2 scan (hyperpolyglot) completes in < 5s for projects up to 10,000 files
- [ ] Detected metadata (including design tokens) is stored in SQLite and displayed in the Project Dashboard (Activity Bar)
- [ ] If design tokens are found, OrqaStudio's UI theme adapts to match the project's color palette (per-project theming). Missing tokens fall back to OrqaStudio defaults.
- [ ] If `.orqa/` directory exists, governance artifacts are indexed and counts displayed. If only a `.claude/` directory exists (legacy CLI-only project), those artifacts are detected and migration to `.orqa/` is offered.
- [ ] Last-opened project is remembered across app restarts (tauri-plugin-store)
- [ ] Empty project (no recognized files) shows a meaningful empty state

**Architecture References:** [AD-016](AD-016) (onboarding strategy), [AD-014](AD-014) (persistence), [AD-012](AD-012) (plugins)

---

### F-001b: New Project Workflow

**Description:** User can create a brand new project from scratch. OrqaStudio initializes the directory structure, scaffolds an `.orqa/` governance skeleton, and drops the user into a conversation.

**Acceptance Criteria:**

*Directory and registration:*
- [ ] "New Project" action is available alongside "Open Project" (toolbar or welcome state)
- [ ] User provides a project name and selects a parent directory (or selects an existing empty directory)
- [ ] OrqaStudio creates the project directory (if needed) with: `.orqa/project.json`, `.orqa/agents/`, `.orqa/rules/`, `.orqa/skills/`, `.orqa/governance/hooks/`, `.orqa/documentation/`, `.gitignore` (with `orqa.db`)
- [ ] If the directory is not a git repo, OrqaStudio offers to run `git init`
- [ ] Project is registered in SQLite and set as active
- [ ] Nav Sub-Panel shows the new project with "New project, no code detected" status
- [ ] If CLI tool compatibility is requested (e.g., Claude Code CLI), OrqaStudio creates symlinks in the appropriate tool directory (e.g., `.claude/`) pointing to `.orqa/` paths
- [ ] Empty artifact browser categories show actionable empty states ("Create your first agent")

*Project discovery conversation:*
- [ ] After directory creation, OrqaStudio offers to start a project discovery conversation (not a wizard)
- [ ] The discovery session uses a system prompt that structures the conversation through product definition, tech stack, team, conventions, and prior art topics
- [ ] User answers naturally in conversation — Claude synthesizes the responses and asks follow-ups
- [ ] The system prompt guides Claude to cover all topics but does not enforce a rigid order
- [ ] User can say "that's enough" or "skip" at any point to end discovery early
- [ ] At the end of discovery, Claude proposes governance artifacts based on the conversation
- [ ] User reviews and approves, modifies, or rejects each proposed artifact before it is written to disk
- [ ] Generated project summary document (`.orqa/documentation/product/overview.md`) contains a project summary derived from the conversation (product description, tech stack, team, key decisions). If CLI tool compatibility is requested, a corresponding `CLAUDE.md` or similar tool-specific file is also written.
- [ ] Generated agents are tailored to the stated tech stack (not generic defaults)
- [ ] Generated rules reflect stated conventions and preferences
- [ ] The discovery conversation is saved as a regular session (appears in session history, is searchable and resumable)
- [ ] User can skip discovery entirely via a "Skip — Use Defaults" action and start with generic scaffolding
- [ ] Skipping creates a minimal project summary with empty governance directories

**Architecture References:** [AD-015](AD-015) (governance format), [AD-016](AD-016) (onboarding strategy), [AD-014](AD-014) (persistence)

---

### F-002: Agent SDK Sidecar Lifecycle

**Description:** In the current implementation, OrqaStudio uses the Claude Code CLI as the AI provider and spawns an Agent SDK sidecar process for communication. The sidecar is the provider-specific bridge between OrqaStudio's provider-agnostic Rust core and the AI provider's capabilities. Future providers will follow the same sidecar interface.

**Acceptance Criteria:**
- [ ] Sidecar binary is bundled with the application (Bun-compiled, ~18-25 MB)
- [ ] Sidecar spawns via `tauri-plugin-shell` `spawn()` on first conversation request
- [ ] Sidecar communicates via stdin/stdout NDJSON protocol
- [ ] Health check: OrqaStudio detects if sidecar exits unexpectedly and reports status
- [ ] If the AI provider (Claude Code CLI in the current implementation) is not available, OrqaStudio shows a clear error with installation guidance
- [ ] Status bar shows sidecar state: not started, starting, connected, error
- [ ] Sidecar process is terminated cleanly on app close

**Architecture References:** [AD-007](AD-007) (sidecar integration), [AD-008](AD-008) (Max authentication), [AD-012](AD-012) (plugins)

---

### F-003: Conversation Streaming

**Description:** User can send messages and receive streaming AI responses in real-time.

**Acceptance Criteria:**
- [ ] User can type a message in the input area and send with Enter
- [ ] Shift+Enter inserts a newline without sending
- [ ] Message is sent to sidecar via stdin NDJSON
- [ ] Response tokens stream into the conversation panel character by character
- [ ] Streaming uses the pipeline: sidecar stdout → Rust NDJSON parser → Channel<T> → Svelte `$state` → DOM
- [ ] First token appears in < 2 seconds after send (network permitting)
- [ ] Complete messages are rendered as markdown (headings, bold, italic, code blocks, lists)
- [ ] Code blocks have syntax highlighting (svelte-highlight)
- [ ] User messages are visually distinct from assistant messages
- [ ] The conversation auto-scrolls during streaming (scroll-to-bottom behavior)
- [ ] User can scroll up during streaming without being snapped back to bottom
- [ ] Error responses (sidecar errors, provider errors) display as error blocks in the conversation
- [ ] Model selector includes "Auto (recommended)" as the default option when the provider supports auto model selection
- [ ] Auto delegates model choice to the provider; the sidecar resolves to the best available model based on rate limits and availability
- [ ] If the provider does not support auto, the Auto option is hidden and a specific model must be selected

**Architecture References:** [AD-009](AD-009) (streaming pipeline), [AD-013](AD-013) (frontend libraries), [AD-007](AD-007) (sidecar)

---

### F-004: Tool Call Display

**Description:** Tool calls made by the AI during a conversation are displayed as collapsible cards.

**Acceptance Criteria:**
- [ ] Tool calls appear inline in the conversation at the point where the AI invoked them
- [ ] Each tool call card shows: tool name, input summary (truncated), status badge (completed/error)
- [ ] Cards are collapsed by default, showing only the summary line
- [ ] Expanding a card reveals: full input parameters, full output/result
- [ ] File-related tool calls show the file path prominently
- [ ] Edit/Write tool calls show a diff view when expanded (additions green, deletions red)
- [ ] Command tool calls show the command and output with monospace formatting
- [ ] Tool result errors are visually distinct (error styling)
- [ ] Cards are read-only in Phase 1 (no approval/denial controls)

**Architecture References:** [AD-010](AD-010) (tool implementation as MCP), [AD-013](AD-013) (frontend libraries)

---

### F-005: Session Persistence

**Description:** Sessions are saved automatically and can be browsed, resumed, and searched.

**Acceptance Criteria:**
- [ ] Sessions are created automatically when the first message is sent
- [ ] Session metadata (title, created_at, updated_at, message_count, model) is stored in SQLite. The `model` field stores `"auto"` as a string value when auto model selection is active (not null).
- [ ] Messages and content blocks are stored in SQLite (one row per content block)
- [ ] Sessions are auto-titled from the first user message (first 50 characters)
- [ ] Session list appears in the Chat Panel header session dropdown, ordered by most recent
- [ ] Clicking a session in the dropdown loads it in the Chat Panel with full message history
- [ ] The active session title is shown in the dropdown trigger
- [ ] New Session button (`Ctrl+N`) creates a new empty session
- [ ] Sessions persist across app restarts
- [ ] The last-active session is restored on app restart

**Architecture References:** [AD-014](AD-014) (persistence architecture), [AD-005](AD-005) (SQLite)

---

### F-006: Three-Zone + Nav Sub-Panel Layout

**Description:** The main window uses a three-zone + nav sub-panel layout: Activity Bar (fixed 48px icon rail), Nav Sub-Panel (200px, collapsible, per-category sub-navigation), Explorer Panel (flexible, artifact viewer/editor/dashboards), and Chat Panel (flexible, conversation with session dropdown).

**Acceptance Criteria:**
- [ ] Activity Bar is a fixed 48px vertical icon rail on the far left, outside PaneForge
- [ ] Activity Bar has icons for: Project Dashboard (top); Docs (default active), Agents, Rules, Skills, Hooks (lifecycle + hookify); separator; Settings (bottom-aligned)
- [ ] Active Activity Bar icon has 2px left border indicator + highlighted background
- [ ] `Ctrl+0` opens Project Dashboard; `Ctrl+1` through `Ctrl+5` switch artifact categories; `Ctrl+,` opens settings
- [ ] Nav Sub-Panel: default 200px, min 160px, max 280px, collapsible with `Ctrl+B`
- [ ] Nav Sub-Panel shows per-category navigation: structured doc tree for Docs, flat lists for Agents/Rules/Skills/Hooks, category list for Settings
- [ ] Nav Sub-Panel is hidden when Project Dashboard, Scanners, or Metrics is active
- [ ] Doc tree mirrors `docs/` directory structure with expand/collapse state that persists across navigation
- [ ] Explorer Panel fills remaining space, shows selected artifact viewer/editor, project dashboard, or settings form
- [ ] Chat Panel fills remaining space, always shows conversation, not collapsible
- [ ] Chat Panel header includes session dropdown and model selector
- [ ] PaneForge manages three resizable zones: Nav Sub-Panel | Explorer | Chat
- [ ] Pane resize handles are visible and draggable
- [ ] Collapsed Nav Sub-Panel redistributes space to Explorer and Chat
- [ ] Panel sizes and collapse state persist across app restarts (tauri-plugin-window-state)
- [ ] Window size and position persist across restarts
- [ ] Layout is functional at minimum window size of 900x600px
- [ ] Responsive: Nav Sub-Panel auto-collapses below 1200px; overlay Sheet below 720px

**Architecture References:** [AD-019](AD-019) (three-zone + nav sub-panel layout), [AD-013](AD-013) (PaneForge), [AD-012](AD-012) (window-state plugin)

---

### F-007: Artifact Browser

**Description:** User can browse governance artifacts (agents, rules, skills, hooks, docs) in the Explorer Panel.

**Acceptance Criteria:**
- [ ] Explorer Panel shows the artifact browser when an artifact category is active in the Activity Bar
- [ ] Activity Bar icons select the artifact category — no tab bar within the Explorer Panel
- [ ] Each tab lists artifacts found in the corresponding `.orqa/` subdirectory (or `.orqa/documentation/` for documentation)
- [ ] Each list entry shows: filename, brief description (from YAML frontmatter `description` or first paragraph)
- [ ] The Hooks category displays both lifecycle hooks (from `.orqa/governance/hooks/`) and hookify rules (from `.orqa/governance/hooks/hookify.*.local.md`), distinguishable by a subtype indicator
- [ ] Hookify rules display their event type (file edit / bash command), action (block / warn), and pattern summary
- [ ] Users can filter the Hooks list by subtype: All, Lifecycle, Hookify
- [ ] Clicking an artifact opens it in the Explorer Panel artifact viewer (conversation stays visible in Chat Panel)
- [ ] Artifact viewer renders markdown content with proper formatting
- [ ] YAML frontmatter is displayed as structured metadata above the rendered body
- [ ] File watcher detects changes to `.orqa/` files and updates the browser within 500ms
- [ ] Empty categories show a meaningful empty state

**Architecture References:** [AD-015](AD-015) (governance artifact format), [AD-013](AD-013) (frontend libraries)

---

### F-008: Artifact Editor

**Description:** User can edit governance artifacts through the UI.

**Acceptance Criteria:**
- [ ] Artifact viewer has an "Edit" toggle (`Ctrl+E`) that switches to CodeMirror 6 source editing
- [ ] CodeMirror editor has markdown syntax highlighting
- [ ] YAML frontmatter is editable within the same editor
- [ ] `Ctrl+S` saves the file to disk immediately
- [ ] After save, switching back to rendered view shows updated content
- [ ] Unsaved changes are indicated visually (modified indicator in title)
- [ ] Closing the editor with unsaved changes prompts: save, discard, or cancel
- [ ] "New" button in artifact browser creates a template file and opens it in edit mode
- [ ] Edited files are saved to `.orqa/` as standard markdown and are immediately usable by OrqaStudio and any connected CLI tools (via symlinks)

**Architecture References:** [AD-013](AD-013) (CodeMirror 6), [AD-015](AD-015) (governance format)

---

### F-009: Settings View

**Description:** User can view and configure application settings.

**Acceptance Criteria:**
- [ ] Settings are accessible via the Settings icon in the Activity Bar (bottom) or `Ctrl+,`
- [ ] Settings open in the Explorer Panel
- [ ] Provider section shows: sidecar status, AI provider detection status (Claude Code CLI for the current implementation), connection health
- [ ] Project section shows: project root path, detected stack, file watcher status
- [ ] Appearance section has: theme toggle (light/dark/system), font size adjustment
- [ ] Settings are persisted via tauri-plugin-store
- [ ] Theme changes apply immediately without restart

**Architecture References:** [AD-012](AD-012) (plugins), [AD-011](AD-011) (security model)

---

### F-010: Status Bar

**Description:** A status bar at the bottom of the window shows connection and system status.

**Acceptance Criteria:**
- [ ] Status bar spans full window width
- [ ] Shows sidecar connection status: icon + text (Not started / Starting / Connected / Error)
- [ ] Shows active model name (e.g., "Opus 4.6") when a specific model is pinned
- [ ] When Auto model selection is active, shows the resolved model: "Auto → Sonnet 4.6" (updated when streaming begins and the sidecar reports the resolved model)
- [ ] Shows token usage for current session (input tokens / output tokens) — updated per message
- [ ] Status indicators use color coding: green (healthy), yellow (degraded), red (error)

**Architecture References:** [AD-007](AD-007) (sidecar), [AD-013](AD-013) (frontend libraries)

---

### F-011: SQLite Infrastructure

**Description:** SQLite database with schema, migrations, and the core tables needed for Phase 1.

**Acceptance Criteria:**
- [ ] Database file is created in the Tauri app data directory
- [ ] Schema is applied via migrations (tauri-plugin-sql)
- [ ] Phase 1 tables exist: projects, sessions, messages, message_blocks, artifacts, feature_gates
- [ ] FTS5 virtual table exists for session message search (infrastructure ready, UI deferred)
- [ ] WAL mode is enabled for concurrent read/write
- [ ] Database operations use `rusqlite` on the Rust side
- [ ] Frontend queries use `tauri-plugin-sql` via invoke commands

**Architecture References:** [AD-014](AD-014) (persistence), [AD-005](AD-005) (SQLite)

---

### F-012: MCP Tool Server

**Description:** OrqaStudio's tools are exposed as a custom MCP server that the Agent SDK sidecar connects to.

**Acceptance Criteria:**
- [ ] OrqaStudio implements an MCP server that the sidecar registers as `mcpServers: { "orqa-studio": ... }`
- [ ] Agent SDK's built-in tools are disabled (`tools: []`)
- [ ] File tools are available: Read, Write, Edit, Glob, Grep
- [ ] Shell tool is available: Bash (scoped execution)
- [ ] Tool calls from the AI route through the sidecar to OrqaStudio's MCP server
- [ ] Tool results are returned to the AI via the MCP protocol
- [ ] Tools execute natively in Rust (not shelling out)
- [ ] Tool execution respects Tauri security scopes (no access outside project root + home)

**Architecture References:** [AD-010](AD-010) (tool implementation as MCP), [AD-011](AD-011) (security model)

---

### F-013: Session Handoff Summaries

**Description:** When a session ends, OrqaStudio generates an AI-powered summary (via the active AI provider) that captures what happened, what's unfinished, and what the next session should know. Context loss between sessions is the #1 pain point.

**Acceptance Criteria:**
- [ ] When a session ends (user starts a new session, or explicitly closes the current one), the sidecar generates a handoff summary
- [ ] Summary prompt includes: what was accomplished, what was in progress, key decisions made, open questions/blockers, files being modified
- [ ] Summary is stored in `sessions.summary` and `sessions.handoff_notes`
- [ ] Summary is kept under 500 words
- [ ] When a new session starts for the same project, handoff notes from the most recent session are included in the system prompt
- [ ] If the sidecar is unavailable (e.g., app closing quickly), a rule-based fallback generates a basic summary from: last user message, files modified (from tool calls), tool call count
- [ ] Handoff notes are visible in the session list (expandable preview)
- [ ] Generating the summary does not block the user from starting the next session (async generation)

**Architecture References:** [AD-007](AD-007) (sidecar), [AD-009](AD-009) (streaming pipeline), [AD-014](AD-014) (persistence)

---

### F-014: Auto-Session on Plan Mode

**Description:** When a conversation triggers plan mode, OrqaStudio automatically creates a new session to keep planning focused and session history meaningful.

**Acceptance Criteria:**
- [ ] Plan-mode trigger is detected by the sidecar or conversation analysis (both explicit user requests like "plan this" and autonomous Claude-initiated plan mode)
- [ ] When plan mode triggers, a new session is created with title `[Plan] <topic>`
- [ ] The previous session is preserved with its full history
- [ ] The Chat Panel switches to the new session immediately
- [ ] The session dropdown shows the new session after creation
- [ ] User can rename the auto-generated `[Plan]` title
- [ ] Auto-session behavior is configurable: users can disable it in Settings > Project
- [ ] When disabled, plan mode starts in the current session (no new session created)

**Architecture References:** [AD-019](AD-019) (auto-session behavior), [AD-014](AD-014) (persistence), [AD-007](AD-007) (sidecar)

---

## Explicitly Deferred (Not in Phase 1)

These features are valuable but not needed for the dogfooding milestone:

| Feature | Reason for Deferral | Target Phase |
|---------|-------------------|--------------|
| Tool call approval flow | Read-only tool display sufficient for dogfooding | Phase 2 |
| Global search UI | FTS5 infrastructure exists, UI can wait | Phase 2 |
| Project file tree panel | Can browse files through conversation tool calls | Phase 2 |
| Scanner dashboard | No scanners to run yet | Phase 3 |
| Metrics dashboard | No metrics data yet | Phase 5 |
| Learning loop (IMPL capture, promotion) | Process benefit, not blocking dogfooding | Phase 5 |
| Conversational governance backfill | Can manually create/edit artifacts in Phase 1 | Phase 4 |
| Multi-project switching | Single project sufficient for dogfooding | Phase 2 |
| API key provider | Max subscription is primary path | Future |
| Cross-session search UI | FTS5 ready, UI deferred | Phase 2 |
| Multi-user collaborative access | Single-user desktop app first; schema includes nullable user_id for future expansion | Future |

---

## Dogfooding Validation Checklist

After Phase 1 is complete, OrqaStudio must pass these checks to transition from the CLI bootstrap process:

- [ ] **Can browse governance artifacts**: Open OrqaStudio, navigate to artifact browser, see agents/rules/skills/hooks listed (from `.orqa/`), view their content rendered as markdown
- [ ] **Can edit governance artifacts**: Edit an agent file, save, verify change persists on disk in `.orqa/` and in the UI
- [ ] **Can run a conversation**: Send a message, receive a streaming response, see tool calls displayed
- [ ] **Can review tool calls**: Expand a tool call card, see the input and output, understand what the AI did
- [ ] **Can manage sessions**: Start a new session, switch between sessions via the session dropdown, see session history
- [ ] **Can detect project context**: Open OrqaStudio on the OrqaStudio project itself, see detected stack and existing governance artifacts
- [ ] **Persistence works**: Close OrqaStudio, reopen, last session and project are restored

---

## Related Documents

- User Journeys — Workflows that these features support
- Information Architecture — UI structure for these features
- Roadmap — Phase 1 in the context of all phases
- Product Governance — Transition Criteria — Dogfooding milestone
