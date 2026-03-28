---
id: "RES-6d9cafb9"
type: discovery-research
title: "Composability Gate"
description: "Audit codebase for composability alignment, produce the composability skill, scanner, and enforcement rule."
status: completed
created: "2026-03-06"
updated: "2026-03-06"
relationships:
  - target: "EPIC-c1833545"
    type: "guides"
    rationale: "Research findings informed the design of Composability Refactoring"
---

# Composability Gate: Audit, Alignment & Platform Architecture

## Problem Statement

OrqaStudio promotes composability as a core principle ([AD-af88bb69](AD-af88bb69), `composability` skill) but the app itself has composability debt: monolithic commands, god components, tightly coupled features. The app cannot credibly enforce composability on projects it manages if it doesn't adhere to those principles itself.

Beyond the app's own code, composability must be a **platform-level architectural principle** that shapes:

- How the app is built (every layer, from AI provider integration to UI components)
- How the app interacts with projects (the learning loop, enforcement, scanning — all composable)
- How new projects are scaffolded (composability by default)
- How existing projects are onboarded (composability assessment, user choice)

This is the gate before dogfooding. Once this work is complete, every feature built through dogfooding inherits composability by design.

## Guiding Principles

1. **The app practices what it preaches** — no monolithic code anywhere, functional coding principles at every layer
2. **Composability is language/framework-agnostic** — the philosophy applies regardless of Rust, TypeScript, Python, Go
3. **Base + Project layering** — composable core ships with the app, project-specific requirements layer on top
4. **User choice for existing codebases** — non-composable projects are fine, but HOW the app interacts with them is composable
5. **Composability by default for greenfield** — new projects (especially no-code PM building a product) get composable architecture out of the box

---

## Phase A: Deep Composability Audit (OrqaStudio)

Comprehensive audit of the entire OrqaStudio codebase against composability principles. Every layer. No exceptions.

### A1: AI Provider Integration Layer

**Audit scope:** `sidecars/claude-agentsdk-sidecar/src/`, `backend/src-tauri/src/sidecar/`, streaming pipeline

| Check | What to verify |
| ------- | --------------- |
| Provider abstraction | Is the sidecar truly provider-agnostic? Can Claude Agent SDK be swapped for API, Codex, OpenAI without touching Rust or Svelte? |
| Protocol contract | Is the NDJSON protocol a clean interface or does it leak Claude-specific concepts? |
| Authentication | Is auth handling provider-specific or pluggable? |
| Tool server | Is the MCP tool server provider-independent? |
| Configuration | Can provider selection be driven by `.orqa/project.json`? |

**Known issues:** Currently hardcoded to Agent SDK. Auth assumes Claude CLI. Provider selection not yet in project config.

### A2: Rust Backend Layer

**Audit scope:** `backend/src-tauri/src/` — every module

| Check | What to verify |
| ------- | --------------- |
| Command thickness | Every `#[tauri::command]` handler should be ≤30 lines. Business logic lives in `domain/`. |
| Domain service purity | Every domain function is pure (takes inputs, returns outputs, no hidden state). |
| Repository consistency | Every repo follows the standard interface (create/get/list/update/delete). |
| Error composition | All errors flow through `OrqaError` with correct variant selection. |
| Module coupling | No circular dependencies between domain modules. |
| Function size | No function exceeds 50 lines (domain: 20-30, commands: 30-50, utilities: 10-20). |
| Feature isolation | Each feature is a self-contained unit (domain + command + type). Can be removed without cascading changes. |

**Known debt:**

- `stream_commands.rs` (~1000+ lines) — thick command with business logic that should be domain services
- `artifact_commands.rs` — some inline helpers that could be domain functions
- `governance_commands.rs` — `governance_analyze` does sidecar orchestration inline

### A3: Frontend Store Layer

**Audit scope:** `ui/src/lib/stores/` — all 10 stores

| Check | What to verify |
| ------- | --------------- |
| Store independence | No store imports another store (except documented exception). |
| Store size | No store exceeds 300 lines. |
| State encapsulation | All state mutations happen inside the store, not from components. |
| Loading lifecycle | Every async operation follows loading/error/success pattern. |
| Circular deps | No bidirectional store imports. |

**Known debt:**

- `conversationStore` imports `sessionStore` directly (the one coupling violation)

### A4: Frontend Component Layer

**Audit scope:** `ui/src/lib/components/` — every component

| Check | What to verify |
| ------- | --------------- |
| Component purity | Display components receive props only. No `invoke()` in `$lib/components/`. |
| Component size | No component exceeds 150 lines of script + template. |
| Shared component usage | EmptyState, LoadingSpinner, ErrorDisplay, etc. used everywhere — no inline equivalents. |
| Reusable component architecture | Common patterns extracted to shared components with variant props. |
| Snippet/slot composition | Complex components use snippets for customizable sections. |
| State handling | Every component handles all states: loading, error, empty, loaded. |

**Known debt:**

- `ConversationView.svelte` — orchestrates 8+ children, 4 stores, has side effects
- Potential inline empty/loading states in newer components

### A5: CSS/Design System Layer

**Audit scope:** `ui/src/app.css`, `tailwind.config.ts`, `ui/src/lib/components/ui/`

| Check | What to verify |
| ------- | --------------- |
| Design tokens | Colors, spacing, typography defined as tokens, not hardcoded values. |
| Component variants | shadcn-svelte components use variant props, not ad-hoc Tailwind overrides. |
| Consistency | Same visual pattern uses the same component everywhere. |
| Theme composability | Per-project theming works without modifying component code. |

### A6: Configuration & Integration Layer

**Audit scope:** `.orqa/project.json`, `backend/src-tauri/capabilities/`, Makefile

| Check | What to verify |
| ------- | --------------- |
| Config-driven behavior | Project-specific behavior driven by config, not hardcoded conditionals. |
| Capability scoping | Tauri capabilities are minimal and composable. |
| Build composability | Make targets are composable (can run individually or combined). |

### Audit Output

The audit produces:

1. **Composability scorecard** — per-layer scores with specific violations
2. **Refactoring plan** — prioritized by severity and blast radius
3. **Architectural debt register** — tracked items with owners and target dates

---

## Phase B: Refactoring to Composability

Execute the refactoring plan from Phase A. Each refactoring is a worktree task delegated to the appropriate agent.

### B1: Extract Domain Services from Thick Commands

**Target:** `stream_commands.rs` → extract to domain modules

| Extract | Target module | What moves |
| --------- | -------------- | ------------ |
| System prompt assembly | `domain::system_prompt` | `build_system_prompt()`, `read_rules()`, `list_skill_catalog()` |
| Context management | `domain::context` | `load_context_messages()`, context injection logic |
| Stream translation | `domain::stream_translator` | `translate_response()`, event mapping |
| Process compliance | `domain::process_state` (extend) | Violation emission, compliance checking |
| Auto-titling | `domain::session_title` | Title generation, sanitization |

After extraction, `stream_commands.rs` should be ≤200 lines — just the Tauri command wrappers and stream loop.

### B2: Decompose Monolithic Components

**Target:** `ConversationView.svelte` → split into composable units

| New component | Responsibility |
| -------------- | ---------------- |
| `ConversationPanel.svelte` | Message list + streaming display (pure display) |
| `ConversationOrchestrator.svelte` | Store coordination, side effects, keyboard handling |
| `SessionContextLoader.svelte` | Loads doc/research/plan context for the active session |

### B3: Resolve Store Coupling

**Target:** `conversationStore` → `sessionStore` dependency

Options (evaluate during implementation):

- Event bus pattern (decouple via custom events)
- Component-mediated pattern (move title update to ConversationView's $effect)
- Callback injection (store accepts an `onTitleUpdate` callback at init)

### B4: UI Component Architecture Review

- Audit all components for inline empty/loading/error states → replace with shared components
- Identify repeated visual patterns → extract to shared components with variants
- Ensure shadcn-svelte components are used with variant props, not ad-hoc overrides
- Document the component architecture in a design system guide

### B5: Provider Abstraction Hardening

- Audit sidecar protocol for Claude-specific leaks
- Define a provider interface contract that any AI provider must implement
- Ensure auth is pluggable (Claude CLI, API key, OAuth, custom)
- Add provider selection to `.orqa/project.json` schema

---

## Phase C: Composable Learning Loop Architecture

The learning loop (skills, agents, rules, scanning, enforcement, lessons) must itself be composable so it can be assembled per-project.

### C1: Base + Project Layering

**Base layer (ships with app):**

- Generic `composability` skill (language-agnostic philosophy)
- Generic `chunkhound` skill (code search patterns)
- Base agents: orchestrator, code-reviewer, test-engineer, documentation-writer (roles that exist in any project)
- Base rules: error handling, no stubs, coding standards (principles that apply universally)
- Base scanner: composability anti-pattern detection, function size, coupling analysis

**Project layer (generated during init or customized by user):**

- Project-specific skills with real code examples from that codebase
- Project-specific agents with domain knowledge (e.g., `backend-engineer` configured for Rails vs. Django vs. Rust)
- Project-specific rules derived from the project's existing coding standards
- Project-specific scanner rules for domain-specific patterns

**Assembly:** `.orqa/project.json` declares:

```json
{
  "composability": {
    "base_skills": ["composability", "chunkhound"],
    "project_skills": ["myproject-api-patterns", "myproject-db-patterns"],
    "base_agents": ["orchestrator", "code-reviewer", "test-engineer"],
    "project_agents": ["rails-engineer", "react-engineer"],
    "scanner_profiles": ["composability", "security", "myproject-domain-rules"]
  }
}
```

### C2: Skill Composability

Skills must be:

- **Stackable** — base skill provides principles, project skill provides examples
- **Overridable** — project skill can override base skill guidance for specific patterns
- **Discoverable** — the app can suggest skills based on detected tech stack
- **Portable** — skills work in both CLI (Claude Code) and App (OrqaStudio) contexts

### C3: Agent Composability

Agents must be:

- **Role-based** — defined by what they do, not what tech they use
- **Configurable** — the same `backend-engineer` role adapts to Rust, Python, Go, etc.
- **Extensible** — project-specific agents extend base agents without duplicating
- **Removable** — unused agents can be removed without breaking the system

### C4: Rule Composability

Rules must be:

- **Layered** — base rules always apply, project rules add on top
- **Scoped** — rules can target specific file paths, languages, or phases
- **Enforceable** — every rule has a corresponding scanner entry
- **Promotable** — lessons become rules through the promotion pipeline

### C5: Scanner Composability

Scanners must be:

- **Pluggable** — projects can define custom scan rules beyond built-in ones
- **Composable** — multiple scan profiles can be combined
- **Incremental** — scans run on changed files, not the entire codebase every time
- **Actionable** — every finding has a severity, a fix suggestion, and a link to the governing rule

---

## Phase D: Initialization Composability Assessment

When OrqaStudio initializes in an existing project, assess its composability and present options.

### D1: Composability Analysis

During the governance scan (which already runs on project open), add composability-specific analysis:

- **Function size distribution** — histogram of function lengths, identify outliers
- **Coupling analysis** — module dependency graph, circular dependency detection
- **Purity assessment** — ratio of pure functions to stateful ones
- **Feature isolation** — can features be identified as self-contained units?
- **Test coverage patterns** — are tests colocated? Do they test behavior or implementation?

### D2: User-Facing Assessment Report

Present the user with:

```text
Composability Assessment: [Project Name]

Score: 62/100

Strengths:
✓ Small functions (avg 25 lines)
✓ Good test coverage (78%)
✓ Clear module boundaries

Gaps:
✗ 3 functions exceed 100 lines (controllers/auth.py, services/billing.py, utils/parser.py)
✗ Circular dependency: models ↔ services
✗ No clear domain/adapter separation

Options:
(a) Generate a refactoring plan — Claude will create a phased plan to improve composability
(b) Accept current state — governance will work with current patterns, suggest improvements incrementally
(c) Skip composability governance — only enforce project-specific coding standards
```

### D3: Greenfield Default

For new projects (no existing code):

- Composability is ON by default
- Project-specific composability skill is generated from the scaffolded template
- Base agents are configured with composability-aware instructions
- Scanner includes composability rules from day one

### D4: No-Code PM Scenario

When a non-technical user creates a product from scratch:

- The orchestrator agent applies composability principles to all generated code
- The composability skill is loaded as a universal skill
- Code review agent enforces composability before accepting deliverables
- The user never needs to know the word "composability" — it just happens

---

## Implementation Order

| Step | Phase | Scope | Delegated To |
| ------ | ------- | ------- | ------------- |
| 1 | A | Deep audit — all 6 layers | systems-architect + code-reviewer |
| 2 | A | Scorecard + refactoring plan | systems-architect → documentation-writer |
| 3 | B1 | Extract domain services from stream_commands.rs | backend-engineer |
| 4 | B2 | Decompose ConversationView.svelte | frontend-engineer |
| 5 | B3 | Resolve store coupling | frontend-engineer |
| 6 | B4 | UI component architecture review | designer + ux-reviewer |
| 7 | B5 | Provider abstraction hardening | backend-engineer + systems-architect |
| 8 | C1 | Base + Project layering design | systems-architect → documentation-writer |
| 9 | C2-C5 | Composable learning loop implementation | backend-engineer + frontend-engineer |
| 10 | D1-D2 | Initialization composability assessment | backend-engineer + frontend-engineer |
| 11 | D3-D4 | Greenfield + no-code defaults | backend-engineer + documentation-writer |
| 12 | — | Final verification: `make check` + composability scanner on own codebase | code-reviewer + qa-tester |

---

## Verification

1. **Phase A complete when:** Scorecard covers all 6 layers, every violation documented, refactoring plan approved
2. **Phase B complete when:** `stream_commands.rs` ≤200 lines, `ConversationView.svelte` decomposed, no store-to-store imports, all shared components used consistently, `make check` passes
3. **Phase C complete when:** Base skills/agents/rules/scanner defined and separated from project-specific ones, `.orqa/project.json` schema supports composability config, learning loop works with both layers
4. **Phase D complete when:** Initialization scan includes composability assessment, user sees options dialog, greenfield projects get composability by default
5. **Overall gate passed when:** OrqaStudio's own composability score is ≥85/100, and the composability scanner can run on itself as part of `make check`

---

## Pillar Alignment

| Pillar | Alignment |
| -------- | ----------- |
| Learning Through Reflection | The composability scanner feeds findings into the lesson system. Recurring violations are promoted to rules. The system learns which composability patterns work for which project types. |
| Clarity Through Structure | Composability becomes an enforceable governance dimension — scannable, scoreable, visible in dashboards. Rules, agents, and skills are themselves governed by composability principles. |
