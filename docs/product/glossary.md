# Glossary & Domain Model

**Date:** 2026-03-02

Canonical definitions for terms used throughout Orqa Studio documentation. All product, design, and technical documents should use these terms consistently. If a new term is introduced, add it here first.

---

## Core Concepts

### Session

A single conversation between the user and an AI provider through Orqa Studio. A session has a start time, an optional end time, a sequence of messages, and belongs to a project. Sessions are persisted in SQLite and are searchable via FTS5.

**Not:** A login session or application lifecycle. "Session" always means a conversation session.

### Message

A single unit within a session. Messages have a role (user, assistant, system, tool_call, tool_result), content (one or more content blocks), and a timestamp. Messages are stored as one row per content block for granular search and rendering.

### Content Block

A typed segment within a message: text, code, tool call, tool result, image, or error. The atomic unit of persistence — each content block is a row in the `message_blocks` table.

### Project

A codebase and its associated governance framework that the user manages through Orqa Studio. A project has a root directory, a set of governance artifacts, sessions, and configuration. A user may have multiple projects.

### Workspace

The runtime state of an open project in Orqa Studio: the active session, open panels, selected artifacts, and UI layout. Workspace state is ephemeral (window-state plugin handles persistence across restarts).

---

## Governance Artifacts

### Artifact

Any structured document that defines how a project operates. Governance artifacts include agents, rules, skills, hooks, hookify files, and documentation. Artifacts live on disk in `.claude/` or `docs/` (authoritative) and are indexed in SQLite (derived cache).

### Agent

A specialized AI persona with a defined role, skill set, and domain boundary. Agents are defined as markdown files (`.claude/agents/*.md`) with YAML frontmatter specifying skills and capabilities. In the current bootstrap process, agents are Claude Code subagents; in Orqa Studio, they become provider-agnostic definitions that the sidecar instantiates.

**Examples:** backend-engineer, frontend-engineer, code-reviewer, systems-architect.

### Rule

A constraint that agents must follow during implementation. Rules are markdown files (`.claude/rules/*.md`) that encode coding standards, workflow requirements, architectural constraints, and process gates. Rules are the primary mechanism for enforcing governance.

**Examples:** "All Rust code must use Result types, not unwrap," "Every task must pass through a review gate before completion."

### Skill

A reusable knowledge package that provides domain-specific context to agents. Skills are loaded on demand and supply agents with framework-specific patterns, best practices, and reference material. Skills come from the skills.sh registry or custom project definitions.

**Examples:** svelte5-best-practices, rust-async-patterns, tauri-v2, chunkhound.

### Hook

A governance enforcement mechanism triggered automatically during development. Orqa Studio supports two types of hooks:

**Lifecycle hooks** (`.claude/hooks/`) — Shell scripts that execute in response to lifecycle events (session start, stop). Used for process reminders and checklists.

**Hookify rules** (`.claude/hookify.*.local.md`) — Pattern-based enforcement files that block or warn in real-time when a file edit or bash command matches a forbidden pattern. Used for active prevention of code-level violations.

Both types appear in the Hooks view of the Activity Bar.

### Documentation

Structured project knowledge maintained in `docs/`. Documentation covers architecture decisions, research findings, process definitions, UI specifications, and product definitions. Documentation is the foundation layer — agents, rules, and skills reference docs, not the other way around.

---

## Process Concepts

### Governance Framework

The complete set of artifacts (agents, rules, skills, hooks, hookify files, documentation) that define how a project operates. The governance framework is the product that Orqa Studio makes visible and manageable.

### Two-Pillar Test

The feature acceptance gate: every feature must serve at least one of (1) Self-Learning Loop or (2) Process Governance. Features that serve neither are rejected. When pillars conflict, Process Governance takes priority. See [Product Governance](/product/governance).

### Self-Learning Loop (Pillar 1)

The system's ability to improve over time. Implemented through lesson capture, pattern promotion, retrospectives, metrics tracking, and session continuity. The learning loop feeds outcomes from agent sessions back into the governance framework.

### Process Governance (Pillar 2)

The visibility and enforceability of standards, rules, and workflows. Implemented through rule enforcement, agent management, scanner dashboards, architecture decision tracking, and the documentation-first workflow.

### Human Approval Gate

The requirement that implementation plans must be approved by a human (Product Manager or Tech Lead) before coding begins. Agents plan and implement, but humans authorize. This is a foundational principle.

### Documentation-First Workflow

The mandate that all significant work follows: Document → Approve → Implement → Verify. No implementation begins without approved documentation. No feature is complete without verified documentation.

---

## Implementation Cycle

### Implementation Plan

A structured proposal for how a task will be implemented. Includes scope, approach, affected files, testing strategy, and acceptance criteria. Must be approved by the human user (acting as PM/Tech Lead) before coding begins.

### Review Gate

A verification checkpoint after implementation completes. The standard gate involves code review, QA testing, and UX review (if UI-facing). Each reviewer produces a PASS/FAIL verdict with evidence.

### Definition of Ready (DoR)

A checklist that must be satisfied before implementation begins. Includes understanding of requirements, documentation approval, and dependency identification.

### Definition of Done (DoD)

A checklist that must be satisfied before a task is marked complete. Includes build verification, test coverage, review gate passage, and documentation updates.

### Scanner

An automated quality check that evaluates the codebase or governance framework against defined standards. Scanners run periodically or on-demand and produce structured pass/fail results. Scanner results are persisted and visualized as trends.

**Examples:** Lint compliance scanner, test coverage scanner, architecture decision compliance scanner.

---

## Learning Loop Concepts

### Lesson (IMPL Entry)

A documented implementation-level learning captured when something goes wrong or a non-obvious pattern is discovered. Tracked in `development/lessons.md` with a recurrence count and promotion status. Format: `IMPL-NNN`.

### Retrospective (RETRO Entry)

A documented process-level learning that records a problem, root cause, corrective action, and outcome. Retrospectives inform governance evolution. Format: `RETRO-NNN`.

### Promotion

The act of elevating a lesson or pattern into a permanent governance artifact. When an IMPL entry recurs (count >= 2), it is promoted to a rule, coding standard, or skill update. Promotion closes the learning loop.

### Cross-Project Learning

The mechanism by which lessons learned in one project are made available to other projects. Lessons can be promoted from project-local to global scope. When onboarding a new project, Orqa Studio consults global lessons for relevant patterns.

### Global Lesson

A lesson that has been promoted from project scope to global scope. Global lessons apply across all projects managed by Orqa Studio. Example: "Always use constant-time comparison for password hashing" is universally applicable regardless of project.

### Metric (KPI)

A quantitative measurement of process health. Metrics include review failure rate, lesson promotion rate, DoR/DoD compliance, context window incidents, and build verification pass rate. Metrics are tracked over time and visualized in the dashboard.

---

## Architecture Concepts

### Sidecar

A standalone process that handles communication with an AI provider. The sidecar is a Bun-compiled TypeScript binary that Orqa Studio spawns via `tauri-plugin-shell`. It communicates with the Rust backend over stdin/stdout using NDJSON. The sidecar is provider-specific; the Rust core is provider-agnostic.

### Model

A specific AI model used for a conversation session (e.g., claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5). Users select a model per-session via a dropdown in the session header. The special value `"auto"` delegates model selection to the provider, which chooses the best available model based on current rate limits and availability — inspired by Claude Code's default behavior. When auto is active, the status bar displays the resolved model (e.g., "Auto → Sonnet 4.6") so the user always knows which model is actually responding.

### Auto Model Selection

Provider-managed model routing. When a session's model is set to `"auto"`, the sidecar delegates model selection to the provider SDK rather than specifying a model explicitly. The provider chooses based on current rate limits, availability, and subscription tier. Auto is the default when the provider supports it. Not all providers support auto — the provider interface exposes a `supports_auto_model` capability flag. If the provider does not support auto, the user must select a specific model.

### Provider

An AI service that powers conversations. Claude (via Agent SDK + Max subscription) is the primary provider. The architecture supports additional providers (API key, Bedrock, Vertex, alternative models) through the composable sidecar interface, without changing the Rust core or Svelte UI.

### ProviderEvent

The provider-agnostic event protocol that the Rust core understands. All sidecar providers translate their native events into ProviderEvents. Types include: StreamStart, TextDelta, ToolCall, ToolResult, StreamEnd, Error, Usage.

### Agent SDK

The official Anthropic SDK (`@anthropic-ai/claude-agent-sdk`) that spawns the Claude Code CLI as a subprocess. Supports `tools: []` to disable built-in tools, `canUseTool` for approval flows, `includePartialMessages` for token-level streaming, and custom MCP server configuration. The primary integration path for Claude Max subscriptions.

### MCP (Model Context Protocol)

A protocol for connecting AI models to external tools and data sources. Orqa Studio exposes its own tools as a custom MCP server to the Agent SDK. Orqa Studio also acts as an MCP host, allowing users to connect additional MCP servers for extensibility.

### Channel\<T\>

Tauri's streaming mechanism for sending data from the Rust backend to the Svelte frontend. Used for streaming conversation tokens. Complements `invoke()` (used for CRUD operations) and events (used for notifications).

### IPC Boundary

The Tauri command interface — the only communication channel between the Rust backend and Svelte frontend. All commands are defined as `#[tauri::command]` functions. No HTTP servers, no WebSocket servers, no shared memory.

### NDJSON

Newline-Delimited JSON — the wire format for sidecar ↔ Rust communication over stdin/stdout. Each line is a complete JSON object. Same pattern as LSP. Sub-millisecond latency with no port management.

---

## Persistence Concepts

### Hybrid File/DB Model

The persistence strategy where governance artifacts are stored as files on disk (authoritative, git-committed) and indexed in SQLite (derived cache for search and metadata). File changes are detected by a file watcher and trigger re-indexing.

### FTS5

SQLite's full-text search extension. Orqa Studio uses two FTS5 virtual tables: one for session message search, one for governance artifact search. Provides sub-50ms search across all content.

### Handoff Notes

Session continuity data that summarizes what happened in a session and what the next session should know. Rule-based in Phase 1, Claude-generated in Phase 2. Enables context transfer across sessions.

---

## UI Concepts

### Panel

A resizable section of the Orqa Studio window. PaneForge manages three resizable zones: the Nav Sub-Panel (per-category sub-navigation), the Explorer Panel (artifact viewer/editor, dashboards, settings), and the Chat Panel (conversation). The Activity Bar (48px fixed icon rail) sits outside PaneForge and controls what both the Nav Sub-Panel and Explorer Panel display.

### Project Dashboard

A dedicated Explorer Panel view showing project overview information: detected stack, project root path, governance artifact counts (agents, rules, skills, hooks), quick links to scanner status, metrics, and learning, and recent sessions with quick-resume links. Accessed via the top icon in the Activity Bar (`Ctrl+0`). When active, the Nav Sub-Panel is hidden.

### Activity Bar

A fixed 48px vertical icon rail on the far left of the window. The top icon is Project Dashboard (`Ctrl+0`). Below it are artifact categories (Docs, Agents, Rules, Skills, Hooks), then dashboards (Scanners, Metrics, Learning), and Settings at the bottom. Clicking an icon switches the Explorer Panel and Nav Sub-Panel to the corresponding view. The Activity Bar is always visible and not collapsible.

### Explorer Panel

The central content area that displays artifacts, dashboards, or settings based on the active Activity Bar icon. When an artifact category is active and an artifact is selected in the Nav Sub-Panel, shows the artifact's viewer/editor. If no artifact is selected (or the Nav Sub-Panel is collapsed), shows the artifact list as a fallback. When a dashboard icon is active, shows that dashboard. The Explorer Panel is always visible and never collapses — it is the focal point of the artifact-centric layout.

### Nav Sub-Panel

A collapsible 200px panel between the Activity Bar and Explorer Panel. Provides per-category sub-navigation: a structured doc tree for the Docs category, flat/categorized lists for Agents, Rules, Skills, and Hooks. Hidden when Project Dashboard, Scanners, or Metrics is active. Toggle with `Ctrl+B`. Collapse range: 200px default, 160px minimum, 280px maximum, collapses to 0px.

### Session Dropdown

A dropdown control in the Chat Panel header for switching between sessions. Shows the active session title (clickable to open), a list of recent sessions with search filter, and a "New Session" button. Replaces the dedicated Sessions Panel from the previous four-zone layout (AD-018). `Ctrl+N` creates a new session directly.

### Chat Panel

The rightmost zone, always visible. Shows the active conversation session — message stream, streaming tokens, tool call cards, and message input. The conversation stays visible while browsing artifacts in the Explorer Panel, enabling side-by-side collaboration.

### Artifact Browser

The combined Nav Sub-Panel + Explorer Panel view for browsing governance artifacts. The active category is determined by the Activity Bar icon (Docs, Agents, Rules, Skills, or Hooks). The Nav Sub-Panel shows the artifact list or tree navigation; the Explorer Panel renders the selected artifact's markdown content in the viewer and provides source editing via CodeMirror 6.

### Streaming Display

The real-time rendering of AI response tokens as they arrive. Tokens flow through the pipeline: Agent SDK → sidecar → NDJSON → Rust → Channel\<T\> → Svelte `$state` → DOM update. Each token triggers a fine-grained reactive update.

### Tool Call Card

A UI component that displays a tool invocation within a conversation: the tool name, input parameters, and result. Tool call cards are collapsible and will support approval/denial flows in Phase 2.

### Feature Gate

A row in the SQLite `feature_gates` table that controls whether a feature is visible to the user. Used for progressive disclosure — features appear as they become relevant, avoiding upfront complexity.

---

## Roles

### Product Manager (PM)

A human user who defines product requirements, reviews implementation plans, and manages the governance framework. The primary persona for Orqa Studio. May also fill the Tech Lead role.

### Tech Lead

A human user who approves implementation plans, reviews architecture decisions, and has final authority on technical approach. May also fill the PM role. Implementation plans require Tech Lead approval before coding begins.

### Developer

A human user who uses Orqa Studio for structured, repeatable AI-assisted development. The secondary persona — benefits from governance visibility but may not define the governance framework themselves.

### Orchestrator

The coordinating intelligence in the agentic team. In the bootstrap process, this is the Claude Code main session. In Orqa Studio, this becomes the Rust backend + sidecar system. The orchestrator delegates implementation to agents and gates completion on review.

---

## Related Documents

- [Product Vision](/product/vision) — Problem statement, solution, pillars
- [Product Governance](/product/governance) — Two-Pillar Test, foundational principles
- [Architecture Decisions](/architecture/decisions) — Formal technical decisions
- [Orchestration](/process/orchestration) — Current (bootstrap) process model
