# Forge TODO

**Last Updated:** 2026-03-02

---

## Phase 0: Tech Stack Research & Decisions

Research and resolve these decision points before writing application code. Each item should result in a documented decision with rationale.

### Claude Integration

- [ ] **Claude Agent SDK vs raw API** — The Agent SDK handles the tool-use loop (tool calls, results, multi-turn). Research: Does it support streaming responses? Can we intercept tool calls to render them in the UI before execution (approval flow)? Does it work from Rust via FFI, or do we need a Node/Python sidecar? Can it target both Claude API (API key) and Claude Max (subscription)?
- [ ] **Claude Max integration path** — How does Claude Max expose API access for third-party apps? OAuth flow? Session tokens? Is there a documented API, or would this require reverse-engineering the web client? This determines whether Claude Max support is feasible at launch or a later addition.
- [ ] **Tool implementation strategy** — Claude Code's tools (Read, Write, Edit, Bash, Glob, Grep) need equivalents in Forge. Options: (a) implement natively in Rust, (b) shell out to system commands, (c) embed Claude Code as a subprocess and proxy its tool calls. Option (c) preserves exact compatibility but adds complexity. Option (a) gives full control over the UI integration.
- [ ] **Streaming architecture** — Claude API streams tokens via SSE. The UI needs to render tokens as they arrive (conversation), but also intercept structured events (tool calls, agent delegation). Design the event pipeline: API stream → Rust backend parser → Tauri IPC → Svelte reactive store.

### Tauri v2

- [ ] **Tauri v2 capability audit** — Confirm Tauri 2.0 covers all requirements: file system access (read/write arbitrary project files), process spawning (git commands, Python scanners, shell commands), file watching (detect changes for re-scans), system tray, auto-update, cross-platform builds (Windows, macOS, Linux).
- [ ] **IPC design** — Tauri uses `invoke` for Rust↔JS communication. Design the command interface: what Rust functions does the frontend call? What events does the backend push to the frontend? How do long-running operations (agent loops, scanner runs) report progress?
- [ ] **Security model** — Tauri's permission system restricts what the frontend can access. Forge needs broad file system access (it reads/writes project files). Document the required permissions and how to scope them per-project.
- [ ] **Plugin ecosystem** — Check if Tauri plugins exist for: SQLite (sql-plugin), file watching (fs-watch), system shell (shell-plugin), auto-update, window management. Prefer plugins over custom Rust code where available.

### Frontend

- [ ] **Markdown rendering + editing** — Need a component that renders markdown beautifully AND supports inline editing (click to edit, save back to file). Options: MDsveX (compile-time, not ideal for dynamic content), unified/remark (runtime rendering), CodeMirror 6 (editor with markdown mode), Milkdown (WYSIWYG markdown editor built on ProseMirror). Evaluate for: live preview, syntax highlighting in code blocks, frontmatter support, performance with large docs.
- [ ] **Conversation UI component** — Research existing chat/conversation UI patterns for Svelte. Need: streaming token display, tool call rendering (collapsible, with approval buttons), agent delegation indicators, message search, session history navigation. Consider: shadcn-svelte as component base? Custom conversation component?
- [ ] **Panel layout system** — The app has conversation + multiple artifact panels (docs, scanners, tasks, metrics). Need a flexible panel layout: resizable, collapsible, tabbed panels, drag to reorder. Options: custom CSS grid, svelte-splitpanes, or a dock layout library.
- [ ] **Chart/visualization library** — For scanner dashboards and metrics. Options: Chart.js (simple, well-known), D3 (powerful, complex), LayerCake (Svelte-native), uPlot (fast, minimal). Evaluate for: time series (scanner pass/fail over time), bar charts (coverage), status indicators.

### Persistence

- [ ] **SQLite schema design** — What tables? At minimum: projects, sessions, messages, artifacts (docs, rules, agents, skills), scanner_results, metrics, tasks. Design for: fast session history queries, full-text search over conversation history, linking messages to artifact changes.
- [ ] **File vs DB boundary** — Process artifacts (docs, rules, agents, skills) live on the filesystem as markdown files (they're committed to git). But Forge also needs metadata (last scan time, edit history, relationship graph). Decision: store metadata in SQLite, always read artifact content from disk? Or cache content in DB with file-watch invalidation?
- [ ] **Session persistence model** — How much conversation history to persist? Full message history (can get large)? Summary + key messages? How to handle context window limits across sessions — does Forge manage its own summarization?

### Project Onboarding / Backfill

- [ ] **Codebase scanning strategy** — When opening a new project, Forge needs to understand the structure (languages, frameworks, services, test setup). Options: tree-sitter for language parsing, simple glob/regex heuristics, or delegate to Claude ("here's the file tree, what do you see?"). Balance between speed and accuracy.
- [ ] **Governance framework format** — The Alvarez framework uses `.claude/` for agents/rules/hooks and `.agents/skills/` for portable skills. Should Forge adopt this exact format (compatibility with Claude Code CLI), or define its own format that's more structured (JSON/YAML instead of markdown)? Strong argument for compatibility: users can switch between Forge and CLI.
- [ ] **Progressive disclosure** — New users shouldn't see a blank governance framework with 15 empty agent slots. Design the onboarding flow: start with conversation only, introduce docs as they're created, surface the process dashboard after enough history accumulates. The framework grows organically from the conversation.

---

## Phase 1: Scaffold

- [ ] Initialize Tauri v2 + Svelte 5 project
- [ ] Set up Rust backend with basic IPC commands
- [ ] Set up Svelte frontend with basic layout (conversation + side panel)
- [ ] Configure SQLite for session persistence
- [ ] Add basic Claude API integration (send message, receive streaming response)
- [ ] First working demo: chat with Claude in the desktop app

---

## Phase 2: File System Integration

- [ ] Implement file tools (Read, Write, Edit, Glob, Grep) in Rust backend
- [ ] Project file tree panel in UI
- [ ] File viewer/editor panel (markdown rendering + code highlighting)
- [ ] Git status integration (show modified files, branch info)

---

## Phase 3: Process Layer

- [ ] Documentation panel (browse, render, edit project docs)
- [ ] Scanner runner (execute Python scanners, parse results)
- [ ] Scanner dashboard (pass/fail history, violation details)
- [ ] Agent activity panel (which agent is working, what tools it's using)

---

## Phase 4: Governance Backfill

- [ ] Codebase scanner (detect languages, frameworks, structure)
- [ ] Conversational onboarding flow (ask questions → generate governance artifacts)
- [ ] Agent definition generator
- [ ] Rule generator from conversation
- [ ] Architecture decision capture from conversation

---

## Phase 5: Learning Loops

- [ ] Retrospective UI (IMPL/DEBT/RETRO cards with promotion workflow)
- [ ] Metrics dashboard (KPIs from process/metrics.md rendered as charts)
- [ ] Session continuity (handoff notes, cross-session search)
- [ ] Bug investigation workflow (screenshot + annotation + doc comparison)
