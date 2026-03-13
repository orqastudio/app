---
id: DOC-041
title: User Journeys
description: User journey maps covering key workflows from project creation through ongoing governance.
created: "2026-03-02"
updated: "2026-03-08"
---

**Date:** 2026-03-02

End-to-end workflows for key scenarios in OrqaStudio™. Each journey describes what the user does, what OrqaStudio does in response, and what the user sees at each step. Journeys are organized by scenario, not by persona — all personas follow the same workflows, but emphasis and entry points differ.

These journeys inform UI design (Phase 0d) and the MVP feature specification.

---

## Journey 1: First-Time Setup

**Trigger:** User launches OrqaStudio for the first time.
**Persona emphasis:** Jordan (needs instant value), Alex (wants to configure immediately).
**MVP scope:** Yes — core path for Phase 1.

### Steps

1. **Launch OrqaStudio** — The main window opens with a welcome state. No project loaded. OrqaStudio checks for Claude Code CLI availability. If found, the status bar shows "CLI: Connected" with the version. If not found, OrqaStudio shows a setup prompt with installation guidance (the rest of the UI remains functional for project browsing and artifact viewing).

2. **Open a project** — User clicks "Open Project" or drags a folder onto the window. OrqaStudio prompts for a directory. User selects their project root.

3. **Automatic codebase scan** — OrqaStudio runs the three-tier scan:
   - **Tier 1 (instant):** Manifest-file heuristics detect languages, frameworks, package managers. Results appear within 100ms.
   - **Tier 2 (~1-2s):** `hyperpolyglot` confirms language detection across the full file tree. Results update the project metadata.
   - **Tier 3 (deferred):** Claude analysis is available on-demand but not triggered automatically on first open.

   The Project Dashboard populates with the project name, detected stack, and a file tree summary.

4. **Detect existing governance** — OrqaStudio checks for `.orqa/` directory (and `.claude/` as a legacy fallback for existing CLI-only projects). If found, governance artifacts are indexed into SQLite and displayed in the artifact browser. If not found, the UI notes "No governance framework detected" and offers to help create one (Journey 2).

5. **Start a conversation** — The conversation input is immediately available. The user types their first message. If no sidecar is configured, OrqaStudio checks for the configured AI provider and spawns the appropriate sidecar.

6. **First response streams** — Tokens stream into the conversation panel in real-time. The user sees the AI responding. Tool calls (if any) appear as collapsible cards.

7. **Session persisted** — The session is automatically saved to SQLite. It appears in the session dropdown in the Chat Panel header. The user can close and reopen OrqaStudio without losing context.

### Success Criteria

- Time from launch to first AI response: < 1 minute (assuming the AI provider is installed and configured)
- No wizard, no multi-step configuration
- Project metadata is populated automatically
- The user can start another session or continue the current one at any time

### Error Paths

- **AI provider not found:** OrqaStudio shows a clear message explaining what provider is required and links to installation or configuration instructions. The rest of the UI (project browsing, artifact viewing) remains functional without AI.
- **Empty project:** Scan results show "No recognized project structure." The conversation still works — the user can describe their project in conversation.
- **Large project (>10,000 files):** Tier 2 scan is throttled. Progress indicator shows scan status. Conversation is available immediately regardless of scan progress.

---

## Journey 2: Define Governance

**Trigger:** User wants to establish or modify their project's governance framework.
**Persona emphasis:** Alex (defines standards upfront), Jordan (builds gradually).
**MVP scope:** Partial — browse and edit existing artifacts in Phase 1. Conversational creation in Phase 4.

### Steps

1. **Open artifact browser** — User clicks an artifact category icon in the Activity Bar. The Explorer Panel shows the artifact list for the selected category.

2. **Browse existing artifacts** — Each artifact shows its name, a brief description (from YAML frontmatter or first paragraph), and status. User clicks to view the full content rendered as markdown with metadata displayed.

3. **Edit an artifact** — User clicks "Edit" on an artifact. The view switches from rendered markdown to CodeMirror 6 source editing. YAML frontmatter is editable. Changes are saved to disk on Ctrl+S / Cmd+S.

4. **Create a new artifact** — User clicks "New Agent" (or Rule, Skill, etc.). OrqaStudio creates a template file in the appropriate `.orqa/` subdirectory with standard frontmatter. The editor opens immediately.

5. **Conversational governance (Phase 4)** — User describes what they want in conversation: "I need a rule that prevents agents from using unwrap in Rust code." The AI generates the rule file, the user reviews and approves it, and OrqaStudio writes it to `.orqa/rules/`. For pattern-matchable violations (e.g., specific strings or regex patterns in file edits or bash commands), the user can also create hookify rules — OrqaStudio generates an `.orqa/process/hooks/hookify.*.local.md` file with the appropriate `event`, `action`, and `conditions` fields.

6. **File watcher sync** — Any changes made to `.claude/` files outside OrqaStudio (e.g., in a text editor, via git pull, or via a Claude Code CLI session) are detected by the file watcher and reflected in the artifact browser within 500ms. This means artifacts edited in OrqaStudio are immediately available to Claude Code CLI sessions, and vice versa.

### Success Criteria

- Artifacts are browsable without opening a text editor
- Editing feels natural (syntax highlighting, markdown preview)
- Changes are reflected immediately in the file system
- External changes are detected and reflected in the UI

---

## Journey 3: Implementation Cycle

**Trigger:** User has a task to implement (feature, bug fix, refactor).
**Persona emphasis:** Alex (manages the cycle), Sam (executes within it), Jordan (runs the full cycle solo).
**MVP scope:** Yes — core conversation + streaming in Phase 1. Tool approval in Phase 2.

### Steps

1. **State the requirement** — User types a requirement in the conversation input: "Add a settings panel with API key entry and model selection." The message is sent to the AI via the sidecar.

2. **AI proposes a plan** — The AI responds with an implementation plan: scope, affected files, approach, testing strategy. The plan streams into the Chat Panel as structured content.

3. **Human reviews and approves** — The user reads the plan. They can:
   - **Approve:** "Looks good, go ahead." The AI proceeds to implementation.
   - **Modify:** "Change the approach to X instead of Y." The AI revises the plan.
   - **Reject:** "Actually, let's do something different." The conversation pivots.

   This is the **Human Approval Gate** — no implementation proceeds without explicit approval.

4. **AI implements** — The AI begins implementation. Tool calls appear as cards in the conversation:
   - File reads (show file path and snippet)
   - File writes/edits (show diff preview)
   - Shell commands (show command and output)
   - Each tool call is collapsible — summary by default, expandable for details.

5. **User monitors** — As the AI works, the user can:
   - Watch streaming token output
   - Expand tool call cards to inspect details
   - Interrupt to ask questions or redirect
   - In Phase 2: approve/deny individual tool calls before execution

6. **AI reports completion** — The AI summarizes what was done: files changed, tests run, issues encountered. The user reviews the summary.

7. **Quality check** — The user can ask the AI to run quality checks: "Run the linter," "Check test coverage," "Verify the build." Results stream into the conversation.

8. **Session saved** — Everything is persisted. The user can return to this session later, search for it by content, or use handoff notes to continue in a new session.

### Success Criteria

- The plan → approve → implement → verify cycle is natural and low-friction
- Tool calls are visible but not overwhelming (collapsed by default)
- The user always has a clear picture of what the AI is doing
- The human approval gate is enforced — no silent implementation

---

## Journey 4: Review and Approve

**Trigger:** AI has completed implementation and the user needs to review.
**Persona emphasis:** Alex (reviews at a strategic level), Sam (reviews at code level).
**MVP scope:** Partial — tool call display in Phase 1. Approval flow in Phase 2.

### Steps

1. **Review tool call history** — The conversation shows a sequence of tool calls made during implementation. Each card shows:
   - Tool name (Read, Write, Edit, Bash, etc.)
   - Input summary (file path, command)
   - Result summary (lines changed, output preview)
   - Timestamp and duration

2. **Inspect specific changes** — User expands a tool call card to see full details:
   - For file edits: unified diff view with syntax highlighting
   - For file writes: full content with syntax highlighting
   - For commands: full output with error highlighting
   - For reads: the content that was read (collapsible for large files)

3. **Ask follow-up questions** — User can ask the AI about specific changes: "Why did you change this function signature?" or "What's the impact of removing this dependency?" The AI responds within the same session context.

4. **Request changes** — If the user isn't satisfied: "Revert the change to config.rs and use the original approach." The AI adjusts. This loops back to the implementation cycle.

5. **Approve** — User confirms the implementation is acceptable. In Phase 2+, this triggers the formal review gate (scanner run, compliance check). In Phase 1, approval is conversational.

### Success Criteria

- Users can understand what changed without reading every line of code
- Diff views are readable and syntax-highlighted
- The conversation context is preserved — asking "why" about a change gets a contextual answer
- Approval/rejection is explicit, not implicit

---

## Journey 5: Learning Loop

**Trigger:** A mistake is made, a pattern is discovered, or the user wants to improve the process.
**Persona emphasis:** Alex (strategic improvement), Sam (tactical improvement), Jordan (safety net).
**MVP scope:** No — Phase 5. Designed here to inform information architecture.

### Steps

1. **Mistake occurs** — During implementation, the AI makes an error that the review catches (or that the user catches). Examples: using unwrap instead of Result, missing a test case, violating an architecture decision.

2. **Capture the lesson** — The user (or the AI, prompted by the user) creates an IMPL entry: what went wrong, what the correct approach is, and what should change to prevent recurrence. The lesson is saved to the lessons log and indexed for search.

3. **Recurrence tracking** — When a similar mistake occurs in a future session, OrqaStudio links it to the existing IMPL entry and increments the recurrence count. The user sees: "This is the 3rd time this pattern has been observed."

4. **Promotion trigger** — When recurrence >= 2 (configurable), OrqaStudio suggests promotion: "This lesson has recurred. Would you like to promote it to a rule?" The user reviews and approves.

5. **Governance artifact created** — OrqaStudio generates a rule file from the lesson and adds it to `.orqa/rules/`. Future sessions enforce the rule automatically. The IMPL entry is marked as "Promoted to rule-xyz." If the violation is pattern-matchable (a specific string or regex in file edits or bash commands), OrqaStudio also offers to create a hookify enforcement rule (`.orqa/process/hooks/hookify.*.local.md`) that blocks or warns on the pattern at the action boundary — providing immediate, automated enforcement alongside the instructional rule.

6. **Cross-project promotion** — When a lesson or promoted rule is broadly applicable (not project-specific), the user can promote it to global scope. OrqaStudio asks: "This rule seems generally useful. Should it apply to all your projects?" If approved, the lesson/rule is stored in the global knowledge base. New projects automatically inherit relevant global rules during onboarding (Journey 7).

7. **Metrics update** — The lesson promotion rate metric is updated. The dashboard shows the trend: lessons captured, lessons promoted, recurrence rates.

### Success Criteria

- Capturing a lesson takes < 30 seconds
- Recurrence is detected automatically (not manually tagged)
- Promotion to governance artifact is a one-click approval, not a manual file creation
- The learning loop demonstrably improves quality over time (metrics prove it)
- Lessons from one project can benefit other projects without manual duplication

---

## Journey 6: Onboard Existing Project

**Trigger:** User opens a project that has code but incomplete or no governance framework. This is a key scenario for projects that already have `.claude/` artifacts created through Claude Code CLI sessions.
**Persona emphasis:** Jordan (most common entry point), Sam (existing CLI user discovering OrqaStudio), Alex (systematic approach).
**MVP scope:** Partial — codebase scan in Phase 1. Conversational backfill in Phase 4.

### Steps

1. **Open project** — User opens a directory containing an existing codebase. OrqaStudio runs the three-tier scan (same as Journey 1, Step 3).

2. **Scan results** — OrqaStudio displays detected information:
   - Languages and frameworks (from manifests and hyperpolyglot)
   - Project structure (directory layout, entry points)
   - Existing configuration (CI/CD files, linters, test frameworks)
   - Existing `.claude/` artifacts (if any — these may have been created through Claude Code CLI usage)

3. **Governance gap analysis** — OrqaStudio identifies what governance artifacts exist and what's missing. Display: "Found: 3 rules, 1 agent. Missing: architecture decisions, skills, documentation." For projects with existing `.claude/` artifacts from CLI usage, this is the moment where previously invisible governance becomes visible for the first time in OrqaStudio's UI.

4. **Conversational backfill (Phase 4)** — OrqaStudio initiates a guided conversation:
   - "I see this is a TypeScript project using Next.js and Prisma. What are your coding standards?"
   - "Who typically reviews code? What do they check for?"
   - "What architectural decisions should agents respect?"
   - Based on answers, OrqaStudio generates governance artifacts (agents, rules, architecture decisions).

5. **Review and approve** — The user reviews each generated artifact. Approved artifacts are written to `.orqa/`. Rejected artifacts are discarded or revised.

6. **Ongoing refinement** — As the user works with the project, the learning loop (Journey 5) captures additional patterns and promotes them to governance. The framework grows organically from use.

### Success Criteria

- Existing projects are usable immediately (scan results populate within seconds)
- Governance backfill is conversational, not a configuration wizard
- Generated artifacts are high-quality starting points, not boilerplate
- The user can start working before backfill is complete

---

## Journey 7: New Project

**Trigger:** User wants to start a brand new project from scratch using OrqaStudio.
**Persona emphasis:** Jordan (starting a new product), Alex (setting up governance for a new initiative).
**MVP scope:** Yes — core path for Phase 1.

### Steps

1. **Create new project** — User clicks "New Project" (distinct from "Open Project"). OrqaStudio prompts for a project name and directory location. The user either selects an empty directory or provides a name for OrqaStudio to create one.

2. **Initialize project structure** — OrqaStudio creates:
   - The project directory (if it doesn't exist)
   - An `.orqa/` directory with `project.json` and a minimal project summary document
   - Empty subdirectories: `.orqa/agents/`, `.orqa/rules/`, `.orqa/skills/`, `.orqa/process/hooks/`, `.orqa/documentation/`
   - A `.gitignore` with `orqa.db` entry
   - Optionally: `git init` if the directory isn't already a git repo
   - Optionally: CLI tool symlinks (e.g., `.claude/` pointing to `.orqa/`) if the user requests CLI compatibility

3. **Project registered** — OrqaStudio registers the project in SQLite, sets it as active, and displays it in the Project Dashboard. The scan runs but finds no code — the project info shows "New project, no code detected."

4. **Project discovery conversation** — Instead of dropping the user into a blank conversation, OrqaStudio starts a structured discovery session. The conversation view opens with a system prompt that guides the AI through a series of topics to understand what the user is building. This is a real conversation — not a wizard or a form — and the user can answer naturally, skip topics, or go deep on what matters to them.

   The discovery conversation covers these topics (in whatever order feels natural):

   - **Product definition:** "What are you building? Who is it for? What problem does it solve?" — Claude asks open-ended questions to understand the product vision, target users, and core value proposition.
   - **Tech stack:** "What languages and frameworks are you planning to use?" — If the scan detected anything (e.g., user created some initial files), the AI confirms or refines. For a truly empty project, the AI asks and may suggest options based on the product description.
   - **Team:** "Are you working solo or with a team? What roles are involved?" — Understanding team size and roles informs agent design (solo developer needs different agents than a team lead managing multiple contributors).
   - **Conventions:** "Do you have existing coding standards? What's your testing approach? Any architecture patterns you prefer?" — Captures preferences that become rules and architecture decisions.
   - **Prior art:** "Is this based on any existing projects or templates? Any reference implementations?" — Helps Claude understand the starting point and avoid reinventing patterns the user already knows.

   The discovery is:
   - **Conversational, not a wizard** — the user types naturally; Claude synthesizes and asks follow-ups
   - **Optional** — the user can say "that's enough" or "skip this" at any time
   - **Progressive** — the session is saved like any other, so the user can come back and add more context later
   - **Resumable** — if the user closes OrqaStudio mid-discovery, the session persists and can be continued

5. **Tailored governance generation** — When the user signals they're done (or the AI has enough context), the AI proposes governance artifacts based on the conversation — not generic templates, but artifacts informed by everything discussed:

   - A **project summary document** (`.orqa/documentation/product/overview.md`) derived from the conversation (product description, tech stack, team structure, key decisions). If CLI compatibility is requested, a corresponding tool-specific summary file (e.g., `CLAUDE.md`) is also written.
   - **Agents** tailored to the tech stack and team structure (e.g., a Rust backend agent and a Svelte frontend agent, rather than a generic "default" agent)
   - **Rules** based on stated conventions (e.g., "always use Result instead of unwrap" if the user mentioned Rust error handling preferences)
   - **Initial architecture decisions** captured from the conversation (e.g., "monorepo with separate crates" or "API-first design")

   The user reviews each proposed artifact: approve, modify, or reject. Approved artifacts are written to `.orqa/`. This follows the same review pattern as Journey 4 — the human approval gate applies here too.

6. **Skip path** — At any point, the user can skip the discovery conversation entirely and start with generic scaffolding (a minimal project summary with just the project name and date, empty agent/rule directories). They can always return to discovery later by starting a new conversation and describing their project — the system is designed so governance can grow organically.

7. **Ongoing development** — From here, the project follows the same journeys as any other: implementation cycle (Journey 3), review (Journey 4), and eventually the learning loop (Journey 5).

### Success Criteria

- Creating a new project takes < 30 seconds to reach the discovery conversation
- The discovery conversation feels natural — like talking to a colleague about a new project, not filling out a form
- Generated governance artifacts are meaningfully tailored to the project (not boilerplate)
- If CLI compatibility was requested, the corresponding symlinks are in place and the user can switch to an external CLI tool and the governance artifacts work immediately
- Skipping discovery is easy and guilt-free — the user can always come back
- The discovery session is a regular session (searchable, resumable, appears in session history)

### Difference from Journey 1 (First-Time Setup) and Journey 6 (Onboard Existing)

| Aspect | Journey 1 | Journey 6 | Journey 7 |
|--------|-----------|-----------|-----------|
| Starting point | Existing code, any state | Existing code, no governance | No code, no governance |
| Scan results | Languages, frameworks detected | Languages detected, governance gap | Empty — new project |
| Governance | May or may not exist | Doesn't exist, backfill offered | Doesn't exist, discovery offered |
| Conversation style | Open-ended | Governance backfill conversation (infers from existing code) | Project discovery conversation (builds understanding from scratch) |
| User intent | Start working with AI on existing project | Bring governance to existing code | Build something new from scratch |
| Generated artifacts | N/A | Tailored to detected codebase | Tailored to discovery conversation |

---

## Journey 8: Existing CLI User Discovers OrqaStudio

**Trigger:** A developer who already uses Claude Code CLI with `.claude/` governance opens OrqaStudio for the first time.
**Persona emphasis:** Sam (primary), Jordan (secondary).
**MVP scope:** Yes — covered by Journey 1 + Journey 6 infrastructure.

### Steps

1. **Launch OrqaStudio** — Sam has been using Claude Code CLI for months. Their project has `.claude/rules/`, `.claude/agents/`, `.claude/hooks/`, and a well-developed `CLAUDE.md`. They install OrqaStudio and open it.

2. **Detect CLI and artifacts** — OrqaStudio detects Claude Code CLI (status bar: "CLI: Connected"). Sam opens their project directory. The codebase scan runs, and OrqaStudio discovers the existing `.claude/` directory.

3. **Governance surfaces in UI** — For the first time, Sam sees their governance framework as a browsable, visual interface. The artifact browser shows: "12 rules, 5 agents, 3 skills, 2 hooks." Sam clicks through and sees their rules rendered as formatted markdown with YAML metadata displayed — instead of reading raw files in a text editor.

4. **Continue working** — Sam starts a conversation in OrqaStudio. The AI session respects the same `.claude/` governance that their CLI sessions use. Tool calls appear as visual cards instead of terminal output.

5. **Edit through OrqaStudio** — Sam edits a rule through OrqaStudio's artifact editor. They save, then switch to a terminal and run a Claude Code CLI session — the edited rule is immediately effective in the CLI because it was written to `.orqa/rules/`, which `.claude/rules/` symlinks to.

6. **Ongoing dual usage** — Sam uses OrqaStudio for governance visibility (browsing artifacts, reviewing learning loop metrics, checking scanner dashboards) and the CLI for rapid coding sessions. Both tools operate on the same files — `.orqa/` is the source of truth, with `.claude/` symlinks providing CLI compatibility.

### Success Criteria

- Existing `.claude/` artifacts are fully recognized and displayed without migration or conversion
- No "import" step — OrqaStudio reads the files directly from disk
- Sam can switch between OrqaStudio and CLI within the same work session without any sync issues
- Changes made in either tool are immediately visible in the other

---

## Journey Interdependencies

```
Journey 1 (First-Time Setup)
    ├──→ Journey 7 (New Project) — if starting from scratch
    ├──→ Journey 2 (Define Governance) — if no governance framework found
    ├──→ Journey 6 (Onboard Existing) — if code exists without governance
    ├──→ Journey 8 (CLI User Discovers OrqaStudio) — if .claude/ exists from CLI usage
    └──→ Journey 3 (Implementation Cycle) — immediate conversation
              └──→ Journey 4 (Review and Approve)
                        └──→ Journey 5 (Learning Loop) — when mistakes occur
                                  └──→ Journey 2 (Define Governance) — when lessons promote to rules
```

---

## MVP Journey Coverage

| Journey | Phase 1 Coverage | Full Coverage |
|---------|-----------------|---------------|
| 1. First-Time Setup | Full (scan + conversation) | Phase 1 |
| 2. Define Governance | Browse + edit existing artifacts | Phase 4 (conversational creation) |
| 3. Implementation Cycle | Conversation + streaming + tool display | Phase 2 (tool approval) |
| 4. Review and Approve | Tool call display (read-only) | Phase 2 (approval flow) |
| 5. Learning Loop | Not in MVP | Phase 5 |
| 6. Onboard Existing | Codebase scan only | Phase 4 (conversational backfill) |
| 7. New Project | Directory creation + .orqa/ scaffold + project discovery conversation | Phase 1 (full discovery flow) |
| 8. CLI User Discovers OrqaStudio | Full (artifact detection + display) | Phase 1 |

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Journey 2 (Define Governance), Journey 6 (Onboard Existing), and Journey 8 (CLI User Discovers OrqaStudio) are entirely about making invisible governance structure visible and manageable. |
| Learning Through Reflection | Journey 5 (Learning Loop) directly operationalises Pillar 2 — capturing lessons, tracking recurrence, and promoting patterns into governance artifacts. Every journey feeds session history that the learning loop draws from. |

---

## Related Documents

- User Personas — Who follows these journeys
- MVP Feature Specification — What Phase 1 delivers
- Information Architecture — UI structure that supports these journeys
