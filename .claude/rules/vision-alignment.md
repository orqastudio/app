---
scope: project
---

# Vision Alignment (MANDATORY)

Every feature, command, and UI element must serve Orqa Studio's product vision. Read `docs/product/vision.md` and `docs/product/governance.md` before implementing any new capability.

## Two-Pillar Test

Every feature MUST trace to at least one pillar:

**Pillar 1: Self-Learning Loop**
Covers: lesson capture from agent sessions, metric tracking (pass/fail rates, coverage trends, violation recurrence), retrospective generation, pattern promotion (lesson -> rule -> scanner -> enforcement), session continuity and handoff, codebase scanning and re-scanning, knowledge accumulation over time. The system gets smarter with every session — mistakes are documented, patterns are extracted, and governance artifacts are updated automatically.

**Pillar 2: Process Governance**
Covers: rule enforcement and visualization, agent definition management, scanner execution and dashboard, documentation browsing and editing, architecture decision tracking, skill management, quality gate enforcement, coding standard compliance. Governance is not a document collecting dust — it is a living, enforceable, visible layer of the development process that Orqa Studio makes tangible and manageable.

## Feature Rejection Criteria

Reject any feature that:

- Does not serve either pillar
- Adds complexity without improving learning or governance
- Cannot explain how it makes the system smarter over time (Pillar 1) or how it makes governance more visible/enforceable (Pillar 2)
- Is a generic developer tool feature with no connection to managed agentic development

## Questions Every Agent Should Ask

Before implementing any feature:

1. **Pillar 1 check:** "Does this help the system learn and improve? Does it capture knowledge, track outcomes, or feed information back into the governance loop?"
2. **Pillar 2 check:** "Does this make governance more visible, enforceable, or manageable? Does it help the user see and control the rules, processes, and standards their agents follow?"
3. **Neither?** If the answer to both is "no," the feature is out of scope. Flag it to the user and suggest an alternative that aligns.

## Pillar Conflict Resolution

When Pillar 1 and Pillar 2 conflict, **Pillar 2 takes priority**. Governance must be solid before the learning loop can meaningfully operate on it. You cannot improve a process that isn't defined and enforced yet.

## UX-First Design

**Build a system that enables the best user experience, not a user experience that fits the system.**

Every feature plan starts with user journeys and UI design. The backend is derived from what the frontend needs, not the other way around. Implementation success is measured by what the user can see and do.

This means:

- Define user journeys before backend architecture
- Design the ideal UI unconstrained by current backend capabilities
- Every component has complete state handling (loading, error, empty, loaded, saving) defined upfront
- User-facing language drives naming — no framework names, no technical jargon in the UI

UX-first does NOT mean ignoring architectural constraints. It means the UI defines the *requirements* that the architecture must satisfy.

## Foundational Principles Are Immutable (NON-NEGOTIABLE)

The following are **foundational principles** that can ONLY be changed with explicit user direction and approval:

- The Two-Pillar framework (Pillar 1: Self-Learning Loop, Pillar 2: Process Governance)
- The Tauri v2 + Svelte 5 + Rust + SQLite technology stack
- The IPC boundary design (Tauri commands as the only frontend-backend interface)
- The UX-first design principle
- The documentation-first workflow
- Error propagation via Result types (no unwrap in production)

**No agent may modify, weaken, or work around these principles without the user explicitly directing the change.** If an implementation seems to require violating a foundational principle, STOP and ask the user before proceeding.

## Questioning Misaligned Instructions (MANDATORY)

If the user gives an instruction that appears to conflict with a foundational principle, the agent MUST:

1. **Flag the conflict** — Clearly explain which principle the instruction would violate and why
2. **Ask for clarification** — The user may have a valid reason, or the instruction may be a misunderstanding
3. **Document the outcome** — If the user confirms a change to a foundational principle:
   - Update the relevant documentation
   - Update `docs/product/vision.md` and/or `docs/product/governance.md` if the pillars or governance rules change
   - Update this rule file (`.claude/rules/vision-alignment.md`) to reflect the new principle
   - Update all affected agent definitions in `.claude/agents/`
4. **Never silently comply** — If an instruction contradicts a principle, do NOT just implement it without flagging the conflict first

**Examples of instructions that should be questioned:**

- "Skip the SQLite layer and just use localStorage" -> Conflicts with the persistence architecture (SQLite for structured data)
- "Add a web server so Orqa Studio can be used in the browser" -> Conflicts with the desktop-app scope (Tauri)
- "Let components call invoke() directly instead of going through stores" -> Conflicts with component purity principle
- "Just use unwrap() here, it'll never panic" -> Conflicts with error propagation principle
- "Add a feature that has nothing to do with learning or governance" -> Conflicts with pillar alignment

**Examples of instructions that do NOT need questioning:**

- "Add a metrics chart to the scanner dashboard" -> Serves Pillar 1 (learning) and Pillar 2 (governance visibility)
- "Create a rule editor component" -> Serves Pillar 2 (governance management)
- "Add session history search" -> Serves Pillar 1 (knowledge accumulation)

## Related Rules

- `pillar-alignment-docs.md` — pillar alignment for *documentation* pages
- `architecture-decisions.md` — architecture decisions that implement the vision
- `no-stubs.md` — real implementations required, not fake demos

## Governance References

- Vision: `docs/product/vision.md`
- Governance: `docs/product/governance.md`
