# Orqa Studio TODO

**Last Updated:** 2026-03-04

**Current goal: DOGFOOD MILESTONE — use Orqa Studio to build itself.**

The app already has conversations, streaming, 6 file tools (read, write, edit, bash, glob, grep), tool approval protocol, session persistence, governance scanning, and artifact browsing. The gaps below are what's needed to switch from Claude Code CLI to Orqa Studio for all future development.

---

## Dogfood Blockers (DO THESE FIRST)

Critical issues that prevent using the app for real development work.

### D-001: System Prompt — Inject CLAUDE.md + Governance

`stream_commands.rs:156` hardcodes `system_prompt: None`. Claude inside the app has no project context — no rules, no coding standards, no governance. This is the single biggest blocker.

- [ ] On `stream_send_message`, read CLAUDE.md from the active project and pass it as the system prompt
- [ ] Include AGENTS.md content if present
- [ ] Include a summary of governance scan results (coverage, key rules)
- [ ] Frontend: Allow users to view/edit the system prompt before sending (optional, can be Phase 2)

### D-002: Tool Approval UI

The tool approval protocol exists (sidecar → Rust → frontend → back) but needs verification that the frontend actually renders approve/deny buttons. Without this, write_file and bash are either auto-approved (dangerous) or silently blocked.

- [ ] Verify tool approval requests reach the frontend and render a UI prompt
- [ ] If missing: implement approve/deny dialog for tool calls (especially write_file, edit_file, bash)
- [ ] Read-only tools (read_file, glob, grep) can auto-approve
- [ ] Show tool input (file path, content preview) in the approval dialog

### D-003: Context Window Management

The app hit "Prompt is too long" when Claude tried to read all project docs. The CLI handles this with auto-compaction; the app needs a strategy.

- [ ] Set a reasonable max context size in the sidecar/Agent SDK config
- [ ] Handle context overflow gracefully (show error, suggest shorter prompt)
- [ ] Consider: summarize large file reads, truncate tool outputs over a threshold

### D-004: Hook Errors

User reports hook errors on every prompt. Investigate and fix.

- [ ] Diagnose which hook is erroring (session-start or skill-instructions)
- [ ] Fix the root cause (likely `pwd` sensitivity in session-start-hook.sh)
- [ ] These are Claude Code CLI hooks, not app hooks — but they affect the development workflow

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
