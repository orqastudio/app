---
id: DOC-044
title: Roadmap
description: Product roadmap organized by milestones with prioritized epics and future ideas.
created: "2026-03-02"
updated: "2026-03-07"
---

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

This roadmap is structured around two milestones and a feature enhancement backlog:

1. **Milestone 1: Dogfooding** ([MS-001](MS-001)) — OrqaStudio is usable as a daily workspace for its own development
2. **Milestone 2: MVP** ([MS-002](MS-002)) — Someone else can use OrqaStudio to fulfil the mission statement
3. **Feature Enhancements** — Future ideas ([IDEA-001](IDEA-001) through [IDEA-022](IDEA-022)) that enhance the platform beyond MVP

Items within milestones are prioritised using the framework defined below. All items are tracked as first-class artifacts in `.orqa/` — see [Artifact Framework](artifact-framework.md) for schemas and structure.

---

## Prioritisation Framework

Every backlog item is scored on four dimensions. The composite score determines ordering within milestones.

| Dimension | Scale | Weight | What It Measures |
|-----------|-------|--------|-----------------|
| **Pillar Alignment** | 0-5 | x3 | How strongly this serves Clarity Through Structure and/or Learning Through Reflection. 0=neither, 3=one pillar, 5=both pillars. |
| **User Impact** | 1-5 | x2 | How much this changes the daily experience for the target user. |
| **Dependency** | 1-3 | x3 | How many other items are blocked by this. 1=independent, 2=some dependents, 3=critical path. |
| **Effort** | 1-5 | divisor | Implementation size. Larger effort = lower score. Prevents big items from always winning. |

**Score = (Pillar x 3 + Impact x 2 + Dependency x 3) / Effort**

Priority bands derived from score:

| Band | Meaning |
|------|---------|
| **P1** | Critical path — milestone blocked without this |
| **P2** | High value — significant quality improvement |
| **P3** | Nice to have — improves experience but not blocking |

This framework will be built into OrqaStudio itself as an MVP feature (see Milestone 2), enabling structured prioritisation for any project.

---

## Completed Work

### Milestone 0: Foundation & Scaffold ([MS-000](MS-000)) — Complete

All pre-build phases are complete. Each phase is tracked as a retroactive epic for full traceability.

- [x] **[EPIC-025](EPIC-025): Phase 0a — Tech Stack Research** — AI provider integration research (Agent SDK sidecar pattern), Tauri v2 capability audit, frontend library selection, persistence design, onboarding strategy. All findings documented in `.orqa/research/mvp/`.
- [x] **[EPIC-026](EPIC-026): Phase 0b — Architecture Decisions** — [AD-001](AD-001) through [AD-020](AD-020) recorded. Covers thick backend architecture, IPC boundary, error propagation, Svelte 5 runes, SQLite persistence, component purity, sidecar integration, streaming pipeline, security model, MCP host, persistence, governance format, composability principle, layout decisions, and documentation browsing model. See `docs/architecture/decisions.md` for the index.
- [x] **[EPIC-027](EPIC-027): Phase 0c — Product Definition** — Glossary, personas, user journeys, information architecture, MVP feature specification. All in `docs/product/`.
- [x] **[EPIC-028](EPIC-028): Phase 0d — UX Design** — Design system, wireframes (core layout, conversation, artifact browser, settings/onboarding, dashboard), component inventory, interaction patterns, responsive behaviour. All in `docs/ui/`.
- [x] **[EPIC-029](EPIC-029): Phase 0e — Technical Design** — SQLite schema, IPC command catalogue, Rust module architecture, Svelte component tree, streaming pipeline, tool definitions, MCP host interface, error taxonomy. All in `docs/architecture/`.
- [x] **[EPIC-030](EPIC-030): Phase 1 — Scaffold** — Working Tauri v2 app with AI conversations via Agent SDK sidecar. Includes:
  - Tauri v2 + Svelte 5 project with configured plugins
  - Rust backend: Agent SDK sidecar with streaming (Channel<T>)
  - Rust backend: SQLite with schema + migrations
  - Rust backend: Session and message CRUD
  - Rust backend: 40+ IPC commands across 8 domains
  - Frontend: Four-zone layout, conversation UI, streaming token display
  - Frontend: Tool call rendering (collapsible cards with input/output)
  - Frontend: Session dropdown with history, search, and navigation
  - Frontend: Settings view (provider config, model selection)
  - Semantic code search: ONNX embeddings + DuckDB vector search
  - Integration: Send message → stream response → render in UI
- [x] **[EPIC-031](EPIC-031): Phase 2b — Governance Bootstrap** — Governance scanner, analysis, recommendation UI, coverage indicator. Includes:
  - Governance scanner — filesystem walk to collect .claude/ and other governance files
  - Governance analysis with recommendations
  - Recommendation review and approval UI
  - Governance coverage indicator on dashboard

### Additional Completed Features (verified by audit)

- [x] **Setup wizard** — Full 5-step wizard (CLI detection, auth check, sidecar startup, embedding model, completion)
- [x] **Session persistence** — Sessions and messages persisted to SQLite with full CRUD
- [x] **Session resume (happy path)** — SDK provider_session_id persisted and passed on resume
- [x] **Session history UI** — Dropdown with search, status badges, message preview, delete
- [x] **Session history search** — FTS5 full-text search across all project messages
- [x] **File tools** — Read, Write, Edit, Glob, Grep, Bash execution engine with path resolution
- [x] **Tool approval flow** — End-to-end approve/deny dialog for write/execute tools
- [x] **Lesson management** — Full CRUD, auto-assigned IDs, recurrence tracking, promotion scanning, UI components
- [x] **Rule enforcement engine** — Runtime enforcement integrated into file operations
- [x] **Governance artifact browsing** — Read-only viewers for agents, rules, skills, hooks with sidebar navigation
- [x] **Provider abstraction** — Provider-agnostic protocol, factory pattern, zero Claude-specific leaks
- [x] **AI Transparency types** — StreamEvent types (SystemPromptSent, ContextInjected) defined in Rust + TypeScript
- [x] **AI Transparency components** — ContextEntry, ContextDetailDialog, ThinkingBlock — all production-ready
- [x] **Thinking accumulation** — Store accumulates thinking deltas during streaming
- [x] **Project settings fields** — show_thinking, custom_system_prompt, enable_thinking in sidecar protocol

---

## Milestone 1: Dogfooding

**Goal:** OrqaStudio is usable as a daily workspace for building OrqaStudio itself.

**Gate question:** *"Can we use this app instead of the terminal for governance management, conversation debugging, and structured thinking about the project?"*

**Dogfooding context:** OrqaStudio is developed using itself (`.orqa/project.json` has `dogfood: true`). The app runs with `--no-watch` so editing Rust files doesn't kill the active session. Frontend changes hot-reload via Vite HMR. Rust changes require manual restart.

### P1 — Critical Path

These items block dogfooding. Each represents a gap between "the pieces exist" and "it actually works end-to-end."

#### D1. AI Transparency Wiring — [EPIC-001](EPIC-001) (Done)

The types, components, and store handling all exist. The emission logic now connects them.

- [x] Emit `SystemPromptSent` event from `stream_commands.rs` ([TASK-001](TASK-001))
- [x] Verify end-to-end rendering pipeline ([TASK-002](TASK-002))
- [x] Update streaming pipeline documentation ([TASK-003](TASK-003))
- [x] Emit `ContextInjected` event when prior messages exist in session ([TASK-004](TASK-004))

**Completed:** `SystemPromptSent` emitted before sidecar request. Full pipeline verified: Rust emission → Channel<T> → store accumulation → ContextEntry + ContextDetailDialog rendering. ThinkingBlock already wired. `ContextInjected` emission deferred to [EPIC-003](EPIC-003).

#### D2. Settings UI for Thinking & Custom Prompt

Fields exist in both Rust and TypeScript. Missing: UI controls to set them.

- [ ] Add `show_thinking` toggle in Settings > Model section
- [ ] Add `custom_system_prompt` textarea in Settings > Project section
- [ ] Add "View auto-generated prompt" collapsible preview (reuse `build_system_prompt()`)

**Why P1:** Can't control reasoning behaviour without these toggles.

#### D3. Context Injection on Failed Resume

SDK session resume works on the happy path. Missing: fallback when SDK state is lost (app restart, cleared storage).

- [ ] Detect SDK resume failure in sidecar (returned session_id !== passed session_id)
- [ ] Sidecar emits `context_needed` event to Rust
- [ ] Rust loads last ~20 text messages from SQLite for the session
- [ ] Rust sends `context_history` to sidecar for injection
- [ ] Rust emits `ContextInjected` event for transparency (D1)

**Why P1:** Can't restart the app during development without losing conversation context.

#### D4. Artifact Editing UI

Backend CRUD exists (`artifact_create`, `artifact_update`, `artifact_delete`). Read-only viewers exist. Missing: an editor component.

- [ ] CodeMirror 6 editor component for markdown/YAML editing
- [ ] Edit mode toggle on artifact viewers (view → edit)
- [ ] Create new artifact from template (agents, rules, skills, hooks)
- [ ] Delete artifact with confirmation dialog
- [ ] Wire artifact store methods to backend CRUD commands

**Why P1:** Can't manage governance in-app without editing. Currently requires switching to a text editor.

#### D5. Artifact Browser & Visibility

Make `.orqa/` artifacts (milestones, epics, ideas, plans, research, lessons) browsable as rendered markdown documents in OrqaStudio's UI. This is the **underlying UX model** — all richer views are optional layers built on top.

- [ ] `.orqa/` directory scanner — read and parse all artifact types
- [ ] Frontmatter parser — extract YAML frontmatter into structured data
- [ ] Artifact browser sidebar — tree navigation by type
- [ ] Markdown renderer view — render artifact body with syntax highlighting
- [ ] Frontmatter metadata panel — display structured metadata alongside the document
- [ ] Connection links — clickable references to related artifacts
- [ ] Status badges and priority band indicators

**Why P1:** Structured thinking artifacts must be visible in the app. Markdown-first visibility is the foundational UX layer that all other views build upon.

### P2 — Enablers

These significantly improve dogfooding quality but aren't strict blockers.

#### D6. File Watcher for External Changes

- [ ] Add `notify` crate for filesystem watching
- [ ] Watch `.claude/` and `.orqa/` directories for external modifications
- [ ] Refresh artifact list and viewer when files change on disk
- [ ] Debounce rapid changes (editor auto-save)

**Why P2:** Without this, CLI edits to governance files aren't reflected in the app until manual refresh.

#### D7. Composability Refactoring

The codebase has solid boundaries (stores, IPC, components) but monolithic service files.

- [ ] Extract `StreamOrchestrator` from `stream_commands.rs` (2,232 lines → command handlers + orchestrator service)
- [ ] Implement `Tool` trait and `ToolRegistry` from `tool_executor.rs` (966 lines → pluggable tools)
- [ ] Decompose `ConversationView.svelte` (367 lines) into smaller composable units if complexity grows
- [ ] Service layer for enforcement, governance, and scanning (wrap procedural code in structs)

**Why P2:** Code health for sustained velocity. Functions individually pass size limits, but file-level cohesion is poor.

#### D8. Code Quality Audit

- [ ] Coding standards compliance audit against `docs/development/coding-standards.md`
- [ ] Enforcement artifact review — rules/hooks/skills completeness
- [ ] Abstraction pattern audit — identify over-complicated patterns from iterative development
- [ ] Fix function size violations in `tool_executor.rs` (`tool_bash` 97 lines, `execute_tool` 69 lines, `project_root_from_state` 152 lines)

**Why P2:** Can't credibly enforce quality on managed projects if our own code has violations.

#### D9. Frontend Test Suite

- [ ] Vitest setup for Svelte component and store testing
- [ ] Store unit tests (conversation, session, project, settings — state transitions, reactive updates)
- [ ] Component tests for critical UI (ConversationView, ToolApprovalDialog, SessionDropdown)
- [ ] IPC contract tests — verify invoke calls match actual Tauri commands

**Why P2:** 465 Rust tests exist but zero frontend tests. Changes to stores break components silently.

### P3 — Polish

#### D10. Developer Experience

- [ ] Project-local database — move SQLite from `app_data_dir` to `.orqa/orqa.db` so session history travels with the project
- [ ] Build splash window — small branded window during `make dev` compilation
- [ ] Custom system prompt templates — pre-built prompts for common scenarios (dogfooding, greenfield, legacy)

---

## Milestone 2: MVP

**Goal:** Someone other than the developer can use OrqaStudio to fulfil the mission statement — both a technical PM managing a software project and someone doing non-software structured thinking.

**Gate question:** *"Can a new user install this, open a project (or create one), and get value from AI-assisted structured thinking within 10 minutes?"*

**Prerequisites:** Milestone 1 complete. The app is stable enough for daily use by its own team.

### P1 — Core Experience

Minimum for someone else to get value from OrqaStudio.

#### M1. Chat-Guided Onboarding

No directed onboarding flows needed — the chat itself guides new users through setup.

- [ ] System prompt includes project state awareness (what artifacts exist, what's configured, what's missing)
- [ ] AI suggests next steps when project is new or incomplete ("I notice you don't have any rules defined yet — would you like to...")
- [ ] First-conversation guidance — when no sessions exist, AI introduces OrqaStudio's capabilities naturally
- [ ] Entry mode detection — AI identifies which mode (Problem, Idea, Goal, Chaos) fits the user's opening message and adapts accordingly

**Why P1:** The conversation IS the onboarding. Without this, new users don't know what to do.

#### M2. Process Visibility Dashboard

Governance coverage exists. Missing: richer process visibility.

- [ ] Scanner runner — execute code quality checks (clippy, eslint, tests) and collect results
- [ ] Scanner dashboard — pass/fail history, violation details, trend charts (LayerChart)
- [ ] Metrics dashboard — KPI cards for key project health indicators
- [ ] Agent activity panel — which agent is active, what tools it's using, current task

**Why P1:** Pillar 1 (Clarity Through Structure) — governance must be visible, not buried in terminal output.

#### M3. Learning Through Reflection

Lesson management exists (CRUD, UI). Missing: the automated capture and promotion pipeline.

- [ ] Post-session hooks that capture lessons automatically to `.orqa/lessons/`
- [ ] Rules enforcing lesson checking before implementation
- [ ] Automated promotion suggestions when recurrence >= threshold
- [ ] Lesson dashboard with recurrence trends (LayerChart)
- [ ] Session analytics — pass/fail rates, coverage trends

**Why P1:** Pillar 2 (Learning Through Reflection) — the system must get smarter with each cycle.

#### M4. Project Type System

OrqaStudio's governance applies to any project, not just software. The app needs to know what kind of project it's managing.

- [ ] `project_type` field in `.orqa/project.json` and `ProjectSettings` — Software, Research, Product, Personal, Custom
- [ ] Project type determines which agents, tools, and scanning are available
- [ ] Software projects get code tools (ChunkHound, file tools, git); others get domain-appropriate tooling
- [ ] Non-software projects work without requiring a codebase
- [ ] Project type selector in project creation/settings

**Why P1:** Without this, non-software users encounter software-specific features that don't apply to them.

#### M5. CI/CD Pipeline & Distribution

Must be distributable for anyone else to use it.

- [ ] GitHub Actions: PR checks (`make check` on all platforms)
- [ ] GitHub Actions: build artifacts on merge to main (pre-release)
- [ ] GitHub Actions: build release on tag push (stable)
- [ ] Platform matrix: Windows (x64), macOS (x64, arm64), Linux (x64)
- [ ] Artifact signing with Tauri updater keys
- [ ] Semantic versioning: tauri.conf.json + Cargo.toml + package.json sync
- [ ] Auto-update via `tauri-plugin-updater` with GitHub Releases
- [ ] Update channel selector in Settings (pre-release / stable)

**Why P1:** Can't ship to users without a build pipeline and update mechanism.

#### M6. Prioritisation Framework

The scoring model defined above, built into the app.

- [ ] Priority dimensions — user configures dimensions and weights in `.orqa/project.json`
- [ ] Default dimensions: Impact (x2), Effort (x2, inverted), Urgency (x2), Pillar Alignment (x3)
- [ ] Composite priority score — weighted sum produces comparable number across all item types
- [ ] Priority bands — score ranges map to P0-P4 labels
- [ ] Auto-scoring — suggest dimension scores based on frontmatter when items are created
- [ ] Manual override with tracking
- [ ] Priority views — backlog sorted by composite score, filterable by band
- [ ] Custom dimensions — users add project-specific scoring dimensions

**Why P1:** OrqaStudio is a structured thinking tool. Prioritisation is a core structured thinking capability.

### P2 — Depth

Richer experience and broader use cases.

#### M7. Enforcement & Continuity

- [ ] Hooks that inject relevant rules into conversations based on file context
- [ ] Real-time violation detection during streaming (pattern matching on streamed tokens)
- [ ] Visual compliance dashboard
- [ ] Session handoff and continuity — cross-session search, handoff summaries

#### M8. Sub-Agent Support

- [ ] Agent registry — reads `.orqa/agents/*.md`, indexes capabilities
- [ ] `spawn_agent` tool — spawns sub-agent with role and instructions
- [ ] Explore mode — lightweight codebase exploration agent (no tool approval)
- [ ] Output aggregation — child tool calls collected, summary card with expandable detail
- [ ] Turn limits — configurable max turns per sub-agent invocation

#### M9. MCP Host — External Servers

- [ ] MCP host module — JSON-RPC protocol handler, connection state machine
- [ ] stdio transport — spawn external MCP servers, process lifecycle management
- [ ] SSE transport — HTTP client for remote MCP servers
- [ ] Config loader — merge project + user MCP server configs
- [ ] Tool aggregator — merge built-in + external tools, namespace external tools
- [ ] MCP Servers section in Settings — server list, add/remove, test connection, trust levels

#### M10. Discovery & Research Artifacts

- [ ] Research artifact type — structured, queryable, filterable
- [ ] Decision traceability graph (research → AD → feature → implementation)
- [ ] Research-to-AD promotion workflow
- [ ] Discovery dashboard — open questions, pending decisions
- [ ] Conversational research workflow — Claude-assisted investigation producing structured artifacts

#### M11. Idea & Feedback Capture

- [ ] Idea artifact type in `.orqa/ideas/` with frontmatter
- [ ] Quick-capture from conversation — slash command or highlight to create idea
- [ ] Quick-capture from anywhere — global shortcut or status bar button
- [ ] Idea inbox — uncategorised ideas, sortable
- [ ] Idea-to-research and idea-to-plan promotion
- [ ] Duplicate detection via FTS5 + semantic search

#### M12. Implementation Breakdown & Work Management

- [ ] Epic → Backlog Item → Task hierarchy
- [ ] Plan-to-backlog breakdown with Claude assistance
- [ ] Unified backlog view — bugs and features together, filterable, sortable
- [ ] Status workflow — draft → ready → in-progress → review → done
- [ ] Bug artifact type in `.orqa/bugs/` with reproduction steps
- [ ] Task-to-agent assignment and worktree branch generation
- [ ] Progress dashboard — plan completion %, epic progress

#### M13. Onboarding Flow Review

- [ ] Audit first-run setup wizard — verify each step is clear and skippable
- [ ] New Project flow — scaffold sensible project structure with optional templates
- [ ] Initialize Existing Folder — improve scan results presentation
- [ ] Guided first conversation — suggest first task after project setup
- [ ] Open Project validation — graceful handling of corrupted `.orqa/`

### P3 — Nice to Have for Launch

#### M14. Additional Polish

- [ ] Semantic search index management UI — trigger re-index, show status, configure patterns
- [ ] Multi-window / multi-project — system tray, per-window sidecar lifecycle
- [ ] Composability Gate phases C+D — composable learning loop, initialization assessment
- [ ] Sprint/iteration planning — optional timeboxing, burndown
- [ ] Dependency tracking between backlog items

---

## Milestone 3: Feature Enhancements

Future ideas that enhance OrqaStudio beyond MVP. Grouped thematically. Not prioritised — these are candidates for future milestone planning once MVP ships.

### Multi-Provider Ecosystem

The provider-agnostic sidecar interface supports additional providers without changing the Rust core or Svelte UI.

- [ ] Third-party AI cloud provider research — OpenRouter, Together AI, Fireworks, Replicate
- [ ] Direct Anthropic API provider — Rust-native HTTP, pay-per-token
- [ ] Direct OpenAI-compatible API provider — OpenAI, Azure OpenAI, compatible endpoints
- [ ] Gemini Developer API provider
- [ ] OpenAI Agents SDK sidecar — second agent runtime
- [ ] Google ADK sidecar — third agent runtime
- [ ] Ollama / local LLM provider — offline/air-gapped use
- [ ] Budget & billing prediction — usage tracking and cost prediction
- [ ] Multi-provider cost optimisation — route work to cheapest capable provider
- [ ] Provider selection in project config

### Transportable Governance Format

Extend the existing `.orqa/`-canonical governance model with a full suite of CLI tool adapters, making governance portable across all AI tooling environments without duplication.

- [ ] Adapter framework — standardised interface for mapping `.orqa/` artifacts to tool-specific layouts
- [ ] Claude Code adapter — `.claude/` symlinks to `.orqa/` content (partially done; formalise as a named adapter)
- [ ] Continue adapter — `.continue/` directory adapter
- [ ] Cursor adapter — `.cursor/` directory adapter
- [ ] Other environment adapters — pattern for any AI tool with directory-based config
- [ ] Adapter management UI — enable/disable adapters per project in Settings
- [ ] Migration tooling — import existing `.claude/` or `.cursor/` artifacts into `.orqa/` with adapter enabled

### Entry Modes & Directed Onboarding

AI-assisted onboarding flows beyond chat guidance. Each mode supports new projects and existing work adaptation.

- [ ] Problem mode — guided diagnosis flow with root cause mapping
- [ ] Idea mode — validation flow with feasibility exploration
- [ ] Goal mode — planning flow with gap analysis
- [ ] Chaos mode — triage flow with situation mapping
- [ ] Existing work assessment — AI scans and assesses current state
- [ ] Mode convergence — all paths produce same artifact structure
- [ ] Domain-agnostic templates — personal planning, healthcare, research, consulting
- [ ] Mode detection — AI suggests most appropriate entry mode

### Structured Thinking Tools

Tools that operationalise the agile learning loop beyond conversation.

- [ ] Decision matrices — structured comparison with weighted criteria
- [ ] Experiment design — hypothesis → test → measure → learn templates
- [ ] Assumption mapping — surface and track assumptions
- [ ] Impact/effort prioritisation — visual prioritisation with AI-suggested rankings
- [ ] Dependency mapping — visualise relationships between tasks and decisions
- [ ] Risk registers — structured risk identification and mitigation
- [ ] Stakeholder mapping — who cares about what

### Knowledge & Artifact Intelligence

Making the accumulated knowledge base actively useful.

- [ ] Semantic search across artifacts — search by meaning, not just keywords
- [ ] Cross-artifact linking — automatic relationship detection
- [ ] Knowledge graph visualisation — visual map of artifact connections
- [ ] Artifact staleness detection — flag stale artifacts
- [ ] AI-assisted artifact summarisation
- [ ] Contradiction detection — surface conflicting information

### Cross-Project Intelligence

Learning that transfers between projects.

- [ ] Project templates — pre-configured governance for common project types
- [ ] Cross-project pattern detection — recurring lessons suggest universal rules
- [ ] Shared governance libraries — publish and import governance artifacts
- [ ] Organisation-level learning — aggregate metrics across projects
- [ ] Benchmarking — compare project health against averages

### Spaces & Organisational Structure

Containers for organising projects as adoption scales.

- [ ] Spaces — named containers (organisations, teams, programs) grouping projects
- [ ] Space-level governance — shared rules across projects
- [ ] Space-level insights — aggregated metrics and learning
- [ ] Space hierarchy — nested spaces for complex structures

### Collaboration & Teams

Multi-user support for the clarity engine.

- [ ] Shared projects — multiple users viewing and contributing
- [ ] Role-based access — viewer, contributor, approver per project
- [ ] Review workflows — artifact review with audit trails
- [ ] Handoff protocols — structured handoff with context preservation
- [ ] Activity feeds — what changed, who, why
- [ ] Conflict resolution — structured merge with AI assistance

### Integration Ecosystem

Connecting OrqaStudio with external tools and workflows.

- [ ] Git integration — branch awareness, commit correlation with decisions
- [ ] Issue tracker sync — bidirectional with GitHub Issues, Linear, Jira
- [ ] Documentation platform export — Notion, Confluence, static sites
- [ ] Calendar integration — experiment timelines and retrospective schedules
- [ ] Notification channels — Slack/Teams/email for artifact changes
- [ ] CI/CD integration — pull quality gate results into metrics
- [ ] Import from existing tools — Notion, Obsidian, markdown repos

### Domain Expansion

Extending beyond software development.

- [ ] Research domain — hypothesis tracking, literature review, experiment logs
- [ ] Product management domain — feature prioritisation, user research synthesis
- [ ] Operations domain — process documentation, incident retrospectives, SOP management
- [ ] Personal productivity domain — goal setting, habit tracking, reflection journals
- [ ] Consulting domain — client engagement structure, deliverable tracking
- [ ] Education domain — learning path design, curriculum planning

### Reporting & Stakeholder Access

Making understanding accessible to non-users.

- [ ] Executive dashboards — project health, progress, risk summaries
- [ ] Shareable artifact links — read-only access
- [ ] PDF/HTML export — formatted reports from artifact collections
- [ ] Progress reports — auto-generated periodic summaries
- [ ] Audit trails — decision history for compliance

### Session Intelligence

Making AI conversations smarter over time.

- [ ] Conversation summarisation — auto-generate session summaries
- [ ] Context-aware suggestions — AI suggests relevant artifacts during conversation
- [ ] Session branching — fork conversations to explore alternatives
- [ ] Conversation replay — step through past sessions
- [ ] Proactive learning — surface past lessons before user hits same problem

### Reasoning Transparency

Maintaining clear reasoning trails.

- [ ] Reasoning summaries — AI-generated summaries of how conclusions were reached
- [ ] Change justification notes — structured explanations on artifact changes
- [ ] Confidence indicators — AI signals uncertainty in recommendations
- [ ] Decision logs — chronological record with context and rationale

### Chaos-to-Clarity Engine

Dedicated workflows for transforming unstructured situations.

- [ ] Cognitive unloading interfaces — structured brain-dump flows
- [ ] AI-driven theme detection — automatic pattern identification in unstructured input
- [ ] Situation mapping — visual representation with relationships
- [ ] Pathway recommendation — AI suggests promising paths forward

### Multi-View Output System

Expanding artifact projections beyond backlogs.

- [ ] Strategy maps — visual connections to strategic objectives
- [ ] Experiment frameworks — structured hypothesis → learn cycle views
- [ ] Stakeholder systems diagrams — visual maps of stakeholder relationships
- [ ] Communication summaries — audience-tailored summaries from same artifacts

### Reflective AI Facilitation

Active structured reflection support.

- [ ] AI retrospective facilitator — guided sessions with prompts from project history
- [ ] Assumption challenge prompts — proactive assumption surfacing
- [ ] Insight synthesis — cross-session learning analysis
- [ ] Learning cycle detection — identify recurring mistakes or patterns

### Institutional Memory

Long-term knowledge preservation.

- [ ] Historical project evolution maps — timeline of understanding evolution
- [ ] Organisational learning records — lessons that outlive projects
- [ ] Strategic timeline visualisations — strategy shifts with decision context

### Output Composability

New artifact generation modules for different domains.

- [ ] Service design maps
- [ ] Policy development frameworks
- [ ] Innovation pipelines — idea → validation → development → launch
- [ ] Research planning systems — literature review, methodology, findings

### Ecosystem Potential

Long-term platform potential.

- [ ] Ethical AI capability platform — transparent, auditable reasoning
- [ ] Open reasoning framework — open-source core reasoning patterns
- [ ] Community innovation infrastructure — shared structured thinking spaces
- [ ] Tools for charities and social organisations

### Design Tool Integration

Integration with 3rd-party design tools.

- [ ] Figma integration — import design tokens, Figma MCP server
- [ ] Design token sync — bidirectional between project tokens and design tools
- [ ] Code-to-Figma backfill — generate Figma components from existing frontend
- [ ] Designer persona — Designer role in governance framework

### Developer Experience (Dogfooding)

DX improvements discovered through self-use.

- [ ] Wireframe browser — browsable, interactive wireframe artifacts
- [ ] Custom system prompt templates — pre-built prompts for common scenarios
- [ ] Dev environment toolbar — start/stop dev server, build, test from the app
- [ ] Terminal integration — embedded terminal panel
- [ ] Git integration panel — branch management, diff viewer

### Platform Access

Expanding beyond desktop.

- [ ] Web companion — read-only web view for artifacts and dashboards
- [ ] Mobile companion — quick capture and review
- [ ] API access — programmatic access to artifact store and learning loop
- [ ] CLI companion — lightweight command-line for artifact queries
