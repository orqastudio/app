---
id: RULE-303c1cc8
title: Plan Mode Compliance
description: Every implementation plan must have architectural compliance verification and UX-first design.
status: active
created: 2026-03-07
updated: 2026-03-12
enforcement: "agent system prompt — orchestrator verifies Architectural Compliance and Systems Architecture Checklist sections exist before delegating implementation; plans missing these sections are rejected"
relationships:
  - target: AD-c8535011
    type: enforces
---
Every implementation plan — whether captured in an epic, a task, or proposed in conversation — MUST satisfy two mandatory requirements before implementation begins.

## 1. Architectural Compliance Verification

Every plan MUST include an explicit "Architectural Compliance" section that verifies adherence to all foundational principles. This section comes BEFORE the phase-by-phase implementation details.

**Required checks (verify every one that applies):**

| Principle | Verify |
|-----------|--------|
| Error propagation | All functions return result/error types. No silent failures in production. |
| Service boundary | Backend services are the only interface between frontend and backend. No side channels. |
| Component purity | Display components receive data via props. Pages/containers fetch data. No direct service calls in reusable components. |
| Type safety | Strict typing throughout. No implicit any types. Service types match across layers. |
| Immutability | Domain types are immutable by default. Reactive state uses the project's established store pattern. |
| UX-first | Plan starts with user journeys and UI design. Backend derived from frontend needs. |
| End-to-end completeness | Every new feature includes all required layers in the same task. |
| Coding standards | Function size limits, zero lint errors, required coverage. |

**The section must show HOW each principle is satisfied, not just list them.** Include patterns, anti-patterns, and examples specific to the plan.

## 1b. Systems Architecture Checklist (MANDATORY)

Every plan MUST include a "Systems Architecture Checklist" section that explicitly addresses each dimension below. Leaving a dimension unaddressed is a plan rejection. State "N/A — [reason]" for dimensions that don't apply.

| Dimension | What to Address |
|-----------|----------------|
| **Data Persistence** | What new data is created? Database tables? File system artifacts? Schema design. Migration strategy. |
| **Service Contract** | New/modified backend endpoints or commands. Input/output types. Which modules expose them. |
| **State Management** | Frontend state: where stored? How loaded/saved? What happens on app restart? |
| **Configuration** | What config files are read/written? What config values are new? Where do defaults come from? |
| **Health & Status** | How does the system know this feature is working? Status reporting? Degraded mode behavior? |
| **Error Handling** | What can go wrong? How does each error surface to the user? Recovery paths? |
| **Testing Strategy** | Unit test approach. Integration test approach. E2E coverage? |
| **User Preferences** | Are there user choices that need persisting across sessions? Default values? Override mechanisms? |
| **Documentation** | Which docs need updating? Docs MUST be written before code (documentation-first). |

The Systems Architecture Checklist section MUST appear after "Architectural Compliance" and before "Target UX" in every plan.

## 2. UX-First Design

Every plan that includes user-facing changes MUST be structured UX-first:

1. **User journeys first** — What does the user see and do? Cover: first-time, power user, error states, edge cases, no-data states.
2. **UI design second** — What components, layouts, and interactions best serve those journeys?
3. **Component state table** — Every component lists ALL states: loading, error, empty, loaded, saving, unsaved changes, etc.
4. **User-facing language** — Internal keys mapped to display labels. No framework names, no technical jargon in the UI.
5. **Backend last** — Service endpoints, data models, and backend modules derived from the UI requirements.

**Measurement:** Every phase's success is measured by what the user can see and do, not by what the backend implements.

See the `planning` skill for the plan structure template and full methodology.

## Artifact Integration

Plans exist within the artifact framework. When an implementation plan is created:

1. **If the plan serves an epic:** The implementation design lives in the epic body, and the epic's `research-refs` field references any supporting research documents
2. **If the plan serves an idea being shaped:** The plan is created during the `exploring → shaped` transition
3. **Plans produce epics:** A plan that is approved should result in an `EPIC-NNN` being created (or updated) with `docs-required` and `docs-produced` fields populated from the plan's documentation sections
4. **The plan's documentation section feeds the epic's gates:** Items in the plan's "Documentation" dimension of the Systems Architecture Checklist become the epic's `docs-required` and `docs-produced` lists

See [RULE-7b770593](RULE-7b770593) (artifact-lifecycle) for the full artifact lifecycle and `.orqa/documentation/guide/artifact-workflow.md` for day-to-day workflow.

## Roadmap Reconciliation (MANDATORY)

Before a plan is approved, the orchestrator MUST reconcile the plan's task list with the epic's roadmap entry:

1. Read the roadmap entry for the epic
2. Verify every roadmap item appears as a task in the plan
3. If any roadmap item is missing from the plan, either add it as a task or get explicit user approval to descope it
4. No item may be silently moved to "Out of Scope" or deferred to another epic without user approval

See [RULE-e120bb70](RULE-e120bb70) (no-deferred-deliverables) for the full enforcement rule.

## When This Rule Applies

- Creating a new implementation plan (in an epic or task artifact)
- Proposing a multi-phase feature
- Any work that touches 3+ files or crosses a service boundary
- Any work that adds or modifies user-facing functionality

## When This Rule Does NOT Apply

- Single-file bug fixes
- Documentation-only changes
- Refactoring that doesn't change user-facing behavior
- Research/exploration tasks

## Verification Gate Enforcement (NON-NEGOTIABLE)

After every plan phase is implemented, an INDEPENDENT verification must occur before the phase is considered complete. The implementing agent CANNOT self-certify completion.

### Gate Protocol

1. **Implementing agent** completes the phase and commits the work
2. **Code-reviewer agent** runs an independent audit with PASS/FAIL verdict:
   - Quality checks: linter, type-checker, test suite
   - Doc compliance: every UI label matches docs, every service type matches docs
   - Behavioral verification: features work as documented, not just type-check
3. **If FAIL**: implementing agent fixes issues, code-reviewer re-audits
4. **If PASS**: phase is marked complete, next phase can begin

### What the Reviewer Checks

- No stub/mock/placeholder data in production source code
- No TODO/FIXME comments in committed code
- Code matches governing documentation exactly (labels, states, layouts)
- All layers exist end-to-end
- Tests exist and pass for new functionality

### Evidence Requirements (NON-NEGOTIABLE)

Claims without evidence are not verification. The reviewer MUST collect and include:

**For backend endpoints or commands:**

- Actual invocation output showing the response data (not "command returns data")
- If the command returns computed data, show that the data is REAL (not empty/default)

**For frontend components:**

- Description of what the user would SEE if they opened the app right now
- If a component displays backend data, verify the endpoint is called and returns real values

**For end-to-end wiring:**

- Trace the full chain from UI component to backend and back
- Show that each hop exists with actual evidence (search results, test output)

**For "it works":**

- "Works" means: the user can perform the documented action and see the documented result
- "Works" does NOT mean: the code compiles, the types check, or the tests pass
- If you cannot demonstrate it working, it is NOT done

### The implementing agent MUST NOT

- Mark a phase complete without reviewer confirmation
- Classify its own findings as "minor" or "can be done later"
- Skip the reviewer step for "simple" changes

## Loop Closure Phase (MANDATORY for enforcement/audit epics)

Any epic that produces enforcement tooling, audit tooling, or governance automation MUST include a loop closure phase as its final phase. This phase:

1. **Runs all tooling** produced by the epic against the full codebase
2. **Triages findings** — immediate fixes vs items to reconcile
3. **Reconciles findings** — findings are added to the epic's task list for resolution within the same epic, not deferred to future work
4. **Updates this rule** if the epic reveals planning methodology gaps

Without loop closure, enforcement tooling is created but never run against the codebase that prompted its creation. The findings that motivated the epic go unaddressed. Deferring findings to future epics violates [RULE-e120bb70](RULE-e120bb70) — if the tooling is in scope, its findings are in scope.

## Scope Verification (MANDATORY)

Before any epic moves to `done`, the orchestrator MUST present the Out of Scope section to the user for explicit approval. Out of Scope sections are user decisions — the orchestrator proposes exclusions, the user approves or rejects them.

**FORBIDDEN:** Writing an Out of Scope section and committing it without user verification. Every scope reduction must be an explicit user decision, not an orchestrator assumption.

## Enforcement

Plans that omit architectural compliance or UX-first design are rejected. The orchestrator MUST verify both sections exist and are substantive (not just boilerplate) before delegating implementation to subagents.

After every phase implementation, the orchestrator MUST invoke the code-reviewer agent for independent verification before proceeding to the next phase.

## Related Rules

- [RULE-7b770593](RULE-7b770593) (artifact-lifecycle) — artifact creation, status transitions, promotion gates
- [RULE-1e8a1914](RULE-1e8a1914) (vision-alignment) — the pillar framework that plans must serve
- [RULE-65973a88](RULE-65973a88) (architecture-decisions) — the architecture decisions plans must comply with
- [RULE-b49142be](RULE-b49142be) (coding-standards) — the standards implementations must meet
- [RULE-1acb1602](RULE-1acb1602) (end-to-end-completeness) — the full-stack requirement per feature
