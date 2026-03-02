# Plan Mode Compliance (NON-NEGOTIABLE)

Every implementation plan — whether created in plan mode, written to TODO.md, or proposed in conversation — MUST satisfy two mandatory requirements before implementation begins.

## 1. Architectural Compliance Verification

Every plan MUST include an explicit "Architectural Compliance" section that verifies adherence to all foundational principles. This section comes BEFORE the phase-by-phase implementation details.

**Required checks (verify every one that applies):**

| Principle | Verify |
|-----------|--------|
| Error propagation | All Rust functions return `Result`. No `unwrap()` in production. `thiserror` for typed errors. |
| IPC boundary | Tauri commands are the only frontend-backend interface. No side channels. |
| Component purity | Display components receive props only. Pages/containers fetch data. No `invoke()` in reusable components. |
| Type safety | Strict TypeScript. No `any`. Rust types derive Serialize/Deserialize for IPC. |
| Immutability | Rust domain types are immutable by default. Svelte stores use runes. |
| UX-first | Plan starts with user journeys and UI design. Backend derived from frontend needs. |
| End-to-end completeness | Every new feature includes all 4 layers (Rust command + IPC type + Svelte component + store binding) in the same task. |
| Coding standards | Function size limits, zero clippy/lint errors, 80%+ coverage. |

**The section must show HOW each principle is satisfied, not just list them.** Include patterns, anti-patterns, and examples specific to the plan.

## 1b. Systems Architecture Checklist (MANDATORY)

Every plan MUST include a "Systems Architecture Checklist" section that explicitly addresses each dimension below. Leaving a dimension unaddressed is a plan rejection. State "N/A — [reason]" for dimensions that don't apply.

| Dimension | What to Address |
|-----------|----------------|
| **Data Persistence** | What new data is created? SQLite tables? File system artifacts? Schema design. Migration strategy. |
| **IPC Contract** | New/modified Tauri commands. Input/output types. Which Rust modules expose them. |
| **State Management** | Frontend state: where stored (rune store, component, URL)? How loaded/saved? What happens on app restart? |
| **Configuration** | What config files are read/written? What config values are new? Where do defaults come from? |
| **Health & Status** | How does the system know this feature is working? Status reporting? Degraded mode behavior? |
| **Error Handling** | What can go wrong? How does each error surface to the user? Recovery paths? |
| **Testing Strategy** | Unit test approach (cargo test, Vitest). Integration test approach. E2E coverage (Playwright)? |
| **User Preferences** | Are there user choices that need persisting across sessions? Default values? Override mechanisms? |
| **Documentation** | Which docs need updating? Docs MUST be written before code (documentation-first). |

The Systems Architecture Checklist section MUST appear after "Architectural Compliance" and before "Target UX" in every plan.

## 2. UX-First Design

Every plan that includes user-facing changes MUST be structured UX-first:

1. **User journeys first** — What does the user see and do? Cover: first-time, power user, error states, edge cases, no-data states.
2. **UI design second** — What components, layouts, and interactions best serve those journeys?
3. **Component state table** — Every component lists ALL states: loading, error, empty, loaded, saving, unsaved changes, etc.
4. **User-facing language** — Internal keys mapped to display labels. No framework names, no technical jargon in the UI.
5. **Backend last** — Tauri commands, data models, and Rust modules derived from the UI requirements.

**Measurement:** Every phase's success is measured by what the user can see and do, not by what the backend implements.

## Plan Structure Template

```markdown
## Architectural Compliance
[Verify each principle with specific patterns for this plan]

## Systems Architecture Checklist
[Address each dimension: Data Persistence, IPC Contract, State Management, Configuration,
Health & Status, Error Handling, Testing Strategy, User Preferences, Documentation.
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

## When This Rule Applies

- Creating a new implementation plan (plan mode or TODO.md)
- Proposing a multi-phase feature
- Any work that touches 3+ files or crosses the Rust/TypeScript boundary
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
   - Quality checks: `cargo clippy`, `cargo test`, `npm run check`, `npm run lint`, `npm run test`
   - Doc compliance: every UI label matches docs, every IPC type matches docs
   - Behavioral verification: features work as documented, not just type-check
3. **If FAIL**: implementing agent fixes issues, code-reviewer re-audits
4. **If PASS**: phase is marked complete, next phase can begin

### What the Reviewer Checks

- No stub/mock/placeholder data in production source code
- No TODO/FIXME comments in committed code
- Code matches governing documentation exactly (labels, states, layouts)
- All layers exist end-to-end (Rust command -> IPC type -> Svelte component -> store)
- Tests exist and pass for new functionality

### Evidence Requirements (NON-NEGOTIABLE)

Claims without evidence are not verification. The reviewer MUST collect and include:

**For Tauri commands:**

- Actual invocation output showing the response data (not "command returns data")
- If the command returns computed data, show that the data is REAL (not empty/default)

**For frontend components:**

- Description of what the user would SEE if they opened the app right now
- If a component displays backend data, verify the command is called and returns real values

**For IPC wiring (end-to-end):**

- Trace the full chain: component -> invoke -> Rust command -> database/filesystem
- Show that each hop exists with actual evidence (search results, test output)

**For "it works":**

- "Works" means: the user can perform the documented action and see the documented result
- "Works" does NOT mean: the code compiles, the types check, or the tests pass
- If you cannot demonstrate it working, it is NOT done

### The implementing agent MUST NOT

- Mark a phase complete without reviewer confirmation
- Classify its own findings as "minor" or "can be done later"
- Skip the reviewer step for "simple" changes

## Enforcement

Plans that omit architectural compliance or UX-first design are rejected. The orchestrator MUST verify both sections exist and are substantive (not just boilerplate) before delegating implementation to subagents.

After every phase implementation, the orchestrator MUST invoke the code-reviewer agent for independent verification before proceeding to the next phase.

## Related Rules

- `vision-alignment.md` — the Two-Pillar framework that plans must serve
- `architecture-decisions.md` — the architecture decisions plans must comply with
- `coding-standards.md` — the standards implementations must meet
- `end-to-end-completeness.md` — the full-stack requirement per feature
