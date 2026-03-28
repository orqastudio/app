---
id: KNOW-21d28aa0
type: knowledge
title: Planning
domain: methodology/planning
summary: "Every implementation task follows a strict documentation-first workflow: **Document → Approve → Implement → Verify**. No code is written before documentation is approved."
description: |
  Enforces documentation-first planning for all implementation tasks.
  Plans must start with documentation, get user approval, then implement with
  mandatory verification gates. Prevents documentation drift across sessions.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
user-invocable: false
allowed-tools: "Read, Glob, Grep"
relationships:
  - target: AGENT-4c94fe14
    type: employed-by
  - target: AGENT-85be6ace
    type: employed-by
  - target: AGENT-e333508b
    type: employed-by
  - target: AGENT-bbad3d30
    type: employed-by
  - target: DOC-bad8e26f
    type: synchronised-with

---

Every implementation task follows a strict documentation-first workflow: **Document → Approve → Implement → Verify**. No code is written before documentation is approved. Documentation is the source of truth — code that diverges from docs is wrong and must be fixed.

## Collaborative Design Workflow

For any non-trivial feature, follow this preferred workflow before writing the formal plan:

1. **Discuss** — User describes the product need. Agent explores the codebase and asks clarifying questions. Both discuss architecture options, data model choices, UX ideas, and technical trade-offs conversationally until alignment is reached. Do NOT write a plan yet.
2. **Agree** — User and agent reach agreement on the approach, data model, UX, and key technical decisions through conversation. Agent captures these decisions explicitly so they can be incorporated into the plan.
3. **Plan** — Agent writes the formal implementation plan incorporating all agreed decisions. The plan follows the Systems Architecture Checklist (below) and the Architectural Compliance section.
4. **Approve** — User reviews and approves the plan (or requests changes). No implementation proceeds until the user explicitly approves.
5. **Implement** — Agent executes the approved plan phase by phase, with verification gates between phases.

**Why this workflow exists:** Writing a plan before discussing trade-offs produces plans that need to be thrown away. Discussing first produces plans that reflect real decisions.

## Documentation-First Principle

**The workflow for every implementation task:**

1. **Document** — Write or update documentation describing the planned implementation
2. **Approve** — Get explicit user approval before writing any code
3. **Implement** — Write code that matches the approved documentation exactly
4. **Verify** — Audit implementation against documentation and fix drift

**No exceptions.** This prevents documentation from becoming stale, ensures cross-session consistency, and makes the codebase self-explanatory.

## Pre-Implementation Documentation Checklist

**MANDATORY before ANY code changes.** Read these documents to understand context and constraints:

### Always Read

- Project governance directory `documentation/reference/` — Existing feature designs related to the task
- Project governance directory `process/decisions/` — Architecture decision artifacts
- Project governance directory `delivery/tasks/` — Task artifacts with context, constraints, and priorities
- Project roadmap — Verify the work is prioritised and not scope creep

### Read When Modifying Backend

- Existing backend module structure, command handlers, domain types

### Read When User-Facing Changes

- Product vision document — Core principles and product vision
- Governance documentation — Rules and decision-making process

### Use Search Tools First

Before reading entire files:

- `search_research` — "How does [feature area] work?" for architectural understanding
- `search_semantic` — Find relevant docs and code for specific concepts
- `search_regex` — Verify command names, function names, or specific symbols exist

**Why search tools first:** Avoids pulling entire files into context. Narrows down exactly what to read.

## Plan Structure Requirements

Every implementation plan must include these sections in order:

### 1. Architectural Compliance

> **Reminder:** When this plan reaches implementation phases, Phase 1 MUST be documentation updates. See section 5 below.

**Verify adherence to all foundational principles.** Show HOW each principle is satisfied with patterns specific to the plan — not just a list of decision IDs.

**Mandatory checks (verify every one that applies):**

| Principle | Verify |
| ----------- | -------- |
| Architecture decisions | Each relevant architecture decision is satisfied — show HOW with specific patterns |
| Error propagation | All functions return typed results, no unchecked panics or exceptions |
| Layer separation | Domain logic in backend, frontend is view layer only |
| Type safety | Types are consistent across all layers |
| End-to-end completeness | Every feature includes all required layers end-to-end |
| Coding standards | Function size limits, zero linter warnings, adequate test coverage |

**Example (good):**

```markdown
## Architectural Compliance

**Thick backend:** Session management logic lives entirely in the backend domain module.
Frontend only displays session list and current conversation.

**IPC boundary:** New commands `create_session` and `list_sessions` exposed via the IPC bridge.
Frontend calls via the invoke mechanism.

**Error propagation:** All session functions return typed Result types.
Command handlers map errors for serialization.
```

**Anti-pattern (bad):**

```markdown
## Architectural Compliance

Complies with all architecture decisions.
```

### 1b. Systems Architecture Checklist

Every plan MUST explicitly address each dimension below. For each, state either the specific approach OR "N/A — [reason]". Leaving a dimension blank is a plan rejection.

| Dimension | What to Address |
| ----------- | ---------------- |
| **Data Persistence** | What new data is created? Where is it stored? Schema design. Migration strategy. |
| **IPC Contract** | New/modified commands. Request/response types. Serialization. |
| **State Management** | Frontend state: where stored? How loaded/saved? What happens on window refresh? |
| **Configuration** | What config files are read/written? What config values are new? Where do defaults come from? |
| **Error Handling** | What can go wrong? How does each error surface to the user? Recovery paths? |
| **Testing Strategy** | Unit test approach. Integration test approach. E2E coverage? |
| **User Preferences** | Are there user choices that need persisting across sessions? Default values? Override mechanisms? |
| **Documentation** | Which docs need updating? Docs MUST be written before code (documentation-first). |

### 2. UX-First Design

**For user-facing changes:** Design the ideal user experience BEFORE the backend architecture.

**Required subsections:**

1. **User Journeys** — What the user sees and does in every scenario
2. **UI Design** — Components, layouts, and interactions
3. **Component State Table** — Every component, every state it can be in:

| Component | State | User Sees |
| ----------- | ------- | ----------- |
| SessionList | Loading | Spinner with "Loading sessions..." |
| SessionList | Empty | "No sessions yet" with create button |
| SessionList | Loaded | List of session cards with timestamps |
| SessionList | Error | Error message with retry button |

1. **Backend Requirements** — Derived from the above. What commands, types, and domain logic are needed to enable the UX?

### 3. Governing Documentation

**List the documentation that governs each implementation phase.**

### 4. Verification Gates

**Define what "done" means for each phase.** Include both quality checks and documentation compliance audits.

**Example:**

```markdown
## Verification Gate: Phase 1 Complete

**Quality Checks:**
- Format check passes
- Linter passes with zero warnings
- Tests pass with adequate coverage
- Type check passes

**Documentation Compliance:**
- IPC command signatures match architecture decision artifacts
- Component states match the plan's component state table
- Error types match documented error propagation strategy
```

### 5. Documentation Update (ALWAYS Phase 1 — NON-NEGOTIABLE)

**Every plan's FIRST implementation phase updates the documentation to define the target state BEFORE any code is written.**

**Required phase ordering:**

```text
Phase 1: Documentation update ← Define target state first
Phase 2: Backend changes       ← Governed by Phase 1 docs
Phase 3: Frontend changes      ← Governed by Phase 1 docs
Phase 4: Documentation verification ← Confirm docs still match
```

## Documentation Drift Prevention

**Drift = code that no longer matches the documentation.**

### Mandatory Rules During Implementation

1. **Re-read governing docs at the start of EVERY phase** — even if you "remember" from a prior session.
2. **Documentation is ALWAYS right** — if code diverges from docs, the code is wrong.
3. **No silent improvements** — if you discover a better approach during coding, STOP, update the doc FIRST, then resume.
4. **Between sessions: re-read docs before continuing.**
5. **Documentation compliance is a gate** — audit after every phase.

## Verification Gate Protocol

**After each implementation phase:**

1. **Verify documentation currency**
2. **Run quality checks** — format, lint, test, type-check
3. **Documentation compliance audit**
4. **Fix cycle:** If any check fails, fix it and re-run
5. **Gate pass:** Only when all checks pass AND documentation compliance is verified

**NEVER proceed to the next phase with failing checks or undocumented drift.**

## Plan Structure Template

Use this template for every implementation plan. Sections must appear in this order:

```markdown
## Architectural Compliance
[Verify each principle with specific patterns for this plan — show HOW, not just a list]

## Systems Architecture Checklist
[Address each dimension: Data Persistence, IPC Contract, State Management, Configuration,
Error Handling, Testing Strategy, User Preferences, Documentation.
State "N/A — [reason]" for inapplicable ones.]

## Target UX
[Wireframes/mockups/descriptions of what the user sees]

## User Journeys
[Every scenario: first-time, power user, error, edge cases]

## Component States
[Table: component x state -> what the user sees]

## User-Facing Language
[Internal key -> display label mapping]

## Phase N: [Name]
[Implementation details — backend derived from the above]

## Verification
[Measured by user-visible outcomes]
```

## Related Skills

- See the **search** skill for pre-implementation codebase research
- See the **architecture** skill for architectural compliance during planning
