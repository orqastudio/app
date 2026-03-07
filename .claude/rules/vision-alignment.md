---
scope: project
---

# Vision Alignment (MANDATORY)

Every feature, command, and UI element must serve OrqaStudio's product vision. Read `docs/product/vision.md` and `docs/product/governance.md` before implementing any new capability.

## Two-Pillar Test

Every feature MUST trace to at least one pillar:

**Pillar 1: Clarity Through Structure**
Covers: making governance artifacts visible and manageable, producing structured knowledge (plans, decisions, rules), surfacing what would otherwise be hidden in files or terminal output, enforcing documentation-first workflows, architecture decision tracking, rule enforcement and visualisation, agent definition management, scanner execution and dashboard, quality gate enforcement. Governance is not a document collecting dust — it is a living, enforceable, visible layer that OrqaStudio makes tangible and manageable.

**Pillar 2: Learning Through Reflection**
Covers: lesson capture, metric tracking (pass/fail rates, coverage trends, violation recurrence), retrospective generation, pattern promotion (lesson -> rule -> scanner -> enforcement), session continuity and handoff, codebase scanning and re-scanning, knowledge accumulation over time. The system and its users get smarter with every cycle — mistakes are documented, patterns are extracted, and governance artifacts are updated automatically.

## Feature Rejection Criteria

Reject any feature that:

- Does not serve either pillar
- Adds complexity without improving clarity or learning
- Cannot explain how it makes work more visible and structured (Pillar 1) or how it makes the process smarter over time (Pillar 2)
- Is a generic tool feature with no connection to structured thinking or reflective improvement

## Questions Every Agent Should Ask

Before implementing any feature:

1. **Pillar 1 check:** "Does this make thinking, standards, or decisions more visible and structured? Does it help the user see and manage their governance framework?"
2. **Pillar 2 check:** "Does this help the system or its users improve over time? Does it capture knowledge, track outcomes, or feed information back into the governance loop?"
3. **Neither?** If the answer to both is "no," the feature is out of scope. Flag it to the user and suggest an alternative that aligns.

## Pillar Conflict Resolution

When Pillar 1 and Pillar 2 conflict, **Pillar 1 takes priority**. You cannot improve a process that isn't visible and structured. Governance must be solid before the learning loop can meaningfully operate on it.

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

- The Two-Pillar framework (Pillar 1: Clarity Through Structure, Pillar 2: Learning Through Reflection)
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
- "Add a web server so OrqaStudio can be used in the browser" -> Conflicts with the desktop-app scope (Tauri)
- "Let components call invoke() directly instead of going through stores" -> Conflicts with component purity principle
- "Just use unwrap() here, it'll never panic" -> Conflicts with error propagation principle
- "Add a feature that has nothing to do with clarity or learning" -> Conflicts with pillar alignment

**Examples of instructions that do NOT need questioning:**

- "Add a metrics chart to the scanner dashboard" -> Serves both pillars (visibility + learning trends)
- "Create a rule editor component" -> Serves Pillar 1 (governance made visible and editable)
- "Add session history search" -> Serves Pillar 2 (knowledge accumulation across sessions)

## Related Rules

- `artifact-lifecycle.md` — artifact creation, status transitions, promotion gates, documentation gates
- `pillar-alignment-docs.md` — pillar alignment for *documentation* pages
- `architecture-decisions.md` — architecture decisions that implement the vision
- `no-stubs.md` — real implementations required, not fake demos

## Governance References

- Vision: `docs/product/vision.md`
- Governance: `docs/product/governance.md`
- Artifact Framework: `docs/product/artifact-framework.md`
- Artifact Workflow: `docs/process/artifact-workflow.md`
