---

id: RES-5435cae9
type: discovery-research
title: Agent Governance Overhaul
description: "Make the agentic framework operational: fix agent paths, add skills, remove broken tools, add dual-context awareness, and enforce usage."
status: surpassed
created: 2026-03-06
updated: 2026-03-12
---

## Problem Statement

All 15 agent definitions in `.orqa/agents/` are effectively dormant. Dogfooding readiness requires that the agentic framework actually works — agents must know what tools they have, where to find documentation, what patterns to follow, and how to operate in both CLI and app contexts.

**Current gaps:**

1. **Wrong Required Reading paths** — Agents reference directories that don't exist (`docs/standards/`, `docs/decisions/`, `docs/vision/`, `docs/process/lessons.md`)
2. **Missing skills** — Almost all agents only declare `chunkhound`. Domain-specific skills are missing.
3. **Nonexistent tools** — 6 agents reference `mcp__MCP_DOCKER__*` tools that don't exist.
4. **No context awareness** — Agents don't know they operate in two contexts (CLI vs. app) with different tool names and capabilities.
5. **Generic bodies** — Agent instructions are boilerplate templates, not OrqaStudio-specific.
6. **No enforcement** — No mechanism ensures agents are actually used or that they load their skills.
7. **Research organization** — MVP-phase research files aren't grouped as such.

## Goal

**Dogfooding-ready agent framework.** After this plan is complete, every agent:

- Knows the OrqaStudio tech stack (Tauri v2, Svelte 5, Rust, SQLite, Agent SDK sidecar)
- Knows which tools are available and what they're called in each context
- Has correct documentation paths that resolve to real files
- Has domain-appropriate skills loaded before implementation begins
- Produces output that matches the project's actual patterns, not generic boilerplate

---

## Phase 1: Dogfooding-Ready Agent Definitions

**Goal**: Every agent is fully operational in both CLI and app contexts with OrqaStudio-specific instructions.

### 1a: Dual-Context Tool Configuration

OrqaStudio permanently supports two operating contexts. Both are first-class — neither is temporary or deprecated.

**CLI context** (Claude Code CLI with `.mcp.json`):

- File tools: `Read`, `Edit`, `Write`, `Bash`, `Glob`, `Grep` (built-in Claude Code tools)
- Search tools: `mcp__chunkhound__search_regex`, `mcp__chunkhound__search_semantic`, `mcp__chunkhound__code_research` (via ChunkHound MCP server in `.mcp.json`)
- ChunkHound is a permanent dev-tool environment enhancement for CLI usage
- No sidecar, no streaming, no Tauri commands

**App context** (OrqaStudio sidecar):

- File tools: `read`, `write`, `edit`, `bash`, `glob`, `grep` (native Rust tools exposed via built-in MCP server)
- Search tools: `search_regex`, `search_semantic`, `code_research` (native, embedded ONNX + DuckDB)
- Streaming, tool approval UI, governance scanning, sub-agent spawning

**What each agent needs to know:**

- Both contexts are permanent and first-class
- The context is determined by how the agent is invoked — CLI subagent or app sidecar
- In CLI: search tools have `mcp__chunkhound__` prefix (MCP server)
- In app: search tools have no prefix (native embedded)
- The `chunkhound` skill teaches query patterns that work in both contexts
- **When dogfooding** (using OrqaStudio to develop OrqaStudio): agents must be explicitly told they are dogfooding. This changes behavior — e.g., `make dev` uses `--no-watch` so editing `.rs` files doesn't kill the running app mid-session

**Agent `tools:` frontmatter**: List both sets so agents work in either context:

```yaml
tools:
  - Read
  - Edit
  - Write
  - Glob
  - Grep
  - Bash
  # CLI context (ChunkHound MCP server — permanent dev tool)
  - mcp__chunkhound__search_regex
  - mcp__chunkhound__search_semantic
  - mcp__chunkhound__code_research
  # App context (native embedded search)
  - search_regex
  - search_semantic
  - code_research
```

### 1f: Dogfood Detection & Enhanced Caution

When the project being managed IS OrqaStudio itself, agents need enhanced caution that doesn't apply to normal projects. This must be detectable and enforceable now — not deferred to the future project type system.

**Detection mechanism:**

- Add `"dogfood": true` flag to `.orqa/project.json`
- In CLI context: agents check `.orqa/project.json` at task start
- In app context: the project settings store exposes `isDogfood` and the system prompt includes dogfood context when true
- `CLAUDE.md` already contains some dogfood rules (no-watch, restart protocol) — these should reference the flag

**Enhanced caution rules (dogfood only):**

- **`make dev` uses `--no-watch`** — editing `.rs` files must NOT auto-restart and kill the session
- **After Rust backend changes**: write session state to `.state/session-state.md`, then tell the user the backend needs a manual restart
- **Sidecar self-edit warnings** — the sidecar they communicate through IS the sidecar they might be editing. Changes to `sidecars/claude-agentsdk-sidecar/src/` require extra care: verify the change won't break the active connection, and warn the user before modifying protocol types
- **No preview tooling** — you can't preview yourself inside yourself (relevant when preview tooling is added later)
- **They are editing the app they are running inside** — Vite HMR handles frontend changes live, but Rust changes require restart

**Deliverables:**

1. Add `"dogfood": true` to `.orqa/project.json`
2. Add `ProjectSettings.dogfood: bool` (Rust + TypeScript)
3. Create `.orqa/rules/dogfood-mode.md` — consolidates all dogfood-specific rules, references the detection flag
4. Every agent's "Operating Context" section includes: "If the project is a dogfood project (`dogfood: true` in `.orqa/project.json`), apply enhanced caution rules — see `.orqa/rules/dogfood-mode.md`"
5. System prompt injection in app context when dogfood is detected

### 1b: Required Reading Path Corrections

| Wrong Path | Correct Path |
| ------------ | ------------- |
| `docs/standards/coding-standards.md` | `docs/development/coding-standards.md` |
| `docs/standards/` | `docs/development/` |
| `docs/decisions/` | `docs/architecture/` |
| `docs/vision/` → anything | `docs/product/vision.md` |
| `docs/process/lessons.md` | `.orqa/lessons/` |
| Backend/frontend dependency manifest | `backend/src-tauri/Cargo.toml` / `package.json` |
| Backend entry point | `backend/src-tauri/src/lib.rs` |

### 1c: Tool Cleanup

- Remove all `mcp__MCP_DOCKER__*` references from: `frontend-engineer`, `designer`, `debugger`, `test-engineer`, `qa-tester`, `ux-reviewer`

### 1d: Domain-Appropriate Skills Per Agent

| Agent | Current Skills | Add Skills |
| ------- | --------------- | ------------ |
| `backend-engineer` | `chunkhound` | `rust-async-patterns`, `tauri-v2` |
| `frontend-engineer` | `chunkhound` | `svelte5-best-practices`, `tailwind-design-system`, `typescript-advanced-types` |
| `designer` | `chunkhound` | `svelte5-best-practices`, `tailwind-design-system` |
| `debugger` | `chunkhound` | `rust-async-patterns`, `tauri-v2`, `svelte5-best-practices` |
| `test-engineer` | `chunkhound` | `rust-async-patterns`, `svelte5-best-practices` |
| `code-reviewer` | `chunkhound` | `rust-async-patterns`, `svelte5-best-practices` |
| `data-engineer` | `chunkhound` | `rust-async-patterns` |
| `devops-engineer` | `chunkhound` | `tauri-v2` |
| `documentation-writer` | `chunkhound` | `planning` |
| `security-engineer` | `chunkhound` | `tauri-v2` |
| `refactor-agent` | `chunkhound` | `rust-async-patterns`, `svelte5-best-practices` |
| `agent-maintainer` | `chunkhound`, `planning`, `skills-maintenance` | *(already correct)* |
| `systems-architect` | `chunkhound` | `architecture`, `planning` |
| `qa-tester` | `chunkhound` | *(none needed beyond chunkhound)* |
| `ux-reviewer` | `chunkhound` | `tailwind-design-system` |

### 1e: OrqaStudio-Specific Agent Bodies

Every agent body must be rewritten to reference the actual project, not generic placeholders. Each agent body must include:

1. **Tech stack context** — "This project uses Tauri v2 + Svelte 5 (runes) + Rust + SQLite" and specifics relevant to that agent's domain
2. **Key patterns** — The actual patterns used in this codebase (e.g., `thiserror` for errors, `Channel\<T\>` for streaming, rune stores in `.svelte.ts`, `invoke()` for IPC)
3. **Development commands** — Reference `make` targets, not raw `cargo`/`npm` commands
4. **Context awareness section** — Explain CLI vs. app tool availability so the agent knows what context it's operating in
5. **Critical files** — List the specific files/directories the agent works with most

**Example context awareness section (for every agent body):**

```markdown
## Operating Context

You may run in two contexts:

**CLI (Claude Code):** File tools are built-in (`Read`, `Edit`, etc.). Search tools
use MCP namespace: `mcp__chunkhound__search_regex`, `mcp__chunkhound__search_semantic`,
`mcp__chunkhound__code_research`.

**App (OrqaStudio):** File tools are native Rust implementations (`read`, `edit`, etc.).
Search tools are native embedded: `search_regex`, `search_semantic`, `code_research`.
No MCP prefix needed.

The `chunkhound` skill teaches query patterns that work in both contexts.
Use `make` targets for all build/test/lint commands (see `docs/development/commands.md`).
```

### 1f: Update `chunkhound-usage.md` Rule

Update `.orqa/rules/chunkhound-usage.md` to:

- Document both tool name sets (CLI MCP vs. app native)
- Explain the native tools are embedded (ONNX + DuckDB in `backend/src-tauri/src/search/`)
- Note that `.mcp.json` ChunkHound is a permanent CLI/dev-tool enhancement; native search is for the app context
- Keep the existing query patterns and anti-patterns (they apply regardless)

### Changes Per Agent Summary

| Agent | Fix Paths | Add Skills | Remove Tools | Add Native Search | Make Project-Specific |
| ------- | ----------- | ------------ | -------------- | ------------------- | ---------------------- |
| `backend-engineer` | Y | `rust-async-patterns`, `tauri-v2` | — | Y | Rust/Tauri v2, `thiserror`, `Channel\<T\>`, SQLite, sidecar protocol |
| `frontend-engineer` | Y | `svelte5-best-practices`, `tailwind-design-system`, `typescript-advanced-types` | `mcp__MCP_DOCKER__*` (5) | Y | Svelte 5 runes, shadcn-svelte, `invoke()`, rune stores |
| `designer` | Y | `svelte5-best-practices`, `tailwind-design-system` | `mcp__MCP_DOCKER__*` (3) | Y | shadcn-svelte variants, Lucide icons, Orqa color system |
| `debugger` | Y | `rust-async-patterns`, `tauri-v2`, `svelte5-best-practices` | `mcp__MCP_DOCKER__*` (3) | Y | IPC boundary, sidecar NDJSON, streaming pipeline |
| `test-engineer` | Y | `rust-async-patterns`, `svelte5-best-practices` | `mcp__MCP_DOCKER__*` (3) | Y | `make test` variants, Vitest, cargo test |
| `code-reviewer` | Y | `rust-async-patterns`, `svelte5-best-practices` | — | Y | `make check`, zero-error, function size limits |
| `data-engineer` | Y | `rust-async-patterns` | — | Y | rusqlite, migrations, repository pattern |
| `devops-engineer` | Y | `tauri-v2` | — | Y | Makefile, Tauri build, sidecar bundling |
| `documentation-writer` | Y | `planning` | — | Y | Pillar alignment, `.orqa/research/`, frontmatter schemas |
| `security-engineer` | Y | `tauri-v2` | — | Y | Tauri capabilities, CSP, keyring |
| `refactor-agent` | Y | `rust-async-patterns`, `svelte5-best-practices` | — | Y | Module boundaries, IPC patterns |
| `agent-maintainer` | minor | *(already correct)* | — | Y | Orqa-specific governance paths |
| `systems-architect` | Y | `architecture`, `planning` | — | Y | Two-pillar, IPC boundary, streaming |
| `qa-tester` | Y | — | `mcp__MCP_DOCKER__*` (3) | Y | `make test`, E2E Playwright, Tauri app |
| `ux-reviewer` | Y | `tailwind-design-system` | `mcp__MCP_DOCKER__*` (3) | Y | shadcn-svelte, Lucide, responsive |

### Acceptance Criteria

- [ ] All 15 agents have correct, resolvable Required Reading paths (every path verified with `ls`)
- [ ] Every agent declares skills matching its domain
- [ ] No agent references `mcp__MCP_DOCKER__*` tools
- [ ] All agents list BOTH `mcp__chunkhound__*` (CLI) AND native `search_regex`, `search_semantic`, `code_research` (app) — both permanent
- [ ] Every agent body includes an "Operating Context" section explaining CLI vs. app tool names
- [ ] Every agent body references OrqaStudio's specific tech stack, patterns, and `make` commands
- [ ] `chunkhound-usage.md` rule documents both tool name sets and explains native embedding
- [ ] `.mcp.json` ChunkHound entry is permanent (CLI/dev-tool enhancement)
- [ ] An agent invoked in the CLI can find its Required Reading, load its skills, and use search tools
- [ ] An agent invoked in the app can find its Required Reading, load its skills, and use search tools
- [ ] `.orqa/project.json` has `"dogfood": true` for the OrqaStudio project
- [ ] `dogfood-mode.md` rule exists with consolidated dogfood-specific caution rules
- [ ] Every agent's "Operating Context" section references dogfood mode detection
- [ ] Dogfood-specific caution (no-watch, session state before restart, sidecar self-edit warnings) is only active when `dogfood: true`

---

## Phase 2: Create Project-Level Skills

**Goal**: OrqaStudio-specific patterns captured as loadable skills so agents have concrete guidance, not generic advice.

### Skills to Create

| Skill | Content | Used By |
| ------- | --------- | --------- |
| `orqa-ipc-patterns` | Tauri `invoke()` patterns, `Channel\<T\>` streaming, command registration, IPC type contracts, the full request chain (component → store → invoke → Rust command) | `backend-engineer`, `frontend-engineer`, `debugger`, `systems-architect` |
| `orqa-store-patterns` | Svelte 5 rune stores (`.svelte.ts`), `$state`/`$derived`/`$effect`, store-to-component data flow, the store pattern used throughout the app | `frontend-engineer`, `designer`, `debugger` |
| `orqa-streaming` | Agent SDK → sidecar (Bun) → NDJSON → Rust Channel\<T\> → Svelte pipeline, `ProviderEvent` types, `StreamEvent` types, error handling, backpressure | `backend-engineer`, `frontend-engineer`, `debugger` |
| `orqa-governance` | Artifact types, scanning pipeline, lesson promotion, rule enforcement, frontmatter schemas (`DocFrontmatter`, `ResearchFrontmatter`), `.orqa/` directory structure | `agent-maintainer`, `code-reviewer`, `documentation-writer` |
| `orqa-testing` | `make test` commands, Vitest patterns for Svelte stores/components, cargo test patterns for domain logic, mock boundaries (trait-based), test file locations | `test-engineer`, `qa-tester`, `code-reviewer` |

### Skill Format

Each skill is a `.md` file in `.orqa/skills/` following the standard format:

- YAML frontmatter with `name`, `description`, `tags`
- Patterns section with **real code examples from this codebase** (not generic)
- Anti-patterns section with actual mistakes we've made
- References to the specific project files where the pattern is implemented

### Acceptance Criteria

- [ ] 5 project-level skills created in `.orqa/skills/`
- [ ] Each skill has concrete OrqaStudio code examples (not generic)
- [ ] Agent definitions updated to reference these skills in their `skills:` lists
- [ ] Skills load successfully via `Skill(name)` invocation

---

## Phase 3: Enforcement Mechanisms

**Goal**: Rules and hooks ensuring agents are used and configured correctly in both CLI and app.

### 3a: Orchestration Enforcement Rule

Create `.orqa/rules/agent-delegation.md`:

- The orchestrator MUST delegate implementation to agents, never implement directly
- Every implementation task must name the agent being delegated to
- The orchestrator may only: read files for planning, write plans/docs, coordinate, report status
- Exceptions: single-line fixes, typo corrections, config file changes

### 3b: Agent Usage Audit Hook

Update `.orqa/hooks/session-start-hook.sh` to:

- List available agents and their domains
- Remind the orchestrator to delegate, not implement
- Check that agent definitions have valid Required Reading paths

### 3c: Skill Loading Enforcement

Update `.orqa/rules/skill-enforcement.md`:

- Before any agent starts implementation, it MUST load all skills declared in its frontmatter
- Skill loading must happen BEFORE Required Reading
- If a skill fails to load, the agent must report it (not silently continue)
- In the app, skill loading is tracked and enforced by the governance layer

### 3d: App-Side Enforcement (Design Only)

Capture the design for app-side enforcement in the roadmap:

- Track which agents are invoked per session
- Warn if the orchestrator is writing code directly
- Show agent utilization metrics in the dashboard
- Enforce skill loading before task execution

### Acceptance Criteria

- [ ] `agent-delegation.md` rule exists and is loaded by the orchestrator
- [ ] Session start hook reminds about delegation
- [ ] Skill loading protocol is documented and enforceable
- [ ] App enforcement design captured in roadmap

---

## Phase 4: Implementation Plans as First-Class Entities

**Goal**: Implementation plans are browsable in-app alongside docs and research, so they can be reviewed, referenced, and tracked during dogfooding sessions.

Currently plans live scattered across `docs/process/`, Claude Code plan files (`~/.claude/plans/`), or inline in conversation context where they're lost when context compacts. Making them a first-class entity means future dogfooding sessions can reference approved plans directly.

### Storage

Plans live in `.orqa/plans/` as markdown files with YAML frontmatter:

```yaml
---
title: "Agent Governance Overhaul"
status: completed
created: "2026-03-06"
updated: "2026-03-06"
phases: 4
completed_phases: 0
tags: [agents, governance, dogfooding]
---
```

### Changes

1. Create `.orqa/plans/` directory
2. Move `docs/process/agent-governance-plan.md` → `.orqa/plans/agent-governance.md` (this plan becomes the first entry)
3. Add `PlanFrontmatter` struct to `backend/src-tauri/src/domain/artifact.rs` (follows the generic `parse_frontmatter\<T\>` pattern)
4. Add `plan_tree_scan` and `plan_read` Tauri commands (same pattern as `research_tree_scan` / `research_read`)
5. Add "Plans" activity to the navbar (between Research and Agents) with a clipboard/kanban icon
6. Reuse `DocTreeNav` component with `mode="plans"` (same pattern as research)
7. Plans render in the artifact viewer like any other markdown document

### Acceptance Criteria

- [ ] `.orqa/plans/` directory exists with this plan as the first entry
- [ ] Plans appear in the app navbar with their own activity icon
- [ ] Plan frontmatter (status, phases, completed_phases) is parsed and available
- [ ] Plans are browsable and readable in-app
- [ ] Claude Code plan files can be copied into `.orqa/plans/` and they render correctly

*Note: This phase establishes plans as browsable markdown entities. Enhanced plan management (phase tracking, status transitions, task breakdown, progress dashboards, plan-to-implementation traceability) is deferred to the product/project management tooling feature phase — see roadmap Phase 5: Discovery & Research.*

---

## Phase 5: Research Organization

*Previously Phase 4.*

**Goal**: Existing research files grouped into an MVP subfolder so their origin is clear.

### Changes

1. Create `.orqa/research/mvp/` directory
2. Move all 8 existing research files into it:
   - `branding.md`, `claude-integration.md`, `design-tokens.md`, `frontend.md`
   - `onboarding.md`, `persistence.md`, `tauri-v2.md`, `wireframing.md`
3. Update `README.md` to describe the MVP research collection
4. Update `research_tree_scan` Rust command to handle subdirectories
5. Update cross-references in docs and roadmap

### Acceptance Criteria

- [ ] All MVP research files live under `.orqa/research/mvp/`
- [ ] Research nav in the app shows "MVP" as a collapsible folder
- [ ] `README.md` at `.orqa/research/` describes the research section
- [ ] No broken cross-references

---

## Implementation Order

```text
Phase 1 (Agent Definitions + Dogfood Detection) ─── start immediately, largest phase
Phase 4 (Plans as Entities)  ─── independent, can run in parallel with Phase 1
Phase 5 (Research Org)       ─── independent, can run in parallel with Phase 1
Phase 2 (Project Skills)     ─── depends on Phase 1 (agents reference skills)
Phase 3 (Enforcement)        ─── depends on Phase 1 (rules reference correct agent structure)
```

Recommended: **Phase 1 + Phase 4 + Phase 5 (parallel) → Phase 2 → Phase 3**

Phase 1 is the largest (15 agent files + dogfood detection + rule updates) but each agent is independent, so it can be parallelized. Phase 4 and 5 are small and self-contained.

---

## Risks and Open Questions

1. **Skill loading in app context** — The app manages skills differently from CLI. Phase 2 skills need to work in both. Need to verify the app's skill loading mechanism.
2. **Agent model selection** — Some agents use `model: sonnet`, some `model: inherit`. Propose: `sonnet` for implementation agents, `inherit` for review/planning agents.
3. **Enforcement teeth** — Rules are advisory in the CLI. True enforcement requires app-side checks (Phase 3d). The rules in Phase 3a-c are "honor system" for CLI.
4. **Research tree depth** — `research_tree_scan` currently scans one level. Phase 4 adds a subfolder — verify it handles nesting.
5. **Tool name resolution** — Both `mcp__chunkhound__*` and bare `search_*` names are listed in agent tools. Only one set will resolve in a given context (CLI sees MCP tools, app sees native tools). Verify no conflicts from having both declared.

---

## Verification

| Phase | How to Verify |
| ------- | -------------- |
| 1 | For each agent: `ls` every Required Reading path. Verify skills list matches domain. Verify no `mcp__MCP_DOCKER__` references. Verify both search tool name sets listed. Verify "Operating Context" section exists. Verify dogfood detection works. |
| 2 | For each skill: `Skill(name)` loads. Content includes Orqa-specific code examples from the actual codebase. |
| 3 | Session start hook output includes delegation reminder. `agent-delegation.md` rule is loaded in orchestrator context. |
| 4 | Plans nav shows in app. This plan is browsable and readable. Plan frontmatter parsed correctly. |
| 5 | Research nav shows MVP folder. Click each research doc — content loads. |

## Pillar Alignment

| Pillar | Alignment |
| -------- | ----------- |
| Learning Through Reflection | Properly configured agents with correct skills and reading lists produce higher-quality output, reducing rework and feeding better data into the lesson pipeline. |
| Clarity Through Structure | This plan IS governance — it ensures the agent framework is actually functional, enforceable, and used as designed. Dogfooding readiness means governance is real, not theoretical. |
