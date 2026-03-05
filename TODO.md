# Orqa Studio TODO

**Last Updated:** 2026-03-05

**Current goal: DOGFOOD MILESTONE — use Orqa Studio to build itself.**

The app has conversations, streaming, 6 file tools (read, write, edit, bash, glob, grep), search tools (search_regex, search_semantic), tool approval protocol, session persistence, SDK session resume across restarts, governance scanning, enforcement engine, and artifact browsing. The remaining gaps below are what's needed to switch from Claude Code CLI to Orqa Studio for all future development.

---

## Dogfood Blockers (DO THESE FIRST)

*All dogfood blockers are resolved. The app is ready for dogfooding.*

---

---

## Resolved Dogfood Blockers

<details>
<summary>D-001: System Prompt — RESOLVED</summary>

`build_system_prompt()` in `stream_commands.rs` reads CLAUDE.md, AGENTS.md, all `.claude/rules/*.md` files, and the skill catalog from the active project. Injected automatically on every `stream_send_message`.
</details>

<details>
<summary>D-002: Tool Approval UI — RESOLVED</summary>

`ToolApprovalDialog.svelte` renders approve/deny buttons inline in the conversation. Read-only tools (read_file, glob, grep, search_regex, search_semantic, load_skill, code_research) are auto-approved. Write/execute tools (write_file, edit_file, bash) require explicit user approval. Tool input is displayed in the dialog.
</details>

<details>
<summary>D-003: Context Window Management — PARTIALLY RESOLVED</summary>

Tool outputs are truncated at 100K characters (`truncate_tool_output`). Context overflow errors are caught and shown with a friendly message suggesting a new session. Full auto-compaction (summarize + continue) is deferred — not blocking dogfooding since the user can start a new session.
</details>

<details>
<summary>D-004: Hook Errors — RESOLVED</summary>

Root cause: `session-start-hook.sh` used `$(pwd)` which returns POSIX paths on MINGW64, while `git worktree list` uses Windows paths. The `grep -v` filter never matched, causing false warnings on every prompt. Fix: replaced `$(pwd)` with `$(git rev-parse --show-toplevel)` for consistent path format. Also replaced `find` with `ls -d` for orphan detection, and removed nonexistent `skill-instructions-hook.sh` from CLAUDE.md hooks table.
</details>

<details>
<summary>D-005: code_research Tool — RESOLVED</summary>

Added `code_research` dispatch case in `execute_tool()` combining `search_regex` + `search_semantic` results. Added `code_research` tool definition in sidecar `provider.ts` and updated `TOOL_SYSTEM_PROMPT`. The tool is auto-approved (read-only). Full LLM-synthesis version deferred to Phase 4.
</details>

<details>
<summary>D-006: Process Violation Events — RESOLVED</summary>

Added `process_violation` variant to `StreamEvent` TypeScript union. Added handler in `conversationStore.handleStreamEvent()` that accumulates violations. Added `processViolations` state field, cleared on each new message. Violations display as yellow warning banners inline in the conversation after each turn.
</details>

---

## Dogfood Enhancements (DO AFTER BLOCKERS)

Not strictly blocking but significantly improve the dogfooding experience.

### E-001: Artifact Editing (Phase 2c)

Edit governance artifacts directly in the app instead of switching to a text editor.

- [ ] Artifact editor component (CodeMirror 6 for markdown/YAML)
- [ ] Create new artifacts from templates (agents, rules, hooks)
- [ ] Save artifacts back to disk (write to .claude/ files)
- [ ] File watcher for external changes (notify crate, sync when CLI edits files)

### E-002: Self-Learning Loop (Phase 2d)

The learning loop that makes the system improve over time.

**Native artifacts (work in CLI too):**
- [ ] Post-session hook that captures lessons to `docs/development/lessons.md`
- [ ] Rules enforcing lesson checking before implementation
- [ ] CLAUDE.md section describing the promotion pipeline

**App enhancements:**
- [ ] Lesson dashboard showing recurrence trends
- [ ] Browse/edit lessons in the UI
- [ ] Automated promotion suggestions (lesson → rule when recurrence >= threshold)

### E-003: Enforcement & Continuity (Phase 2e)

Rule injection and violation detection during conversations.

**Native artifacts:**
- [ ] Hooks that inject relevant rules into conversations based on file context
- [ ] Hooks that detect and log violations

**App enhancements:**
- [ ] Real-time violation detection during streaming
- [ ] Session handoff summaries for cross-session continuity

### E-004: Git Status Integration

Show what's changed, what branch you're on, help with commits.

- [ ] Git status display (modified files, current branch)
- [ ] Diff viewer for changed files
- [ ] Commit flow from within the app

---

## Deferred (after dogfooding is stable)

These are important but don't block dogfooding. Tracked in `docs/product/roadmap.md`.

- Phase 4: Process Visibility (scanner dashboard, metrics, agent activity)
- Phase 5: Discovery & Research (research artifacts, decision traceability)
- Future: Provider ecosystem (API key, Bedrock, Vertex, local models)
- Future: Multi-user collaborative access
- Future: Design tool integration (Figma)

---

## Completed Phases

<details>
<summary>Phase 0a–0e: Research, Architecture, Product, UX, Technical Design — COMPLETE</summary>

All research, architecture decisions (AD-007 through AD-017), product definition (glossary, personas, journeys, information architecture, MVP spec), UX design (wireframes, design system, component inventory, interaction patterns), and technical design (SQLite schema, IPC catalog, Rust modules, Svelte components, streaming pipeline, tool definitions, MCP host, error taxonomy) are complete and documented.
</details>

<details>
<summary>Phase 1: Scaffold — COMPLETE</summary>

Working Tauri v2 app with Claude conversations via Agent SDK sidecar, 50+ IPC commands, 91 Svelte components, full CRUD, streaming, 6 file tools (read, write, edit, bash, glob, grep) with Rust implementations, tool approval protocol, and semantic code search (ONNX embeddings + DuckDB).
</details>

<details>
<summary>Phase 2a: First-Run Setup Wizard — COMPLETE</summary>

5-step setup wizard (CLI detection, auth verification, sidecar startup, embedding model check, completion) gated by `setup_version` in SQLite. 5 Tauri commands, 6 Svelte components, 13 backend tests.
</details>

<details>
<summary>Phase 2b: Governance Bootstrap — COMPLETE</summary>

Claude-powered governance scanning with 7 Tauri commands, 2 SQLite tables, 5 Svelte components. Scanner covers 7 canonical Claude governance areas. Wizard auto-triggers on project open when coverage < 3/7. Dashboard shows governance health badge.
</details>

<details>
<summary>Dogfood Infrastructure — COMPLETE</summary>

System prompt injection (CLAUDE.md + AGENTS.md + rules + skills), tool approval UI, tool output truncation, context overflow handling, SDK session resume across app restarts (sdk_session_id persisted to SQLite, passed through NDJSON protocol, sidecar rebuilds session mapping on restart).
</details>
